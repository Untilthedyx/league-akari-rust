use crate::shared::init::lcu::get_lcu_client;
use crate::utils::error::init_error::InitError;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub id: i64,
    pub name: String,
    pub icon_path: String,
}

static CHAMPION_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, Item>>>> = RwLock::const_new(None);
static ITEM_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, Item>>>> = RwLock::const_new(None);
static SPELL_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, Item>>>> = RwLock::const_new(None);
static PERK_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, Item>>>> = RwLock::const_new(None);

pub async fn init_champion_info_cache() -> Result<(), InitError> {
    let client = get_lcu_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let champions = client
        .game_data
        .get_champion_summary()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut champions_map = HashMap::new();
    for champion in champions {
        champions_map.insert(
            champion.id.to_string(),
            Item {
                id: champion.id,
                name: champion.name,
                icon_path: champion.square_portrait_path,
            },
        );
    }
    let mut guard = CHAMPION_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(champions_map));
    Ok(())
}

pub async fn get_champion_info_cache() -> Arc<HashMap<String, Item>> {
    let guard = CHAMPION_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .unwrap_or_else(|| panic!("英雄图标缓存未初始化"))
}

pub async fn clear_champion_info_cache() {
    let mut guard = CHAMPION_ICONS_CACHE.write().await;
    *guard = None;
}

pub async fn init_item_info_cache() -> Result<(), InitError> {
    let client = get_lcu_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let items = client
        .game_data
        .get_items()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut items_map = HashMap::new();
    for item in items {
        items_map.insert(
            item.id.to_string(),
            Item {
                id: item.id,
                name: item.name,
                icon_path: item.icon_path,
            },
        );
    }
    let mut guard = ITEM_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(items_map));
    Ok(())
}

/// 获取物品图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
///
/// # 返回
/// - `Ok(Arc<HashMap<String, Item>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_item_info_cache() -> Arc<HashMap<String, Item>> {
    let guard = ITEM_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .unwrap_or_else(|| panic!("物品图标缓存未初始化"))
}

pub async fn clear_item_info_cache() {
    let mut guard = ITEM_ICONS_CACHE.write().await;
    *guard = None;
}

pub async fn init_spell_info_cache() -> Result<(), InitError> {
    let client = get_lcu_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let spells = client
        .game_data
        .get_summoner_spells()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut spells_map = HashMap::new();
    for spell in spells {
        spells_map.insert(
            spell.id.to_string(),
            Item {
                id: spell.id,
                name: spell.name,
                icon_path: spell.icon_path,
            },
        );
    }
    let mut guard = SPELL_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(spells_map));
    Ok(())
}

/// 获取召唤师技能图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
///
/// # 返回
/// - `Ok(Arc<HashMap<String, Item>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_spell_info_cache() -> Arc<HashMap<String, Item>> {
    let guard = SPELL_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .unwrap_or_else(|| panic!("召唤师技能图标缓存未初始化"))
}

pub async fn clear_spell_info_cache() {
    let mut guard = SPELL_ICONS_CACHE.write().await;
    *guard = None;
}

pub async fn init_perk_info_cache() -> Result<(), InitError> {
    let client = get_lcu_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let perks = client
        .game_data
        .get_perks()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut perks_map = HashMap::new();
    for perk in perks {
        perks_map.insert(
            perk.id.to_string(),
            Item {
                id: perk.id,
                name: perk.name,
                icon_path: perk.icon_path,
            },
        );
    }
    let mut guard = PERK_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(perks_map));
    Ok(())
}

/// 获取符文图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
///
/// # 返回
/// - `Ok(Arc<HashMap<String, Item>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_perk_info_cache() -> Arc<HashMap<String, Item>> {
    let guard = PERK_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .unwrap_or_else(|| panic!("符文图标缓存未初始化"))
}

pub async fn clear_perk_info_cache() {
    let mut guard = PERK_ICONS_CACHE.write().await;
    *guard = None;
}
