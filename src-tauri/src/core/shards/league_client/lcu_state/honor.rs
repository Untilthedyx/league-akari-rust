use crate::shared::types::league_client::honor::Ballot;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct HonorState {
    pub ballot: Option<Ballot>,
}

#[derive(Debug, Default)]
pub struct HonorStateLock {
    pub state: RwLock<HonorState>,
}

impl HonorStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_ballot(&mut self, b: Option<Ballot>) {
        self.state.write().await.ballot = b;
    }
}
