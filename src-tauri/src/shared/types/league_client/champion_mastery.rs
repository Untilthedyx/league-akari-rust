use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerChampionMastery {
    pub masteries: Vec<Mastery>,
    pub puuid: String,
    pub score: i64,
    pub summoner_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mastery {
    pub champion_id: i64,
    pub champion_level: i64,
    pub champion_points: i64,
    pub champion_points_since_last_level: i64,
    pub champion_points_until_next_level: i64,
    pub champion_season_milestone: i64,
    pub highest_grade: Option<String>,
    pub last_play_time: i64,
    pub mark_required_for_next_level: i64,
    pub milestone_grades: Vec<String>,
    pub next_season_milestone: Option<NextSeasonMilestone>,
    pub puuid: String,
    pub tokens_earned: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextSeasonMilestone {
    pub bonus: bool,
    pub require_grade_counts: HashMap<String, i64>,
    pub reward_config: RewardConfig,
    pub reward_marks: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardConfig {
    pub maximum_reward: i64,
    pub reward_value: String,
}
