use crate::shared::web_api::record_sgp::{get_record_list, RecordItem};
use tauri::Runtime;
use tracing::info;

/// 获取召唤师头像图标（Base64 编码）
///
/// # 参数
/// - `icon_id`: 头像图标 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL，可直接用于 `<img src="...">`
/// 这一步慢纯粹就是后端接口慢，跟前端没关系
#[tauri::command]
pub async fn get_rank_list<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    puuid: &str,
    beg_index: i32,
    end_index: i32,
) -> Result<Vec<RecordItem>, String> {
    info!("get_rank_list: puuid: {}, beg_index: {}, end_index: {}", puuid, beg_index, end_index);
    get_record_list(puuid, beg_index, end_index).await
}
