use crate::shared::web_api::info::{get_info as get_info_api, Info}; // TODO: 重命名
use tauri::Runtime;
use tracing::info;

#[tauri::command]
pub async fn get_info<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    puuid: &str,
) -> Result<Info, String> {
    info!("get_info: puuid: {}", puuid);
    get_info_api(puuid).await
}
