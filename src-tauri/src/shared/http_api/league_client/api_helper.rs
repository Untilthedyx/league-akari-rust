/// 似乎没有必要

use crate::shared::http_api::league_client::httpclient::{HttpClient, HttpData};
use crate::utils::error::http_error::HttpError;
use async_trait::async_trait;
use tracing::error;

#[async_trait]
pub trait ApiHelper {
    fn client(&self) -> &HttpClient;

    async fn get<R: HttpData>(&self, url: &str, context: &str) -> Result<R, HttpError> {
        match self.client().get(url).await {
            Ok(res) => Ok(res),
            Err(err) => {
                error!("Failed to get {}: {}", context, err);
                Err(err)
            }
        }
    }
}