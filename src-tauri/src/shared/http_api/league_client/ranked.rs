use crate::shared::http_api::league_client::httpclient::HttpClient;
use crate::shared::types::ranked::*;
use crate::utils::error::http_error::HttpError;

pub struct RankedHttpApi {
    client: HttpClient,
}

impl RankedHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_current_ranked_stats(&self) -> Result<RankedStats, HttpError> {
        let url = "/lol-ranked/v1/current-ranked-stats";
        self.client.get(url).await
    }

    pub async fn get_ranked_stats(&self, puuid: &String) -> Result<RankedStats, HttpError> {
        let url = format!("/lol-ranked/v1/ranked-stats/{}", puuid);
        self.client.get(&url).await
    }

    pub async fn acknowledge_eos_notification(&self, id: &String) -> Result<(), HttpError> {
        let url = format!("/lol-ranked/v1/eos-notifications/{}/acknowledge", id);
        self.client.post(&url, None::<&()>).await
    }

    pub async fn acknowledge_notification(&self, id: &String) -> Result<(), HttpError> {
        let url = format!("/lol-ranked/v1/notifications/{}/acknowledge", id);
        self.client.post(&url, None::<&()>).await
    }
}
