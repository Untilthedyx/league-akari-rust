use crate::shared::http_api::league_client::httpclient::HttpClient;
use crate::shared::types::league_client::rewards::*;
use crate::utils::error::http_error::HttpError;
use urlencoding::encode;

/// 对应 TypeScript 的 PostGrantSelectionDto 接口

/// 奖励相关的 HTTP API 客户端
pub struct RewardsHttpApi {
    client: HttpClient,
}

impl RewardsHttpApi {
    /// 创建新的 RewardsHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 发送庆祝相关的 FSC 数据
    pub async fn post_celebrations_fsc(
        &self,
        data: &PostCelebrationsFscDto,
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-rewards/v1/celebrations/fsc";
        self.client.post(url, Some(data)).await
    }

    /// 获取奖励授予列表（支持按状态筛选）
    pub async fn get_grants(&self, status: Option<&str>) -> Result<Vec<RewardsGrant>, HttpError> {
        let url = match status {
            Some(s) => format!("/lol-rewards/v1/grants?status={}", encode(s)),
            None => "/lol-rewards/v1/grants".to_string(), // 简化格式
        };
        self.client.get(&url).await
    }

    /// 更新奖励授予的查看状态
    pub async fn patch_grants_view(
        &self,
        data: &serde_json::Value, // 临时使用 serde_json::Value 处理未知结构的 data
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-rewards/v1/grants/view";
        self.client.patch(url, Some(data)).await
    }

    /// 提交奖励授予的选择
    pub async fn post_grant_selection(
        &self,
        grant_id: &str,
        data: &PostGrantSelectionDto,
    ) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-rewards/v1/grants/{}/select", grant_id);
        self.client.post(&url, Some(data)).await
    }

    /// 获取奖励组列表（支持按类型筛选）
    pub async fn get_groups(
        &self,
        types: Option<&[&str]>, // 可选的类型数组
    ) -> Result<Vec<RewardsGroup>, HttpError> {
        let base_url = "/lol-rewards/v1/groups";

        let query = types.map_or(String::new(), |ts| {
            ts.iter()
                .map(|t| format!("types={}", encode(t)))
                .collect::<Vec<_>>()
                .join("&")
        });

        let url = if query.is_empty() {
            base_url.to_string()
        } else {
            format!("{}?{}", base_url, query)
        };

        self.client.get(&url).await
    }

    /// 提交奖励回放数据
    pub async fn post_reward_replay(
        &self,
        reward_group_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-rewards/v1/reward/replay";
        let data = serde_json::json!({ "reward_group_id": reward_group_id });
        self.client.post(url, Some(&data)).await
    }

    /// 批量提交选择
    pub async fn post_select_bulk(
        &self,
        selections: &[String],
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-rewards/v1/select-bulk";
        let data = serde_json::json!({ "selection": selections });
        self.client.post(url, Some(&data)).await
    }
}
