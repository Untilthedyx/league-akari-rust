use crate::{
    shared::http_api::lcu::http::HttpClient,
    shared::types::league_client::loot::{LootCraftResponse, LootMap},
    utils::error::http_error::HttpError,
};

#[derive(Clone)]
pub struct LootHttpApi {
    client: HttpClient,
}

impl LootHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取战利品映射表
    pub async fn get_loot_map(&self) -> Result<LootMap, HttpError> {
        let url = "/lol-loot/v1/player-loot-map";
        self.client.get(url).await
    }

    /// 制作/开启战利品
    pub async fn craft_loot(
        &self,
        loot: &str,
        repeat: Option<u32>,
    ) -> Result<LootCraftResponse, HttpError> {
        // 构建带查询参数的 URL（repeat 默认为 1）
        let repeat = repeat.unwrap_or(1);
        let url = format!("/lol-loot/v1/recipes/{}/craft?repeat={}", loot, repeat);
        self.client.post(&url, None::<&()>).await
    }
}
