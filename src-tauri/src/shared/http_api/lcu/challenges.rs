use crate::{
    shared::http_api::http::HttpClient, utils::error::http_error::HttpError,
};

#[derive(Clone)]
pub struct ChallengesHttpApi {
    client: HttpClient,
}

impl ChallengesHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn update_player_preference(
        &self,
        config: &serde_json::Value,
    ) -> Result<(), HttpError> {
        let url = "/lol-challenges/v1/update-player-preferences/";
        self.client.post(&url, Some(&config)).await
    }

    pub async fn ack_challenge_update(&self, id: u32) -> Result<(), HttpError> {
        let url = format!("/lol-challenges/v1/ack-challenge-update/{}", id);
        self.client.post(&url, None::<&()>).await
    }
}
