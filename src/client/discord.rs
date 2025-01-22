use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, MutexGuard};

use discord_rich_presence::{
    activity::{Activity, Assets, Button, Timestamps},
    DiscordIpc, DiscordIpcClient,
};

use crate::util;

#[derive(Debug)]
pub struct DiscordActivityPreload {
    pub state: Option<String>,
    pub details: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub git_remote_url: Option<String>,
}

#[derive(Debug)]
pub struct Discord {
    client: Option<Mutex<DiscordIpcClient>>,
    start_timestamp: Duration,
}

impl Discord {
    pub fn new() -> Self {
        let start_timestamp = SystemTime::now();
        let since_epoch = start_timestamp
            .duration_since(UNIX_EPOCH)
            .expect("Failed to get duration since UNIX_EPOCH");

        Self {
            client: None,
            start_timestamp: since_epoch,
        }
    }

    pub fn create_client(&mut self, application_id: String) {
        let discord_client = DiscordIpcClient::new(application_id.as_str())
            .expect("Failed to initialize Discord Ipc Client");

        self.client = Some(Mutex::new(discord_client));
    }

    pub async fn connect(&self) {
        let mut client = self.get_client().await;
        let result = client.connect();

        result.unwrap();
    }

    pub async fn kill(&self) {
        let mut client = self.get_client().await;
        let result = client.close();

        result.unwrap();
    }

    pub async fn get_client(&self) -> MutexGuard<'_, DiscordIpcClient> {
        self.client
            .as_ref()
            .expect("Discord client not initialized")
            .lock()
            .await
    }

    pub async fn change_activity(&self, preload: DiscordActivityPreload) {
        let mut client = self.get_client().await;
        let timestamp: i64 = self.start_timestamp.as_millis() as i64;

        let activity = Activity::new()
            .timestamps(Timestamps::new().start(timestamp))
            .buttons(
                preload
                    .git_remote_url
                    .as_ref()
                    .map(|url| vec![Button::new("View Repository", url)])
                    .unwrap_or_default(),
            );

        let activity =
            util::set_optional_field(activity, preload.state.as_deref(), Activity::state);
        let activity =
            util::set_optional_field(activity, preload.details.as_deref(), Activity::details);

        let assets = Assets::new();
        let assets =
            util::set_optional_field(assets, preload.large_image.as_deref(), Assets::large_image);
        let assets =
            util::set_optional_field(assets, preload.large_text.as_deref(), Assets::large_text);
        let assets =
            util::set_optional_field(assets, preload.small_image.as_deref(), Assets::small_image);
        let assets =
            util::set_optional_field(assets, preload.small_text.as_deref(), Assets::small_text);

        let activity = activity.assets(assets);

        client
            .set_activity(activity)
            .unwrap_or_else(|_| println!("Failed to set activity with activity"));
    }
}
