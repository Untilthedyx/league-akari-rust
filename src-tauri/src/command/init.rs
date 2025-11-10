use crate::core::app_init::init_and_clear::{parse_summoner, Summoner};
use crate::shared::init::game_data;
use crate::shared::init::lcu::get_lcu_client;
use tauri::Runtime;
use tracing::info;

#[tauri::command]
pub async fn check_init_status<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
) -> Result<(bool, Summoner), String> {
    // 检查关键组件是否已初始化
    info!("check_init_status");
    let cache_ok = game_data::is_cache_initialized().await;
    if !cache_ok{
        return Ok((false, Summoner::default()));
    }
    let client = get_lcu_client().await.unwrap();
    let summoner = client.summoner.get_current_summoner().await.unwrap();
    Ok((cache_ok, parse_summoner(&summoner).unwrap()))
}
