use crate::{
    shared::http_api::league_client::httpclient::HttpClient, shared::types::matchmaking::GetSearch,
    utils::error::http_error::HttpError,
};

pub struct MatchmakingHttpApi {
    client: HttpClient,
}

impl MatchmakingHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 接受就绪检查
    pub async fn accept(&self) -> Result<(), HttpError> {
        let url = "/lol-matchmaking/v1/ready-check/accept";
        self.client.post(url, None::<&()>).await
    }

    /// 拒绝就绪检查
    pub async fn decline(&self) -> Result<(), HttpError> {
        let url = "/lol-matchmaking/v1/ready-check/decline";
        self.client.post(url, None::<&()>).await
    }

    /// 获取当前匹配搜索状态
    pub async fn get_search(&self) -> Result<GetSearch, HttpError> {
        let url = "/lol-matchmaking/v1/search";
        self.client.get(url).await
    }
}
