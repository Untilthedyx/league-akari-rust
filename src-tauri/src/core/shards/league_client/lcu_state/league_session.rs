use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct LeagueSessionState {
    pub token: Option<String>,
}

#[derive(Debug, Default)]
pub struct LeagueSessionStateLock {
    pub state: RwLock<LeagueSessionState>,
}

impl LeagueSessionStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_token(&self, token: Option<String>) {
        self.state.write().await.token = token;
    }
}
