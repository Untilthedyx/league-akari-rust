use crate::shared::types::league_client::matchmaking::{GetSearch, ReadyCheck};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MatchmakingState {
    pub ready_check: Option<ReadyCheck>,
    pub search: Option<GetSearch>,
}

#[derive(Debug, Default)]
pub struct MatchmakingStateLock {
    pub state: RwLock<MatchmakingState>,
}

impl MatchmakingStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_ready_check(&self, ready_check: ReadyCheck) {
        let mut state = self.state.write().await;
        state.ready_check = Some(ready_check);
    }

    pub async fn set_search(&self, search: GetSearch) {
        let mut state = self.state.write().await;
        state.search = Some(search);
    }
}
