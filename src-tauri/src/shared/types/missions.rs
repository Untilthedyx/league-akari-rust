use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub background_image_url: String,
    pub celebration_type: String,
    pub client_notify_level: String,
    pub completed_date: i64,
    pub completion_expression: String,
    pub cooldown_time_millis: i64,
    pub description: String,
    pub display: Display,
    pub display_type: String,
    pub earned_date: i64,
    pub end_time: i64,
    pub expiring_warnings: Vec<ExpiringWarning>,
    pub helper_text: String,
    pub icon_image_url: String,
    pub id: String,
    pub internal_name: String,
    pub is_new: bool,
    pub last_updated_timestamp: i64,
    pub locale: String,
    pub media: MissionMedia,
    pub metadata: Metadata,
    pub mission_type: String,
    pub objectives: Vec<Objective>,
    pub requirements: Vec<String>,
    pub reward_strategy: RewardStrategy,
    pub rewards: Vec<Reward>,
    pub sequence: i32,
    pub series_name: String,
    pub start_time: i64,
    pub status: String,
    pub title: String,
    pub viewed: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub description: String,
    pub icon_needs_frame: bool,
    pub icon_url: String,
    pub is_objective_based_reward: bool,
    pub item_id: String,
    pub media: Media2,
    pub quantity: i32,
    pub reward_fulfilled: bool,
    pub reward_group: String,
    pub reward_group_selected: bool,
    pub reward_type: String,
    pub sequence: i32,
    pub small_icon_url: String,
    pub unique_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Media2 {
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardStrategy {
    pub group_strategy: String,
    pub select_max_group_count: i32,
    pub select_min_group_count: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Objective {
    pub description: String,
    pub has_objective_based_reward: bool,
    pub progress: Progress,
    pub requirements: Vec<serde_json::Value>,  // any[]
    pub reward_groups: Vec<serde_json::Value>, // any[]
    pub sequence: i32,
    pub status: String,
    pub r#type: String, // 转义关键字 type
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Progress {
    pub current_progress: i32,
    pub last_viewed_progress: i32,
    pub total_count: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub chain: i32,
    pub chain_size: i32,
    pub mission_type: String,
    pub npe_reward_pack: NpeRewardPack,
    pub order: i32,
    pub tutorial: Tutorial,
    pub week_num: i32,
    pub xp_reward: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tutorial {
    pub display_rewards: DisplayRewards,
    pub queue_id: String,
    pub step_number: i32,
    pub use_chosen_champion: bool,
    pub use_quick_search_matchmaking: bool,
}

/// 键为数字字符串（如 "1"、"3"）
pub type DisplayRewards = HashMap<String, String>;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NpeRewardPack {
    pub index: i32,
    pub major_reward: MajorReward,
    pub minor_rewards: Vec<MinorRewardVariant>, // 多种 minor reward 类型的统一表示
    pub premium_reward: bool,
    pub reward_key: String,
}

/// 统一表示多种 MinorReward 类型（通过 serde 动态解析）
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum MinorRewardVariant {
    Type1 { data: Data10, renderer: String },
    Type2 { data: Data9, renderer: String },
    Type3 { data: Data, renderer: String },
    Type4 { data: Data4, renderer: String },
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data10 {
    #[serde(rename = "hideInCalendarDetail")]
    pub hide_in_calendar_detail: Option<bool>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data9 {
    pub r#type: Option<String>, // 转义关键字 type
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "gameModes")]
    pub game_modes: Option<Vec<String>>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Data4 {
    pub quantity: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MajorReward {
    pub data: Option<MajorRewardData>, // 支持多种数据类型，可为 null
    pub renderer: String,
}

/// 统一表示 MajorReward 中的多种 data 类型
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum MajorRewardData {
    Datum {
        id: i32,
    },
    Data2 {
        #[serde(rename = "hideInCalendarDetail")]
        hide_in_calendar_detail: bool,
        ids: Vec<i32>,
    },
    Data3 {
        r#type: String,
    }, // 转义关键字 type
    Data4 {
        quantity: i32,
    },
    Data5 {
        #[serde(rename = "gameModes")]
        game_modes: Vec<String>,
    },
    Data6 {
        ids: Vec<i32>,
    },
    Data7 {
        champ_ids: Vec<i32>,
        r#type: String, // 转义关键字 type
    },
    Data8 {
        #[serde(rename = "gameModes")]
        game_modes: Vec<String>,
        #[serde(rename = "hasCustomDetailImage")]
        has_custom_detail_image: bool,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionMedia {
    #[serde(rename = "mission_icon")]
    pub mission_icon: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExpiringWarning {
    pub alert_time: i64,
    pub message: String,
    pub r#type: String, // 转义关键字 type
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    pub attributes: Vec<String>,
    pub locations: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionData {
    pub level: i32,
    pub loyalty_enabled: bool,
    pub player_inventory: PlayerInventory,
    pub user_info_token: Option<()>, // 对应 TypeScript 中的 null
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    pub champions: Vec<serde_json::Value>, // any[]
    pub icons: Vec<serde_json::Value>,     // any[]
    pub inventory_jwts: Vec<String>,
    pub skins: Vec<serde_json::Value>,      // any[]
    pub ward_skins: Vec<serde_json::Value>, // any[]
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MissionSeries {
    pub created_date: i64,
    pub description: String,
    pub display_type: String,
    pub eligibility_type: String,
    pub end_date: i64,
    pub id: String,
    pub internal_name: String,
    pub last_updated_timestamp: i64,
    pub media: SeriesMedia,
    pub opt_in_button_text: String,
    pub opt_out_button_text: String,
    pub parent_internal_name: String,
    pub start_date: i64,
    pub status: String,
    pub tags: Vec<String>,
    pub title: String,
    pub r#type: String, // 转义关键字 type
    pub viewed: bool,
    pub warnings: Vec<serde_json::Value>, // any[]
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeriesMedia {
    pub accent_color: String,
    pub background_image_large_url: String,
    pub background_image_small_url: String,
    pub background_url: String,
    pub tracker_icon: String,
    pub tracker_icon_url: String,
}
