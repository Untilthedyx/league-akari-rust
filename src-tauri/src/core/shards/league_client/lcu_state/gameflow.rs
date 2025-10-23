use crate::shared::types::league_client::gameflow::{GameflowPhase, GameflowSession};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct GameflowState {
    pub phase: Option<GameflowPhase>,
    pub session: Option<GameflowSession>,
}

#[derive(Debug, Default)]
pub struct GameflowStateLock {
    pub state: RwLock<GameflowState>,
}

impl GameflowStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_phase(&mut self, phase: GameflowPhase) {
        self.state.write().await.phase = Some(phase);
    }

    pub async fn set_session(&mut self, session: GameflowSession) {
        self.state.write().await.session = Some(session);
    }
}
