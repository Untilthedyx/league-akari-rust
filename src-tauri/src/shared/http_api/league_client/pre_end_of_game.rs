use crate::{
    shared::http_api::http::HttpClient, utils::error::http_error::HttpError,
};

pub struct PreEndOfGameHttpApi {
    client: HttpClient,
}

impl PreEndOfGameHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn complete(&self, sequence_event_name: &String) -> Result<(), HttpError> {
        let url = format!("/lol-pre-end-of-game/v1/complete/{}", sequence_event_name);
        self.client.post(&url, None::<&()>).await
    }

    pub async fn get_current_sequence_event(&self) -> Result<(), HttpError> {
        let url = "/lol-pre-end-of-game/v1/currentSequenceEvent";
        self.client.get(url).await
    }
}
