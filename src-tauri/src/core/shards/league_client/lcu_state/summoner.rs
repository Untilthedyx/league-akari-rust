use crate::shared::types::league_client::summoner::{SummonerInfo, SummonerProfile};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SummonerState {
    pub me: Option<SummonerInfo>,
    pub profile: Option<SummonerProfile>,
    pub new_id_system_enabled: bool,
}

#[derive(Debug, Default)]
pub struct SummonerStateLock {
    pub state: RwLock<SummonerState>,
}

impl SummonerStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_me(&self, me: Option<SummonerInfo>) {
        let mut state = self.state.write().await;
        state.me = me;
    }

    pub async fn set_profile(&self, profile: Option<SummonerProfile>) {
        let mut state = self.state.write().await;
        state.profile = profile;
    }

    pub async fn set_new_id_system_enabled(&self, enabled: bool) {
        let mut state = self.state.write().await;
        state.new_id_system_enabled = enabled;
    }
}
