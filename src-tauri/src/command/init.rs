use crate::shared::init::game_data;
use tauri::Runtime;

#[tauri::command]
pub async fn check_init_status<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<bool, String> {
    // 检查关键组件是否已初始化
    let cache_ok = game_data::is_cache_initialized().await;
    Ok(cache_ok)
}
