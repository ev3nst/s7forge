use once_cell::sync::Lazy;
use steamworks::Client;

use crate::core::client::SteamState;

pub static STEAM_MANAGER: Lazy<SteamManager> = Lazy::new(SteamManager::new);

pub struct SteamManager {
    steam_state: SteamState,
}

impl SteamManager {
    pub fn new() -> Self {
        Self {
            steam_state: SteamState::new(),
        }
    }

    pub async fn initialize_client(&self, app_id: u32) -> Result<steamworks::Client, String> {
        if !self.steam_state.has_client(app_id) {
            self.steam_state.drop_all_clients();
            let (steam_client, single_client) = Client::init_app(app_id)
                .map_err(|err| format!("Failed to initialize Steam client: {:?}", err))?;
            self.steam_state
                .set_clients(app_id, steam_client, single_client);
        }

        self.steam_state
            .get_client(app_id)
            .ok_or_else(|| "Failed to get Steam client".to_string())
    }

    pub fn run_callbacks(&self, app_id: u32) -> Result<(), String> {
        self.steam_state.run_callbacks(app_id)
    }
}

pub async fn initialize_client(app_id: u32) -> Result<steamworks::Client, String> {
    STEAM_MANAGER.initialize_client(app_id).await
}

pub fn run_callbacks(app_id: u32) -> Result<(), String> {
    STEAM_MANAGER.run_callbacks(app_id)
}
