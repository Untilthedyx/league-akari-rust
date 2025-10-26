// pub mod api_helper; // 似乎没有必要使用 trait
pub mod challenges;
pub mod champ_select;
pub mod champion_mastery;
pub mod chat;
pub mod end_of_game;
pub mod entitlements;
pub mod event_hub;
pub mod game_data;
pub mod gameflow;
pub mod honor;
pub mod league_session;
pub mod loadouts;
pub mod lobby;
pub mod lobby_team_builder;
pub mod login;
pub mod loot;
pub mod match_history;
pub mod matchmaking;
pub mod missions;
pub mod perks;
pub mod player_notifications;
pub mod pre_end_of_game;
pub mod process_control;
pub mod ranked;
pub mod regalia;
pub mod remedy;
pub mod replays;
pub mod reward_track;
pub mod rewards;
pub mod riotclient;
pub mod spectator;
pub mod store;
pub mod summoner;

use super::http::HttpClient;
use challenges::ChallengesHttpApi;
use champ_select::ChampSelectHttpApi;
use champion_mastery::ChampionMasteryHttpApi;
use chat::ChatHttpApi;
use end_of_game::EndOfGameHttpApi;
use entitlements::EntitlementsHttpApi;
use event_hub::EventHubHttpApi;
use game_data::GameDataHttpApi;
use gameflow::GameflowHttpApi;
use honor::HonorHttpApi;
use league_session::LeagueSessionHttpApi;
use loadouts::LoadoutsHttpApi;
use lobby::LobbyHttpApi;
use lobby_team_builder::LobbyTeamBuilderHttpApi;
use login::LoginHttpApi;
use loot::LootHttpApi;
use match_history::MatchHistoryHttpApi;
use matchmaking::MatchmakingHttpApi;
use missions::MissionsHttpApi;
use perks::PerksHttpApi;
use player_notifications::PlayerNotificationsHttpApi;
use pre_end_of_game::PreEndOfGameHttpApi;
use process_control::ProcessControlHttpApi;
use ranked::RankedHttpApi;
use regalia::RegaliaHttpApi;
use remedy::RemedyHttpApi;
use replays::ReplaysHttpApi;
use reward_track::RewardTrackHttpApi;
use rewards::RewardsHttpApi;
use riotclient::RiotClientHttpApi;
use spectator::SpectatorHttpApi;
use store::StoreHttpApi;
use summoner::SummonerHttpApi;

pub struct LeagueClientHttpApiAxiosHelper {
    pub champ_select: ChampSelectHttpApi,
    pub champion_mastery: ChampionMasteryHttpApi,
    pub chat: ChatHttpApi,
    pub entitlements: EntitlementsHttpApi,
    pub game_data: GameDataHttpApi,
    pub gameflow: GameflowHttpApi,
    pub honor: HonorHttpApi,
    pub lobby: LobbyHttpApi,
    pub login: LoginHttpApi,
    pub league_session: LeagueSessionHttpApi,
    pub loot: LootHttpApi,
    pub match_history: MatchHistoryHttpApi,
    pub matchmaking: MatchmakingHttpApi,
    pub player_notifications: PlayerNotificationsHttpApi,
    pub process_control: ProcessControlHttpApi,
    pub ranked: RankedHttpApi,
    pub riotclient: RiotClientHttpApi,
    pub spectator: SpectatorHttpApi,
    pub summoner: SummonerHttpApi,
    pub regalia: RegaliaHttpApi,
    pub loadouts: LoadoutsHttpApi,
    pub challenges: ChallengesHttpApi,
    pub perks: PerksHttpApi,
    pub missions: MissionsHttpApi,
    pub end_of_game: EndOfGameHttpApi,
    pub remedy: RemedyHttpApi,
    pub pre_end_of_game: PreEndOfGameHttpApi,
    pub event_hub: EventHubHttpApi,
    pub rewards: RewardsHttpApi,
    pub reward_track: RewardTrackHttpApi,
    pub store: StoreHttpApi,
    pub replays: ReplaysHttpApi,
    pub lobby_team_builder: LobbyTeamBuilderHttpApi,
}

impl LeagueClientHttpApiAxiosHelper {
    /// reqwest::Client 内部使用 Arc 管理连接池，clone() 是轻量操作（仅复制引用计数），性能开销可忽略。
    pub fn new(client: HttpClient) -> Self {
        Self {
            champ_select: ChampSelectHttpApi::new(client.clone()),
            champion_mastery: ChampionMasteryHttpApi::new(client.clone()),
            chat: ChatHttpApi::new(client.clone()),
            entitlements: EntitlementsHttpApi::new(client.clone()),
            game_data: GameDataHttpApi::new(client.clone()),
            gameflow: GameflowHttpApi::new(client.clone()),
            honor: HonorHttpApi::new(client.clone()),
            lobby: LobbyHttpApi::new(client.clone()),
            login: LoginHttpApi::new(client.clone()),
            league_session: LeagueSessionHttpApi::new(client.clone()),
            loot: LootHttpApi::new(client.clone()),
            match_history: MatchHistoryHttpApi::new(client.clone()),
            matchmaking: MatchmakingHttpApi::new(client.clone()),
            player_notifications: PlayerNotificationsHttpApi::new(client.clone()),
            process_control: ProcessControlHttpApi::new(client.clone()),
            ranked: RankedHttpApi::new(client.clone()),
            riotclient: RiotClientHttpApi::new(client.clone()),
            spectator: SpectatorHttpApi::new(client.clone()),
            summoner: SummonerHttpApi::new(client.clone()),
            regalia: RegaliaHttpApi::new(client.clone()),
            loadouts: LoadoutsHttpApi::new(client.clone()),
            challenges: ChallengesHttpApi::new(client.clone()),
            perks: PerksHttpApi::new(client.clone()),
            missions: MissionsHttpApi::new(client.clone()),
            end_of_game: EndOfGameHttpApi::new(client.clone()),
            remedy: RemedyHttpApi::new(client.clone()),
            pre_end_of_game: PreEndOfGameHttpApi::new(client.clone()),
            event_hub: EventHubHttpApi::new(client.clone()),
            rewards: RewardsHttpApi::new(client.clone()),
            reward_track: RewardTrackHttpApi::new(client.clone()),
            store: StoreHttpApi::new(client.clone()),
            replays: ReplaysHttpApi::new(client.clone()),
            lobby_team_builder: LobbyTeamBuilderHttpApi::new(client.clone()),
        }
    }
}
