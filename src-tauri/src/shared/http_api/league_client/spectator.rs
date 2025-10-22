use crate::shared::http_api::league_client::httpclient::HttpClient;
use crate::utils::error::http_error::HttpError;

/// Riot 客户端相关的 HTTP API 客户端
pub struct SpectatorHttpApi {
    client: HttpClient,
}

impl SpectatorHttpApi {
    /// 创建新的 SpectatorHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn launch_spectator(&self, puuid: &String) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-spectator/v1/spectate/launch";

        let data = serde_json::json!({
        "allowObserveMode": "ALL",
        "dropInSpectateGameId": "",
        "gameQueueType": "",
        "puuid": puuid,
        });

        self.client.post(url, Some(&data)).await
    }
}
