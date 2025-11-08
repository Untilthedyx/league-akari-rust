use crate::shared::http_api::sgp::SgpApi;
use crate::shared::process::get_client_info;
use crate::utils::error::init_error::InitError;
use tokio::sync::RwLock;

static SGP_CLIENT: RwLock<Option<SgpApi>> = RwLock::const_new(None);

pub async fn init_sgp_client() -> Result<(), InitError> {
    let info = get_client_info().map_err(|e| InitError::Init(e.to_string()))?;
    // token = username:password
    let client = SgpApi::new(info.rso_platform_id.as_str(), info.region.as_str());
    let mut guard = SGP_CLIENT.write().await;
    *guard = Some(client);
    Ok(())
}

pub async fn get_sgp_client() -> Result<SgpApi, InitError> {
    let guard = SGP_CLIENT.read().await;
    guard
        .clone()
        .ok_or_else(|| InitError::Get("获取客户端信息失败".to_string()))
}

pub async fn clear_sgp_client() {
    let mut guard = SGP_CLIENT.write().await;
    *guard = None;
}
