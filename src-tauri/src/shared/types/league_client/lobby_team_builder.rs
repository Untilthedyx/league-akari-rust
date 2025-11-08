use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LobbyTeamBuilderChampSelectSession {
    pub actions: Vec<serde_json::Value>, // any[] 用 serde_json::Value 适配动态类型
    pub allow_battle_boost: bool,
    pub allow_duplicate_picks: bool,
    pub allow_locked_events: bool,
    pub allow_rerolling: bool,
    pub allow_skin_selection: bool,
    pub bench_champions: Vec<BenchChampion>,
    pub bench_enabled: bool,
    pub boostable_skin_count: i32,
    pub chat_details: ChatDetails,
    pub counter: i32,
    pub entitled_feature_state: EntitledFeatureState,
    pub game_id: i64,
    pub has_simultaneous_bans: bool,
    pub has_simultaneous_picks: bool,
    pub is_spectating: bool,
    pub local_player_cell_id: i32,
    pub locked_event_index: i32,
    pub my_team: Vec<MyTeam>,
    pub pick_order_swaps: Vec<serde_json::Value>, // any[] 适配动态类型
    pub recovery_counter: i32,
    pub rerolls_remaining: i32,
    pub skip_champion_select: bool,
    pub their_team: Vec<MyTeam>,
    pub timer: Timer,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub cell_id: i32,
    pub id: i32,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Timer {
    pub adjusted_time_left_in_phase: i32,
    pub internal_now_in_epoch_ms: i64,
    pub is_infinite: bool,
    pub phase: String,
    pub total_time_in_phase: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MyTeam {
    pub assigned_position: String,
    pub cell_id: i32,
    pub champion_id: i32,
    pub champion_pick_intent: i32,
    pub entitled_feature_type: String,
    pub name_visibility_type: String,
    pub obfuscated_puuid: String,
    pub obfuscated_summoner_id: i64,
    pub player_type: String,
    pub puuid: String,
    pub selected_skin_id: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub summoner_id: i64,
    pub team: i32,
    pub ward_skin_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntitledFeatureState {
    pub additional_rerolls: i32,
    pub unlocked_skin_ids: Vec<serde_json::Value>, // any[] 适配动态类型
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChatDetails {
    pub chat_room_name: String,
    pub chat_room_password: Option<serde_json::Value>, // 可选且类型不确定的字段
    pub multi_user_chat_jwt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BenchChampion {
    pub champion_id: i32,
    pub is_priority: bool,
}
