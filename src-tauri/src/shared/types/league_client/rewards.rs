use serde::{Deserialize, Serialize};

/// 对应 TypeScript 的 RewardsGrant 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardsGrant {
    pub info: Info,
    pub reward_group: RewardGroup,
}

/// 对应 TypeScript 的 RewardGroup 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardGroup {
    pub active: bool,
    pub celebration_type: String,
    pub child_reward_group_ids: Vec<()>, // any[] 对应空元组向量
    pub id: String,
    pub localizations: Localizations2,
    pub media: Localizations, // 注意：此处与另一个 Media 类型区分
    pub product_id: String,
    pub reward_strategy: String,
    pub rewards: Vec<Reward>,
    pub selection_strategy_config: Option<SelectionStrategyConfig>, // 可为 null
    pub types: Vec<()>,                                             // any[] 对应空元组向量
}

/// 对应 TypeScript 的 SelectionStrategyConfig 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SelectionStrategyConfig {
    pub max_selections_allowed: i32,
    pub min_selections_allowed: i32,
}

/// 对应 TypeScript 的 Reward 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reward {
    pub fulfillment_source: String,
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub localizations: Localizations3,
    pub media: Media,
    pub quantity: i32,
}

/// 对应 TypeScript 的 Media 接口（Reward 中的媒体）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Media {
    pub icon_url: String,
}

/// 对应 TypeScript 的 Localizations3 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Localizations3 {
    pub details: String,
    pub title: String,
}

/// 对应 TypeScript 的 Localizations2 接口（RewardGroup 中的本地化）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Localizations2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // 可选字段
}

/// 对应 TypeScript 的 Info 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Info {
    pub date_created: String,
    pub grant_elements: Vec<GrantElement>,
    pub grantee_id: String,
    pub grantor_description: GrantorDescription,
    pub id: String,
    pub message_parameters: Localizations, // 通用本地化类型
    pub reward_group_id: String,
    pub selected_ids: Vec<()>, // any[] 对应空元组向量
    pub status: String,
    pub viewed: bool,
}

/// 对应 TypeScript 的 GrantorDescription 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrantorDescription {
    pub app_name: String,
    pub entity_id: String,
}

/// 对应 TypeScript 的 GrantElement 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GrantElement {
    pub element_id: String,
    pub fulfillment_source: String,
    pub item_id: String,
    pub item_type: String,
    pub localizations: Localizations, // 通用本地化类型
    pub media: Localizations,         // 此处 media 为本地化结构（与 Reward 中的 Media 不同）
    pub quantity: i32,
    pub status: String,
}

/// 通用 Localizations 接口（多处复用）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Localizations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // 可选字段
}

/// 对应 TypeScript 的 RewardsGroup 接口（注意与 RewardGroup 区分）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardsGroup {
    pub active: bool,
    pub celebration_type: String,
    pub child_reward_group_ids: Vec<()>, // any[] 对应空元组向量
    pub id: String,
    pub localizations: Localizations, // 通用本地化类型
    pub media: RewardsGroupMedia,     // 与其他 Media 类型区分
    pub product_id: String,
    pub reward_strategy: String,
    pub rewards: Vec<Reward2>,
    pub selection_strategy_config: Option<SelectionStrategyConfig>, // 可为 null
    pub types: Vec<String>,
}

/// 对应 TypeScript 的 Reward2 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reward2 {
    pub fulfillment_source: String,
    pub id: String,
    pub item_id: String,
    pub item_type: String,
    pub localizations: Reward2Localizations, // 与其他 Localizations2 区分
    pub media: Reward2Media,
    pub quantity: i32,
}

/// 对应 TypeScript 的 Media2 接口（Reward2 中的媒体）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reward2Media {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splash_image: Option<String>, // 可选字段
}

/// 对应 TypeScript 的 Localizations2 接口（Reward2 中的本地化）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Reward2Localizations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>, // 可选字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>, // 可选字段
}

/// 对应 TypeScript 的 Media 接口（RewardsGroup 中的媒体）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RewardsGroupMedia {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_background_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_design: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro_animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro_animation_audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro_low_spec_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loop_animation_audio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_animation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_animation_audio: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostGrantSelectionDto {
    pub grant_id: String,
    pub selections: Vec<String>,
    pub reward_group_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<String>,
}

/// 对应 TypeScript 的 PostCelebrationsFscDto 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostCelebrationsFscDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsc: Option<serde_json::Value>, // any 类型对应 serde_json::Value
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canvas: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewards: Option<serde_json::Value>,
}
