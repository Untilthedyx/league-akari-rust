use crate::shared::types::league_client::login::LoginQueueState;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LoginState {
    pub login_queue_state: Option<LoginQueueState>,
}

#[derive(Debug, Default)]
pub struct LoginStateLock {
    pub state: RwLock<LoginState>,
}

impl LoginStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_login_queue_state(&self, login_queue_state: Option<LoginQueueState>) {
        let mut state = self.state.write().await;
        state.login_queue_state = login_queue_state;
    }
}
