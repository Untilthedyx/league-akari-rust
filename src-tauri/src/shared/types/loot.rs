use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 战利品映射表（以 lootId 为键）
pub type LootMap = HashMap<String, Loot>;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Loot {
    pub asset: String,
    pub count: i32,
    pub disenchant_loot_name: String,
    pub disenchant_recipe_name: String,
    pub disenchant_value: i32,
    pub display_categories: String,
    pub expiry_time: i64,
    pub is_new: bool,
    pub is_rental: bool,
    pub item_desc: String,
    pub item_status: String,
    pub localized_description: String,
    pub localized_name: String,
    pub localized_recipe_subtitle: String,
    pub localized_recipe_title: String,
    pub loot_id: String,
    pub loot_name: String,
    pub parent_item_status: String,
    pub parent_store_item_id: i32,
    pub rarity: String,
    pub redeemable_status: String,
    pub ref_id: String,
    pub rental_games: i32,
    pub rental_seconds: i64,
    pub shadow_path: String,
    pub splash_path: String,
    pub store_item_id: i32,
    pub tags: String,
    pub tile_path: String,
    pub r#type: String, // 使用 r# 转义关键字 type
    pub upgrade_essence_name: String,
    pub upgrade_essence_value: i32,
    pub upgrade_loot_name: String,
    pub value: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootCraftResponse {
    pub added: Vec<Added>,
    pub redeemed: Vec<serde_json::Value>, // any[] 适配动态类型
    pub removed: Vec<Added>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Added {
    pub delta_count: i32,
    pub player_loot: PlayerLoot,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLoot {
    pub asset: String,
    pub count: i32,
    pub disenchant_loot_name: String,
    pub disenchant_recipe_name: String,
    pub disenchant_value: i32,
    pub display_categories: String,
    pub expiry_time: i64,
    pub is_new: bool,
    pub is_rental: bool,
    pub item_desc: String,
    pub item_status: String,
    pub localized_description: String,
    pub localized_name: String,
    pub localized_recipe_subtitle: String,
    pub localized_recipe_title: String,
    pub loot_id: String,
    pub loot_name: String,
    pub parent_item_status: String,
    pub parent_store_item_id: i32,
    pub rarity: String,
    pub redeemable_status: String,
    pub ref_id: String,
    pub rental_games: i32,
    pub rental_seconds: i64,
    pub shadow_path: String,
    pub splash_path: String,
    pub store_item_id: i32,
    pub tags: String,
    pub tile_path: String,
    pub r#type: String, // 转义关键字 type
    pub upgrade_essence_name: String,
    pub upgrade_essence_value: i32,
    pub upgrade_loot_name: String,
    pub value: i32,
}
