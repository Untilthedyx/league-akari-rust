use crate::{
    shared::http_api::league_client::httpclient::HttpClient, shared::types::league_client::honor::*,
    utils::error::http_error::HttpError,
};

pub struct HonorHttpApi {
    client: HttpClient,
}

impl HonorHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn ballot(&self) -> Result<(), HttpError> {
        let url = "/lol-honor/v1/ballot";
        self.client.post(url, None::<&()>).await
    }

    pub async fn honor(&self, honor_type: &str, recipient_puuid: &str) -> Result<(), HttpError> {
        let url = "/lol-honor/v1/honor";
        let body = HonorRequest {
            honor_type: honor_type.to_string(),
            recipient_puuid: recipient_puuid.to_string(),
        };
        self.client.post(url, Some(&body)).await
    }

    pub async fn v2_honor(
        &self,
        game_id: impl ToString,
        honor_category: HonorCategory,
        summoner_id: Option<impl ToString>,
        puuid: Option<impl ToString>,
    ) -> Result<(), HttpError> {
        let url = "/lol-honor-v2/v1/honor-player/";
        let body = V2HonorRequest {
            game_id: game_id.to_string(),
            honor_category,
            summoner_id: summoner_id.map(|v| v.to_string()),
            puuid: puuid.map(|v| v.to_string()),
        };
        self.client.post(url, Some(&body)).await
    }

    pub async fn get_v2_ballot(&self) -> Result<Ballot, HttpError> {
        let url = "/lol-honor-v2/v1/ballot/";
        self.client.get(url).await
    }

    pub async fn ack_honor_notification(&self, mail_id: &str) -> Result<(), HttpError> {
        let url = format!("/lol-honor-v2/v1/ack-honor-notification/{}", mail_id);
        self.client.post(&url, None::<&()>).await
    }

    pub async fn ack_late_recognition(&self) -> Result<(), HttpError> {
        let url = "/lol-honor-v2/v1/late-recognition/ack";
        self.client.post(url, None::<&()>).await
    }

    pub async fn ack_level_change(&self) -> Result<(), HttpError> {
        let url = "/lol-honor-v2/v1/level-change/ack";
        self.client.post(url, None::<&()>).await
    }

    pub async fn ack_mutual_honor(&self) -> Result<(), HttpError> {
        let url = "/lol-honor-v2/v1/mutual-honor/ack";
        self.client.post(url, None::<&()>).await
    }

    pub async fn ack_reward_granted(&self) -> Result<(), HttpError> {
        let url = "/lol-honor-v2/v1/reward-granted/ack";
        self.client.post(url, None::<&()>).await
    }
}
