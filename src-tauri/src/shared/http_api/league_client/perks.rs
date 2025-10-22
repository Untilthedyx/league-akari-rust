use crate::{
    shared::http_api::league_client::httpclient::HttpClient, shared::types::perks::*,
    utils::error::http_error::HttpError,
};

pub struct PerksHttpApi {
    client: HttpClient,
}

impl PerksHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 创建新符文页
    pub async fn post_perk_page(&self, perk_data: &PostPerkDto) -> Result<PerkPage, HttpError> {
        let url = "/lol-perks/v1/pages/";
        self.client.post(url, Some(perk_data)).await
    }

    /// 获取符文页库存信息
    pub async fn get_perk_inventory(&self) -> Result<PerkInventory, HttpError> {
        let url = "/lol-perks/v1/inventory";
        self.client.get(url).await
    }

    /// 获取所有符文页
    pub async fn get_perk_pages(&self) -> Result<Vec<PerkPage>, HttpError> {
        let url = "/lol-perks/v1/pages";
        self.client.get(url).await
    }

    /// 更新符文页
    pub async fn put_page(&self, perk_data: &PutPageDto) -> Result<(), HttpError> {
        let url = format!("/lol-perks/v1/pages/{}", perk_data.id);
        self.client.put(&url, Some(perk_data)).await
    }

    /// 设置当前使用的符文页
    pub async fn put_current_page(&self, id: i32) -> Result<(), HttpError> {
        let url = "/lol-perks/v1/currentpage";
        self.client.put(url, Some(&id)).await
    }

    /// 获取推荐的英雄位置
    pub async fn get_recommended_champion_positions(
        &self,
    ) -> Result<RecommendPositions, HttpError> {
        let url = "/lol-perks/v1/recommended-champion-positions";
        self.client.get(url).await
    }

    /// 获取指定英雄的推荐位置符文页
    pub async fn get_recommended_pages_position(&self, champion_id: i32) -> Result<(), HttpError> {
        let url = format!(
            "/lol-perks/v1/recommended-pages-position/champion/{}",
            champion_id
        );
        self.client.get(&url).await
    }

    /// 提交指定英雄的位置推荐
    pub async fn post_recommended_page_position(
        &self,
        champion_id: i32,
        position: &str,
    ) -> Result<(), HttpError> {
        let url = format!(
            "/lol-perks/v1/recommended-pages-position/champion/{}/position/{}",
            champion_id, position
        );
        self.client.post(&url, None::<&()>).await
    }

    /// 获取指定英雄、位置和地图的推荐符文页
    pub async fn get_recommended_pages(
        &self,
        champion_id: i32,
        position: &str,
        map_id: i32,
    ) -> Result<Vec<RecommendPage>, HttpError> {
        let url = format!(
            "/lol-perks/v1/recommended-pages/champion/{}/position/{}/map/{}",
            champion_id, position, map_id
        );
        self.client.get(&url).await
    }

    /// 检查是否开启系统自动选择符文
    pub async fn get_rune_recommender_auto_select(&self) -> Result<bool, HttpError> {
        let url = "/lol-perks/v1/rune-recommender-auto-select";
        self.client.get(url).await
    }

    /// 开启系统自动选择符文
    pub async fn post_rune_recommender_auto_select(
        &self,
        data: &serde_json::Value,
    ) -> Result<(), HttpError> {
        let url = "/lol-perks/v1/rune-recommender-auto-select";
        self.client.post(url, Some(data)).await
    }
}
