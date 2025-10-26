use crate::shared::http_api::http::HttpClient;
use crate::utils::error::http_error::HttpError;

pub struct EndOfGameHttpApi {
    client: HttpClient,
}

impl EndOfGameHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn dismiss_stats(&self) -> Result<(), HttpError> {
        let uri = "/lol-end-of-game/v1/state/dismiss-stats";
        self.client.post(uri, None::<&()>).await
    }
}
