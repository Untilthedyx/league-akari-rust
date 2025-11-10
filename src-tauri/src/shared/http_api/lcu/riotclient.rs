use crate::shared::http_api::lcu::http::HttpClient;
use crate::utils::error::http_error::HttpError;

/// Riot 客户端相关的 HTTP API 客户端
#[derive(Clone)]
pub struct RiotClientHttpApi {
    client: HttpClient,
}

impl RiotClientHttpApi {
    /// 创建新的 RiotClientHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 终止 UX 进程
    pub async fn kill_ux(&self) -> Result<serde_json::Value, HttpError> {
        let url = "/riotclient/kill-ux";
        self.client.post(url, None::<&()>).await
    }

    /// 启动 UX 进程
    pub async fn launch_ux(&self) -> Result<serde_json::Value, HttpError> {
        let url = "/riotclient/launch-ux";
        self.client.post(url, None::<&()>).await
    }

    /// 重启 UX 进程（先终止再启动）
    pub async fn restart_ux(&self) -> Result<serde_json::Value, HttpError> {
        let url = "riotclient/kill-and-restart-ux"; // 保持原 URL 路径（缺少开头 / 按原定义保留）
        self.client.post(url, None::<&()>).await
    }

    /// 更新客户端 splash 界面
    pub async fn splash(&self) -> Result<serde_json::Value, HttpError> {
        let url = "/riotclient/splash";
        self.client.put(url, None::<&()>).await
    }

    pub async fn get_token(&self) -> Result<String, HttpError> {
        let url = "/riotclient/auth-token";
        self.client.get(url).await
    }
}