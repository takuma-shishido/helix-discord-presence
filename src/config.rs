#[derive(Debug)]
pub struct LspConfig {
    pub application_id: String,
    pub base_icons_url: String,

    pub state: Option<String>,
    pub details: Option<String>,

    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,

    pub view_repositoy_button: bool,
}

impl LspConfig {
    pub fn new() -> Self {
        Self {
            application_id: String::from("1330779433946189906"),
            base_icons_url: String::from(
                "https://raw.githubusercontent.com/takuma-shishido/helix-discord-presence/main/assets/icons/",
            ),
            state: Some(String::from("Working on {filename}")),
            details: Some(String::from("In {workspace}")),
            large_image: Some(String::from("{base_icons_url}/{language}.png")),
            large_text: Some(String::from("{language:u}")),
            small_image: Some(String::from("{base_icons_url}/helix.png")),
            small_text: Some(String::from("Helix")),
            view_repositoy_button: true,
        }
    }
}
