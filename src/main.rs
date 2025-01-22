use std::ffi::OsStr;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use client::discord::{Discord, DiscordActivityPreload};
use client::git::get_repository_and_remote;
use config::LspConfig;
use tokio::sync::{Mutex, MutexGuard};
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use util::Placeholders;

mod client;
mod config;
mod languages;
mod util;

macro_rules! replace_placeholders {
    ($placeholders:expr, $($field:expr),*) => {
        (
            $( $field.as_ref().map(|v| $placeholders.replace(v)), )*
        )
    };
}

#[derive(Debug)]
struct Document {
    path: PathBuf,
}

#[derive(Debug)]
struct Backend {
    client: Client,
    discord: Arc<Mutex<Discord>>,
    workspace_file_name: Arc<Mutex<String>>,
    git_remote_url: Arc<Mutex<Option<String>>>,
    config: Arc<Mutex<LspConfig>>,
}

impl Document {
    fn new(url: Url) -> Self {
        let url_path = url.path();
        let path = Path::new(url_path);

        Self {
            path: path.to_owned(),
        }
    }

    fn get_filename(&self) -> String {
        let filename = self.path.file_name().unwrap().to_str().unwrap();
        let filename = urlencoding::decode(filename).unwrap();

        filename.to_string()
    }

    fn get_extension(&self) -> &str {
        self.path
            .extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap()
    }
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            discord: Arc::new(Mutex::new(Discord::new())),
            workspace_file_name: Arc::new(Mutex::new(String::new())),
            git_remote_url: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(LspConfig::new())),
        }
    }

    async fn on_change(&self, doc: Document) {
        let preload = self.get_activity_preload(Some(&doc)).await;

        self.get_discord().await.change_activity(preload).await;
    }

    async fn get_workspace_file_name(&self) -> MutexGuard<'_, String> {
        return self.workspace_file_name.lock().await;
    }

    async fn get_git_remote_url(&self) -> Option<String> {
        let guard = self.git_remote_url.lock().await;

        guard.clone()
    }

    async fn get_config(&self) -> MutexGuard<LspConfig> {
        return self.config.lock().await;
    }

    async fn get_discord(&self) -> MutexGuard<Discord> {
        return self.discord.lock().await;
    }

    async fn get_activity_preload(&self, doc: Option<&Document>) -> DiscordActivityPreload {
        let config = self.get_config().await;
        let workspace = self.get_workspace_file_name().await;
        let placeholders = Placeholders::new(doc, &config, workspace.deref());

        let (state, details, large_image, large_text, small_image, small_text) = replace_placeholders!(
            &placeholders,
            &config.state,
            &config.details,
            &config.large_image,
            &config.large_text,
            &config.small_image,
            &config.small_text
        );

        DiscordActivityPreload {
            state,
            details,
            large_image,
            large_text,
            small_image,
            small_text,
            git_remote_url: if config.view_repositoy_button {
                let git_remote_url_guard = self.get_git_remote_url().await;
                git_remote_url_guard.clone()
            } else {
                None
            },
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Set workspace name
        let root_uri = params.root_uri.expect("Failed to get root uri");
        let workspace_path = Path::new(root_uri.path());
        self.workspace_file_name.lock().await.push_str(
            workspace_path
                .file_name()
                .expect("Failed to get workspace file name")
                .to_str()
                .expect("Failed to convert workspace file name &OsStr to &str"),
        );

        let mut git_remote_url = self.git_remote_url.lock().await;
        *git_remote_url = get_repository_and_remote(workspace_path.to_str().unwrap());

        let config = self.config.lock().await;

        let mut discord = self.get_discord().await;
        discord.create_client(config.application_id.to_string());

        discord.connect().await;

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: env!("CARGO_PKG_NAME").into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(
                MessageType::INFO,
                "Discord Presence LSP server intiailized!",
            )
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        self.get_discord().await.kill().await;

        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.on_change(Document::new(params.text_document.uri))
            .await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.on_change(Document::new(params.text_document.uri))
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(Backend::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
