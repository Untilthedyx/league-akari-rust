use crate::shared::http_api::http::HttpClient;
use crate::utils::error::http_error::HttpError;
// 假设奖励轨道相关类型定义在以下路径（根据实际情况调整）

pub struct RewardTrackHttpApi {
    client: HttpClient,
}

impl RewardTrackHttpApi {
    /// 创建 RewardTrackHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取注册信息
    pub async fn get_register(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-reward-track/register/{}", progression_group_id);
        self.client.get(&url).await
    }

    /// 获取奖励轨道的额外物品
    pub async fn get_bonus_items(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/bonus-items",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取额外进度信息
    pub async fn get_bonus_progress(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/bonus-progress",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取失败信息
    pub async fn get_failure(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/failure",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取奖励轨道物品
    pub async fn get_items(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/items",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取进度信息
    pub async fn get_progress(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/progress",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取未领取的奖励
    pub async fn get_unclaimed_rewards(
        &self,
        progression_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!(
            "/lol-reward-track/{}/reward-track/unclaimed-rewards",
            progression_group_id
        );
        self.client.get(&url).await
    }

    /// 获取经验值信息
    pub async fn get_xp(&self, progression_group_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-reward-track/{}/reward-track/xp", progression_group_id);
        self.client.get(&url).await
    }
}
