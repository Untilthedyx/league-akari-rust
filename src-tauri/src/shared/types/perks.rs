use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerkPage {
    pub auto_modified_selections: Vec<serde_json::Value>, // any[]
    pub current: bool,
    pub id: i64,
    pub is_active: bool,
    pub is_deletable: bool,
    pub is_editable: bool,
    pub is_recommendation_override: bool,
    pub is_temporary: bool,
    pub is_valid: bool,
    pub last_modified: i64,
    pub name: String,
    pub order: i32,
    pub page_keystone: PageKeystone,
    pub primary_style_icon_path: String,
    pub primary_style_id: i32,
    pub primary_style_name: String,
    pub quick_play_champion_ids: Vec<serde_json::Value>, // any[]
    pub recommendation_champion_id: i32,
    pub recommendation_index: i32,
    pub rune_recommendation_id: String,
    pub secondary_style_icon_path: String,
    pub secondary_style_name: String,
    pub selected_perk_ids: Vec<i32>,
    pub sub_style_id: i32,
    pub tooltip_bg_path: String,
    pub ui_perks: Vec<PageKeystone>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PageKeystone {
    pub icon_path: String,
    pub id: i32,
    pub name: String,
    pub slot_type: String,
    pub style_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerkInventory {
    pub can_add_custom_page: bool,
    pub custom_page_count: i32,
    pub is_custom_page_creation_unlocked: bool,
    pub owned_page_count: i32,
}

/// 键为英雄 ID（number），值为推荐位置信息
pub type RecommendPositions = HashMap<i32, ChampionRecommendPositions>;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionRecommendPositions {
    pub recommended_positions: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecommendPage {
    pub is_default_position: bool,
    pub is_recommendation_override: bool,
    pub keystone: Keystone,
    pub perks: Vec<Keystone>,
    pub position: String,
    pub primary_perk_style_id: i32,
    pub primary_recommendation_attribute: String,
    pub recommendation_champion_id: i32,
    pub recommendation_id: String,
    pub secondary_perk_style_id: i32,
    pub secondary_recommendation_attribute: String,
    pub summoner_spell_ids: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Keystone {
    pub icon_path: String,
    pub id: i32,
    pub long_desc: String,
    pub name: String,
    pub recommendation_descriptor: String,
    pub short_desc: String,
    pub slot_type: String,
    pub style_id: i32,
    pub style_id_name: String,
    pub tooltip: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PostPerkDto {
    pub name: String,
    #[serde(rename = "isEditable")]
    pub is_editable: bool,
    #[serde(rename = "primaryStyleId")]
    pub primary_style_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PutPageDto {
    pub is_temporary: Option<bool>,
    pub rune_recommendation_id: Option<String>,
    pub recommendation_champion_id: Option<i32>,
    pub is_recommendation_override: Option<bool>,
    pub recommendation_index: Option<i32>,
    pub quick_play_champion_ids: Option<Vec<i32>>,
    pub primary_style_id: Option<i32>,
    pub sub_style_id: Option<i32>,
    pub selected_perk_ids: Option<Vec<i32>>,
    pub name: Option<String>,
    pub order: Option<i32>,
    pub id: i32, // 必须字段（用于 URL 路径）
}
