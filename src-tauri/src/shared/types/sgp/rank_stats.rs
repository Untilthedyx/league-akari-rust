use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgpRankedStats {
    pub queues: Vec<Queues>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub highest_previous_season_achieved_tier: String,
    pub highest_previous_season_achieved_rank: String,
    pub earned_regalia_reward_ids: Vec<Value>,
    pub splits_progress: SplitsProgress,
    pub seasons: Seasons,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Queues {
    pub queue_type: String,
    pub provisional_game_threshold: i64,
    pub tier: Option<String>,
    pub rank: Option<String>,
    pub league_points: i64,
    pub cumulative_lp: i64,
    pub wins: i64,
    pub losses: i64,
    pub current_season_wins_for_rewards: i64,
    pub previous_season_wins_for_rewards: i64,
    pub provisional_games_remaining: i64,
    pub highest_tier: Option<String>,
    pub highest_rank: Option<String>,
    pub previous_season_end_tier: Option<String>,
    pub previous_season_end_rank: Option<String>,
    pub previous_season_highest_tier: Option<String>,
    pub previous_season_highest_rank: Option<String>,
    pub previous_season_achieved_tier: Option<String>,
    pub previous_season_achieved_rank: Option<String>,
    pub rated_rating: i64,
    pub premade_mmr_restricted: bool,
    pub climbing_indicator_active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitsProgress {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Seasons {
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: RankedTft,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: RankedTft,
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: RankedTft,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: RankedTft,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: RankedTft,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RankedTft {
    pub current_season_id: i64,
    pub current_season_end: i64,
    pub next_season_start: i64,
}
