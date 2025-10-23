pub mod champ_select;
pub mod chat;
pub mod entitlements;
pub mod game_data;
pub mod gameflow;
pub mod honor;
pub mod league_session;
pub mod lobby;
pub mod lobby_team_builder;
pub mod login;
pub mod matchmaking;
pub mod summoner;

use crate::core::shards::league_client::utils::task_runner::TaskRunner;
use crate::core::shards::league_client::lcu_state::gameflow::GameflowStateLock;
use crate::core::shards::league_client::lcu_state::chat::ChatStateLock;
use crate::core::shards::league_client::lcu_state::honor::HonorStateLock;
use crate::core::shards::league_client::lcu_state::champ_select::ChampSelectStateLock;
use crate::core::shards::league_client::lcu_state::login::LoginStateLock;
use crate::core::shards::league_client::lcu_state::lobby::LobbyStateLock;
use crate::core::shards::league_client::lcu_state::summoner::SummonerStateLock;
use crate::core::shards::league_client::lcu_state::matchmaking::MatchmakingStateLock;
use crate::core::shards::league_client::lcu_state::game_data::GameDataStateLock;
use crate::core::shards::league_client::lcu_state::entitlements::EntitlementsStateLock;
use crate::core::shards::league_client::lcu_state::league_session::LeagueSessionStateLock;
use crate::core::shards::league_client::lcu_state::lobby_team_builder::LobbyTeamBuilderStateLock;


use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct InitializationProgress {
    pub currend_id: Option<String>,
    pub finished: Vec<String>,
    pub all: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct InitializationState {
    pub progress: Option<InitializationProgress>,
}

impl InitializationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn set_progress(&mut self, progess: Option<InitializationProgress>) {
        self.progress = progess;
    }
}

#[derive(Debug, Default)]
pub struct LeagueClientData {
    pub _state_initializer: TaskRunner,
    pub initialization: InitializationState,
    pub gameflow: GameflowStateLock,
    pub chat: ChatStateLock,
    pub honor: HonorStateLock,
    pub champ_select: ChampSelectStateLock,
    pub login: LoginStateLock,
    pub lobby: LobbyStateLock,
    pub summoner: SummonerStateLock,
    pub matchmaking: MatchmakingStateLock,
    pub game_data: GameDataStateLock,
    pub entitlements: EntitlementsStateLock,
    pub league_session: LeagueSessionStateLock,
    pub lobby_team_builder: LobbyTeamBuilderStateLock,
}


