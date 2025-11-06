use crate::shared::http_api::lcu::http::HttpClient;
use crate::shared::types::league_client::store::GiftableFriend;
use crate::utils::error::http_error::HttpError;

/// Riot 客户端相关的 HTTP API 客户端
#[derive(Clone)]
pub struct StoreHttpApi {
    client: HttpClient,
}

impl StoreHttpApi {
    /// 创建新的 StoreHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_giftabel_friends(&self) -> Result<Vec<GiftableFriend>, HttpError> {
        let url = "/lol-store/v1/giftablefriends";
        self.client.get(url).await
    }
}
