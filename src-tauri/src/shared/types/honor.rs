use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BallotLegacy {
    pub eligible_players: Vec<EligiblePlayerLegacy>,
    pub game_id: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EligiblePlayerLegacy {
    pub champion_name: String,
    pub skin_splash_path: String,
    pub summoner_id: i64,
    pub summoner_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BallotLegacy2 {
    pub eligible_allies: Vec<()>, // any[] 无法精确转换，这里用空元组占位，可根据实际类型修改
    pub eligible_opponents: Vec<()>, // 同上
    pub game_id: i64,
    pub honored_players: Vec<()>, // 同上
    pub num_votes: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ballot {
    pub eligible_allies: Vec<EligiblePlayer>,
    pub eligible_opponents: Vec<EligiblePlayer>,
    pub game_id: i64,
    pub honored_players: Vec<HonoredPlayer>,
    pub vote_pool: VotePool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct HonoredPlayer {
    pub honor_type: String,
    pub recipient_puuid: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VotePool {
    pub from_game_played: i64,
    pub from_high_honor: i64,
    pub from_recent_honors: i64,
    pub from_rollover: i64,
    pub votes: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EligiblePlayer {
    pub bot_player: bool,
    pub champion_name: String,
    pub puuid: String,
    pub role: String,
    pub skin_splash_path: String,
    pub summoner_id: i64,
    pub summoner_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Recognition {
    pub honor_category: String,
    pub sender_puuid: String,
    pub voter_relationship: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum HonorCategory {
    Cool,
    Shotcaller,
    Heart,
    #[default]
    #[serde(rename = "")]
    Empty,
    OptOut,
}

// V2 荣誉请求体结构
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct V2HonorRequest {
    pub game_id: String, // 同时支持字符串和数字，统一转为 String 处理
    pub honor_category: HonorCategory,
    pub summoner_id: Option<String>, // 可选字段
    pub puuid: Option<String>,       // 可选字段
}

// 荣誉请求体结构
#[derive(Debug, Serialize, Default)]
pub struct HonorRequest {
    pub honor_type: String,
    pub recipient_puuid: String,
}