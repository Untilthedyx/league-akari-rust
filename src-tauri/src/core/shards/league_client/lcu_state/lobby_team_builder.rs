use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LobbyTeamBuilderState {
    pub subset_champion_list: Vec<i32>,
}

#[derive(Debug, Default)]
pub struct LobbyTeamBuilderStateLock {
    pub state: RwLock<LobbyTeamBuilderState>,
}

impl LobbyTeamBuilderStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_subset_champion_list(&self, subset_champion_list: Vec<i32>) {
        self.state.write().await.subset_champion_list = subset_champion_list;
    }
}
