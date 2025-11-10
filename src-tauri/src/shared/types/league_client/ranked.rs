use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedStats {
    pub current_season_split_points: i64,
    pub earned_regalia_reward_ids: Vec<Value>,
    #[serde(rename = "highestCurrentSeasonReachedTierSR")]
    pub highest_current_season_reached_tier_sr: String,
    pub highest_previous_season_end_division: String,
    pub highest_previous_season_end_tier: String,
    pub highest_ranked_entry: HighestRankedEntry,
    #[serde(rename = "highestRankedEntrySR")]
    pub highest_ranked_entry_sr: HighestRankedEntry,
    pub previous_season_split_points: i64,
    pub queue_map: QueueMap,
    pub queues: Vec<HighestRankedEntry>,
    pub ranked_regalia_level: i64,
    pub seasons: Seasons,
    pub splits_progress: HashMap<String, i32>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HighestRankedEntry {
    pub current_season_wins_for_rewards: i64,
    pub division: String,
    pub highest_division: String,
    pub highest_tier: String,
    pub is_provisional: bool,
    pub league_points: i64,
    pub losses: i64,
    pub mini_series_progress: String,
    pub previous_season_end_division: String,
    pub previous_season_end_tier: String,
    pub previous_season_highest_division: String,
    pub previous_season_highest_tier: String,
    pub previous_season_wins_for_rewards: i64,
    pub provisional_game_threshold: i64,
    pub provisional_games_remaining: i64,
    pub queue_type: String,
    pub rated_rating: i64,
    pub rated_tier: String,
    pub tier: String,
    pub warnings: Value,
    pub wins: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueMap {
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: HighestRankedEntry,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: HighestRankedEntry,
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: HighestRankedEntry,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: HighestRankedEntry,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: HighestRankedEntry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seasons {
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: RankedFlexSr,
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: RankedFlexSr,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: RankedFlexSr,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: RankedFlexSr,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedFlexSr {
    pub current_season_end: i64,
    pub current_season_id: i64,
    pub next_season_start: i64,
}
