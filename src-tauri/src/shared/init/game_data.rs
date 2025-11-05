use crate::shared::init::http::get_http_client;
use crate::utils::error::init_error::InitError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

static ITEM_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, String>>>> = RwLock::const_new(None);
static SPELL_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, String>>>> = RwLock::const_new(None);
static PERK_ICONS_CACHE: RwLock<Option<Arc<HashMap<String, String>>>> = RwLock::const_new(None);

pub async fn init_item_icons_cache() -> Result<(), InitError> {
    let client = get_http_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let items = client
        .game_data
        .get_items()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut items_map = HashMap::new();
    for item in items {
        items_map.insert(item.id.to_string(), item.icon_path);
    }
    let mut guard = ITEM_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(items_map));
    Ok(())
}

/// 获取物品图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
/// 
/// # 返回
/// - `Ok(Arc<HashMap<String, String>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_item_icons_cache() -> Result<Arc<HashMap<String, String>>, InitError> {
    let guard = ITEM_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .ok_or_else(|| InitError::Init("物品图标缓存未初始化".to_string()))
}

pub async fn clear_item_icons_cache() {
    let mut guard = ITEM_ICONS_CACHE.write().await;
    *guard = None;
}

pub async fn init_spell_icons_cache() -> Result<(), InitError> {
    let client = get_http_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let spells = client
        .game_data
        .get_summoner_spells()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut spells_map = HashMap::new();
    for spell in spells {
        spells_map.insert(spell.id.to_string(), spell.icon_path);
    }
    let mut guard = SPELL_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(spells_map));
    Ok(())
}

/// 获取召唤师技能图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
/// 
/// # 返回
/// - `Ok(Arc<HashMap<String, String>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_spell_icons_cache() -> Result<Arc<HashMap<String, String>>, InitError> {
    let guard = SPELL_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .ok_or_else(|| InitError::Init("召唤师技能图标缓存未初始化".to_string()))
}

pub async fn clear_spell_icons_cache() {
    let mut guard = SPELL_ICONS_CACHE.write().await;
    *guard = None;
}

pub async fn init_perk_icons_cache() -> Result<(), InitError> {
    let client = get_http_client()
        .await
        .map_err(|e: InitError| InitError::Init(e.to_string()))?;
    let perks = client
        .game_data
        .get_perks()
        .await
        .map_err(|e| InitError::Init(e.to_string()))?;
    let mut perks_map = HashMap::new();
    for perk in perks {
        perks_map.insert(perk.id.to_string(), perk.icon_path);
    }
    let mut guard = PERK_ICONS_CACHE.write().await;
    *guard = Some(Arc::new(perks_map));
    Ok(())
}

/// 获取符文图标缓存（返回 Arc 引用，避免克隆整个 HashMap）
/// 
/// # 返回
/// - `Ok(Arc<HashMap<String, String>>)`: 缓存的 Arc 引用（只克隆 Arc 指针，不克隆 HashMap）
/// - `Err(InitError)`: 缓存未初始化时返回错误
pub async fn get_perk_icons_cache() -> Result<Arc<HashMap<String, String>>, InitError> {
    let guard = PERK_ICONS_CACHE.read().await;
    guard
        .as_ref()
        .map(|arc| Arc::clone(arc))
        .ok_or_else(|| InitError::Init("符文图标缓存未初始化".to_string()))
}

pub async fn clear_perk_icons_cache() {
    let mut guard = PERK_ICONS_CACHE.write().await;
    *guard = None;
}
