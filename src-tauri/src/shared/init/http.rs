use crate::shared::http_api::league_client::LeagueClientHttpApi;
use crate::shared::process::get_client_info;
use crate::utils::error::init_error::InitError;
use tokio::sync::RwLock;


static HTTP_CLIENT: RwLock<Option<LeagueClientHttpApi>> = RwLock::const_new(None);

pub async fn init_http_client() -> Result<(), InitError> {
    let info = get_client_info().map_err(|e| InitError::Init(e.to_string()))?;
    let client = LeagueClientHttpApi::new(info.port, info.auth_token, true);
    let mut guard = HTTP_CLIENT.write().await;
    *guard = Some(client);
    Ok(())
}


pub async fn get_http_client() -> Result<LeagueClientHttpApi, InitError> {
    let guard = HTTP_CLIENT.read().await;
    guard
        .clone()
        .ok_or_else(|| InitError::Get("获取客户端信息失败".to_string()))
}

pub async fn clear_http_client() {
    let mut guard = HTTP_CLIENT.write().await;
    *guard = None;
}
