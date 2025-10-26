use crate::shared::http_api::http::HttpClient;
use crate::shared::types::league_client::replays::*;
use crate::utils::error::http_error::HttpError;

pub struct ReplaysHttpApi {
    client: HttpClient,
}

impl ReplaysHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_metadata(&self, game_id: &String) -> Result<ReplayMetadata, HttpError> {
        let url = format!("/lol-replays/v1/metadata/{}", game_id);
        self.client.get(&url).await
    }

    pub async fn watch_rofl(&self, game_id: &String) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-replays/v1/rofls/{}/watch", game_id);
        self.client
            .post(
                &url,
                Some(&serde_json::json!({"componentType": "replay-button_match-history"})),
            )
            .await
    }

    pub async fn down_rofl(&self, game_id: &String) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-replays/v1/rofls/{}/download", game_id);
        self.client
            .post(
                &url,
                Some(&serde_json::json!({"componentType": "replay-button_match-history"})),
            )
            .await
    }

    pub async fn create_metadata(
        &self,
        game_id: &String,
        data: &RequestMetadata,
    ) -> Result<(), HttpError> {
        let url = format!("/lol-replays/v2/metadata/{}/create", game_id);
        self.client.post(&url, Some(&data)).await
    }

    pub async fn get_configuration(&self) -> Result<ReplayConfiguration, HttpError> {
        let url = "/lol-replays/v1/configuration";
        self.client.get(url).await
    }

    pub async fn get_replays_path(&self) -> Result<String, HttpError> {
        let url = "/lol-replays/v1/rofls/path";
        self.client.get(url).await
    }
}
