use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectSearch {
    pub dodge_data: DodgeData,
    pub errors: Vec<serde_json::Value>, // any[]
    pub estimated_queue_time: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub low_priority_data: LowPriorityData,
    pub queue_id: i32,
    pub ready_check: ReadyCheck,
    pub search_state: String,
    pub time_in_queue: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReadyCheck {
    pub decliner_ids: Vec<serde_json::Value>, // any[]
    pub dodge_warning: String,
    pub player_response: String,
    pub state: String,
    pub suppress_ux: bool,
    pub timer: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LowPriorityData {
    pub busted_leaver_access_token: String,
    pub penalized_summoner_ids: Vec<serde_json::Value>, // any[]
    pub penalty_time: i32,
    pub penalty_time_remaining: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSearch {
    pub dodge_data: DodgeData,
    pub errors: Vec<Error>,
    pub estimated_queue_time: i32,
    pub is_currently_in_queue: bool,
    pub lobby_id: String,
    pub low_priority_data: LowPriorityData,
    pub queue_id: i32,
    pub ready_check: ReadyCheck,
    pub search_state: String,
    pub time_in_queue: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    pub error_type: String,
    pub id: i32,
    pub message: String,
    pub penalized_summoner_id: i64,
    pub penalty_time_remaining: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DodgeData {
    pub dodger_id: i64,
    pub state: String,
}
