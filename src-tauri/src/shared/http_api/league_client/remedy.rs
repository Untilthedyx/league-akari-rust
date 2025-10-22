use crate::shared::http_api::league_client::httpclient::HttpClient;
use crate::utils::error::http_error::HttpError;

pub struct RemedyHttpApi {
    client: HttpClient,
}

impl RemedyHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn ack_remedy_notification(
        &self,
        mail_id: &String,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-remedy/v1/ack-remedy-notification/{}", mail_id);
        self.client.put(&url, None::<&()>).await
    }

    pub async fn get_notifications(&self) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-remedy/v1/remedy-notifications";
        self.client.get(url).await
    }

    pub async fn get_verbal_abuse_remedy_model_enabled(
        &self,
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-remedy/v1/config/is-verbal-abuse-remedy-modal-enabled";
        self.client.get(url).await
    }
}
