use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgpSpectatorData {
    pub reconnect_delay: i64,
    pub game_name: String,
    pub game: Game,
    pub player_credentials: PlayerCredentials,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: i64,
    pub game_state: String,
    pub queue_type_name: String,
    pub name: String,
    pub pick_turn: i64,
    pub map_id: i64,
    pub product: String,
    pub game_mode: String,
    pub max_num_players: i64,
    pub game_type: String,
    pub game_queue_config_id: i64,
    pub spectator_delay: i64,
    pub game_version: String,
    pub team_one: Vec<TeamOne>,
    pub team_two: Vec<TeamTwo>,
    pub player_champion_selections: Vec<PlayerChampionSelection>,
    pub banned_champions: Vec<Value>,
    pub observers: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamOne {
    pub puuid: String,
    pub summoner_id: i64,
    pub last_selected_skin_index: i64,
    pub team_owner: bool,
    pub profile_icon_id: i64,
    pub team_participant_id: i64,
    pub champion_id: i64,
    pub selected_role: String,
    pub selected_position: String,
    pub summoner_name: String,
    pub summoner_internal_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamTwo {
    pub last_selected_skin_index: i64,
    pub team_owner: bool,
    pub profile_icon_id: i64,
    pub team_participant_id: i64,
    pub champion_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerChampionSelection {
    pub puuid: String,
    pub champion_id: i64,
    pub selected_skin_index: i64,
    #[serde(rename = "spell1Id")]
    pub spell1id: i64,
    #[serde(rename = "spell2Id")]
    pub spell2id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCredentials {
    pub game_id: i64,
    pub queue_id: i64,
    pub player_id: i64,
    pub puuid: String,
    pub server_port: i64,
    pub spectator_key: String,
    pub champion_id: i64,
    pub last_selected_skin_index: i64,
    pub summoner_id: i64,
    pub observer: bool,
    pub game_version: String,
    pub game_mode: String,
    pub product: String,
    pub observer_encryption_key: String,
    pub observer_server_ip: String,
    pub observer_server_port: i64,
    pub queue_type: String,
    pub game_create_date: i64,
    pub packet_cop_metadata: String,
}
