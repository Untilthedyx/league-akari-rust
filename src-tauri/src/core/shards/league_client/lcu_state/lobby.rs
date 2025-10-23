use crate::shared::types::league_client::lobby::{Lobby, ReceivedInvitation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct LobbyState {
    pub lobby: Option<Lobby>,
    pub received_invitations: Vec<ReceivedInvitation>,
}

#[derive(Default, Debug)]
pub struct LobbyStateLock {
    pub state: RwLock<LobbyState>,
}

impl LobbyStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_lobby(&mut self, lobby: Option<Lobby>) {
        let mut state = self.state.write().await;
        state.lobby = lobby;
    }

    pub async fn set_received_invitations(&mut self, invitations: Vec<ReceivedInvitation>) {
        let mut state = self.state.write().await;
        state.received_invitations = invitations;
    }
}
