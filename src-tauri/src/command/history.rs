use crate::shared::web_api::record::get_record_list;
use crate::shared::web_api::record::RecordItem;
use tauri::Runtime;

/// 获取召唤师头像图标（Base64 编码）
///
/// # 参数
/// - `icon_id`: 头像图标 ID
///
/// # 返回
/// - Base64 编码的图片数据 URL，可直接用于 `<img src="...">`
#[tauri::command]
pub async fn get_rank_list<R: Runtime>(
    _app: tauri::AppHandle<R>,
    _window: tauri::Window<R>,
    puuid: &str,
    beg_index: Option<i32>,
    end_index: Option<i32>,
) -> Result<Vec<RecordItem>, String> {
    get_record_list(puuid, beg_index, end_index).await
}
