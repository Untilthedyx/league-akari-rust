use crate::shared::init::lcu::get_lcu_client;
use tauri::Runtime;

/// 获取召唤师头像图标（Base64 编码）
///
/// # 参数
/// - `icon_id`: 头像图标 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL，可直接用于 `<img src="...">`
#[tauri::command]
pub async fn get_profile_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    icon_id: u32,
) -> Result<String, String> {
    let client = get_lcu_client().await.map_err(|e| e.to_string())?;
    client
        .asset
        .get_profile_icon_base64(icon_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取英雄头像图标（Base64 编码）
///
/// # 参数
/// - `champion_id`: 英雄 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL
#[tauri::command]
pub async fn get_champion_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    champion_id: u32,
) -> Result<String, String> {
    let client = get_lcu_client().await.map_err(|e| e.to_string())?;
    client
        .asset
        .get_champion_icon_base64(champion_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取物品图标（Base64 编码）
///
/// # 参数
/// - `item_id`: 物品 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL
#[tauri::command]
pub async fn get_item_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    item_id: u32,
) -> Result<String, String> {
    let client = get_lcu_client().await.map_err(|e| e.to_string())?;
    client
        .asset
        .get_item_icon_base64(item_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取召唤师技能图标（Base64 编码）
///
/// # 参数
/// - `spell_id`: 召唤师技能 ID（如 "SummonerFlash"）
///
/// # 返回
/// - Base64 编码的图片数据 URL
#[tauri::command]
pub async fn get_spell_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    spell_id: String,
) -> Result<String, String> {
    let client = get_lcu_client().await.map_err(|e| e.to_string())?;
    client
        .asset
        .get_spell_icon_base64(&spell_id)
        .await
        .map_err(|e| e.to_string())
}

/// 获取符文图标（Base64 编码）
///
/// # 参数
/// - `perk_id`: 符文 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL
#[tauri::command]
pub async fn get_perk_icon<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    perk_id: String,
) -> Result<String, String> {
    let client = get_lcu_client().await.map_err(|e| e.to_string())?;
    client
        .asset
        .get_perk_icon_base64(&perk_id)
        .await
        .map_err(|e| e.to_string())
}
