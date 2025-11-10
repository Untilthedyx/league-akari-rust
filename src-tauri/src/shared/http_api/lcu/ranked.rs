use crate::shared::http_api::lcu::http::HttpClient;
use crate::shared::types::league_client::ranked::RankedStats;
use crate::utils::error::http_error::HttpError;

#[derive(Clone)]
pub struct RankedHttpApi {
    client: HttpClient,
}

impl RankedHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 修正：获取当前排位信息
    pub async fn get_current_ranked_stats(&self) -> Result<RankedStats, HttpError> {
        let url = "/lol-ranked/v1/current-ranked-stats";
        self.client.get(url).await
    }

    /// 修正：获取 puuid 的排位信息
    pub async fn get_ranked_stats(&self, puuid: &str) -> Result<RankedStats, HttpError> {
        let url = format!("/lol-ranked/v1/ranked-stats/{}", puuid);
        self.client.get(&url).await
    }

    pub async fn acknowledge_eos_notification(&self, id: &str) -> Result<(), HttpError> {
        let url = format!("/lol-ranked/v1/eos-notifications/{}/acknowledge", id);
        self.client.post(&url, None::<&()>).await
    }

    pub async fn acknowledge_notification(&self, id: &str) -> Result<(), HttpError> {
        let url = format!("/lol-ranked/v1/notifications/{}/acknowledge", id);
        self.client.post(&url, None::<&()>).await
    }
}
