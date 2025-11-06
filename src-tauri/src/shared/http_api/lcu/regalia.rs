use crate::shared::http_api::lcu::http::HttpClient;
use crate::utils::error::http_error::HttpError;

#[derive(Clone)]
pub struct RegaliaHttpApi {
    client: HttpClient,
}

impl RegaliaHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn update_regalia(&self, dto: serde_json::Value) -> Result<(), HttpError> {
        let url = "/lol-regalia/v2/current-summoner/regalia";
        self.client.put(url, Some(&dto)).await
    }

    pub async fn get_regalia(&self) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-regalia/v2/current-summoner/regalia";
        self.client.get(url).await
    }
}
