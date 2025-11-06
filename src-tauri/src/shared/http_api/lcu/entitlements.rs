use crate::shared::http_api::http::HttpClient;
use crate::shared::types::league_client::entitlements::EntitlementsToken;
use crate::utils::error::http_error::HttpError;

#[derive(Clone)]
pub struct EntitlementsHttpApi {
    client: HttpClient,
}

impl EntitlementsHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_entitlements_token(&self) -> Result<EntitlementsToken, HttpError> {
        let uri = "/entitlements/v1/token";
        self.client.get(uri).await
    }
}
