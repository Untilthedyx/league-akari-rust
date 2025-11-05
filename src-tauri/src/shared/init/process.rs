use crate::shared::process::get_client_info;
use crate::shared::process::process_info::ProcessInfo;
use crate::utils::error::init_error::InitError;
use tokio::sync::RwLock;

static PROCESS_INFO: RwLock<Option<ProcessInfo>> = RwLock::const_new(None);

pub async fn init_process_info() -> Result<(), InitError> {
    let info = get_client_info().map_err(|e| InitError::Init(e))?;
    let mut guard = PROCESS_INFO.write().await;
    *guard = Some(info);

    Ok(())
}

pub async fn get_process_info() -> Result<ProcessInfo, InitError> {
    let info = PROCESS_INFO.read().await;
    info.clone()
        .ok_or_else(|| InitError::Get("进程信息未初始化".to_string()))
}

pub async fn clear_process_info() {
    let mut guard = PROCESS_INFO.write().await;
    *guard = None;
}
