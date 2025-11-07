pub mod lcu_state;
pub mod utils;

use crate::shared::http_api::league_client::LcuApi;
use crate::shared::http_api::websocket::WebsocketClient;
use crate::utils::error::websocket_error::WebsocketError;
use lcu_state::LeagueClientState;

pub struct LeagueClient {
    pub state: LeagueClientState,
    pub http_api: LcuApi,
    pub websocket: WebsocketClient,
}

impl LeagueClient {
    pub fn new(port: u32, token: &str) -> Self {
        Self {
            state: LeagueClientState::new(),
            http_api: LcuApi::new(port, token.to_string()),
            websocket: WebsocketClient::new(port, token.to_string()),
        }
    }

    pub async fn init<F>(&mut self, callback: F) -> Result<(), WebsocketError>
    where
        F: Fn(serde_json::Value) + Send + Sync + 'static,
    {
        self.websocket.on_message(callback);
        self.websocket.connect().await?;
        Ok(())
    }
}
