use crate::shared::http_api::lcu::LcuApi;
use crate::shared::init::process::get_process_info;
use crate::utils::error::init_error::InitError;
use tokio::sync::RwLock;

static LCU_CLIENT: RwLock<Option<LcuApi>> = RwLock::const_new(None);

pub async fn init_lcu_client() -> Result<(), InitError> {
    let info = get_process_info().await?;
    let base_url = "https://127.0.0.1".to_string();
    // token = username:password
    let token = format!("riot:{}", info.auth_token);
    let client = LcuApi::new(base_url, info.port, token);
    let mut guard = LCU_CLIENT.write().await;
    *guard = Some(client);
    Ok(())
}

pub async fn get_lcu_client() -> Result<LcuApi, InitError> {
    let guard = LCU_CLIENT.read().await;
    guard
        .clone()
        .ok_or_else(|| InitError::Get("获取客户端信息失败".to_string()))
}

pub async fn clear_lcu_client() {
    let mut guard = LCU_CLIENT.write().await;
    *guard = None;
}
