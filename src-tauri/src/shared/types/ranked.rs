use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 对应 TypeScript 的 RankedTiers 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedTiers {
    pub achieved_tiers: Vec<AchievedTier>, // TypeScript 数组对应 Rust Vec
    pub summoner_id: i64,                  // 大整数用 i64 更安全（TypeScript number 对应）
}

/// 对应 TypeScript 的 AchievedTier 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AchievedTier {
    pub division: i32,
    pub queue_type: String, // 驼峰转蛇形（TypeScript queueType -> Rust queue_type）
    pub tier: String,
}

/// 对应 TypeScript 的 RankedStats 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedStats {
    pub earned_regalia_reward_ids: Vec<()>, // TypeScript any[] 对应空元组（表示任意类型）
    pub highest_current_season_reached_tier_sr: String,
    pub highest_previous_season_achieved_division: String,
    pub highest_previous_season_achieved_tier: String,
    pub highest_previous_season_end_division: String,
    pub highest_previous_season_end_tier: String,
    pub highest_ranked_entry: RankedEntry,
    pub highest_ranked_entry_sr: RankedEntry,
    pub queue_map: QueueMap,
    pub queues: Vec<RankedEntry>,
    pub ranked_regalia_level: i32,
    pub seasons: Seasons,
    pub splits_progress: HashMap<String, i32>, // TypeScript Record<string, number> 对应 HashMap
}

/// 对应 TypeScript 的 Seasons 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Seasons {
    #[serde(rename = "CHERRY")] // 保持与 TypeScript 键名一致（非蛇形）
    pub cherry: Cherry,
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: Cherry,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: Cherry,
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: Cherry,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: Cherry,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: Cherry,
}

/// 对应 TypeScript 的 CHERRY 接口（重命名为 Cherry 符合 Rust 命名规范）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Cherry {
    pub current_season_end: i64, // 时间戳用 i64
    pub current_season_id: i32,
    pub next_season_start: i64,
}

/// 对应 TypeScript 的 QueueMap 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueueMap {
    #[serde(rename = "CHERRY")]
    pub cherry: RankedEntry,
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: RankedEntry,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: RankedEntry,
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: RankedEntry,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: RankedEntry,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: RankedEntry,
}

/// 对应 TypeScript 的 RankedEntry 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedEntry {
    pub division: String,
    pub highest_division: String,
    pub highest_tier: String,
    pub is_provisional: bool,
    pub league_points: i32,
    pub losses: i32,
    pub mini_series_progress: String,
    pub previous_season_achieved_division: String,
    pub previous_season_achieved_tier: String,
    pub previous_season_end_division: String,
    pub previous_season_end_tier: String,
    pub provisional_game_threshold: i32,
    pub provisional_games_remaining: i32,
    pub previous_season_highest_tier: String,
    pub previous_season_highest_division: String,
    pub queue_type: String,
    pub rated_rating: i32,
    pub rated_tier: String,
    pub tier: String,
    pub warnings: Option<()>, // TypeScript 可选的 any 类型，用 Option<()> 表示
    pub wins: i32,
}
