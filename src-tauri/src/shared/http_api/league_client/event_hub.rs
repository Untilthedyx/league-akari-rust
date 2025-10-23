use crate::shared::{
    http_api::league_client::httpclient::HttpClient,
    types::league_client::event_hub::*, // 导入事件相关数据结构
};
use crate::utils::error::http_error::HttpError;
use serde::Serialize;
use tracing::instrument;

/// 赛事活动中心 HTTP API 封装
/// 提供与游戏内赛事活动相关的各种接口，包括活动信息、进度、奖励等查询
#[derive(Debug, Clone)]
pub struct EventHubHttpApi {
    /// 内部使用的 HTTP 客户端
    client: HttpClient,
}

impl EventHubHttpApi {
    /// 创建 EventHubHttpApi 实例
    /// - 参数: `client` - 预配置的 HttpClient 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取所有赛事活动列表
    #[instrument(skip_all)]
    pub async fn get_events(&self) -> Result<Vec<EventHubEvents>, HttpError> {
        self.client.get("/lol-event-hub/v1/events").await
    }

    /// 获取指定活动的章节信息
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_chapters(&self, event_id: &str) -> Result<EventChapters, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/chapters", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的详细数据
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_event_details_data(&self, event_id: &str) -> Result<EventDetailsData, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/event-details-data", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的基本信息
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_info(&self, event_id: &str) -> Result<EventInfo, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/info", event_id);
        self.client.get(&url).await
    }

    /// 检查指定活动是否处于宽限期
    /// - 参数: `event_id` - 活动唯一标识符
    /// - 返回: 布尔值（true 表示处于宽限期）
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_is_grace_period(&self, event_id: &str) -> Result<bool, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/is-grace-period", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的叙事内容（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_narrative(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/narrative", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的目标横幅数据
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_objectives_banner(&self, event_id: &str) -> Result<EventObjectivesBanner, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/objectives-banner", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的通行证背景数据（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_pass_background_data(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/pass-background-data", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的通行证捆绑包信息
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_pass_bundles(&self, event_id: &str) -> Result<EventPassBundle2, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/pass-bundles", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的进度信息数据
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_progress_info_data(&self, event_id: &str) -> Result<EventProgressInfoData, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/progress-info-data", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的进度购买数据
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_progression_purchase_data(&self, event_id: &str) -> Result<EventProgressionPurchaseData, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/progression-purchase-data", event_id);
        self.client.get(&url).await
    }

    /// 购买指定活动的优惠
    /// - 参数:
    ///   - `event_id` - 活动唯一标识符
    ///   - `data` - 购买请求数据（具体结构取决于接口要求）
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn post_purchase_offer<T: Serialize>(&self, event_id: &str, data: &T) -> Result<(), HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/purchase-offer", event_id);
        self.client.post(&url, Some(data)).await
    }

    /// 获取指定活动奖励轨道的额外奖励项
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_bonus_items(&self, event_id: &str) -> Result<Vec<EventRewardTrackBonusItem>, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/bonus-items", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的额外进度
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_bonus_progress(&self, event_id: &str) -> Result<EventRewardTrackBonusProgress, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/bonus-progress", event_id);
        self.client.get(&url).await
    }

    /// 一键领取指定活动奖励轨道的所有奖励
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn post_reward_track_claim_all(&self, event_id: &str) -> Result<(), HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/claim-all", event_id);
        self.client.post(&url, None::<&()>).await
    }

    /// 获取指定活动奖励轨道在指定时间前的计数器值
    /// - 参数:
    ///   - `event_id` - 活动唯一标识符
    ///   - `before_epoch` - 时间戳（纪元秒数）
    #[instrument(skip_all, fields(event_id = %event_id, before_epoch = before_epoch))]
    pub async fn get_reward_track_counter(&self, event_id: &str, before_epoch: u64) -> Result<u64, HttpError> {
        let url = format!(
            "/lol-event-hub/v1/events/{}/reward-track/counter?beforeEpoch={}",
            event_id, before_epoch
        );
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的失败信息（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_failure(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/failure", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的奖励项列表
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_items(&self, event_id: &str) -> Result<Vec<EventRewardTrackItem>, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/items", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的进度信息（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_progress(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/progress", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的未领取奖励
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_unclaimed_rewards(&self, event_id: &str) -> Result<EventRewardTrackUnclaimedRewards, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/unclaimed-rewards", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动奖励轨道的经验值信息
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_reward_track_xp(&self, event_id: &str) -> Result<EventRewardTrackXP, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/reward-track/xp", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的代币商店信息（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_token_shop(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/token-shop", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的代币商店分类与优惠（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_token_shop_categories_offers(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/token-shop/categories-offers", event_id);
        self.client.get(&url).await
    }

    /// 获取指定活动的代币余额（暂未指定具体类型）
    /// - 参数: `event_id` - 活动唯一标识符
    #[instrument(skip_all, fields(event_id = %event_id))]
    pub async fn get_token_shop_token_balance(&self, event_id: &str) -> Result<serde_json::Value, HttpError> {
        let url = format!("/lol-event-hub/v1/events/{}/token-shop/token-balance", event_id);
        self.client.get(&url).await
    }

    /// 获取导航按钮数据
    #[instrument(skip_all)]
    pub async fn get_navigation_button_data(&self) -> Result<EventNarrativeButtonData, HttpError> {
        self.client.get("/lol-event-hub/v1/navigation-button-data").await
    }

    /// 购买物品
    /// - 参数: `data` - 购买请求数据（具体结构取决于接口要求）
    #[instrument(skip_all)]
    pub async fn post_purchase_item<T: Serialize>(&self, data: &T) -> Result<(), HttpError> {
        self.client.post("/lol-event-hub/v1/purchase-item", Some(data)).await
    }

    /// 获取皮肤相关信息（暂未指定具体类型）
    #[instrument(skip_all)]
    pub async fn get_skins(&self) -> Result<serde_json::Value, HttpError> {
        self.client.get("/lol-event-hub/v1/skins").await
    }

    /// 获取代币升级销售信息（暂未指定具体类型）
    #[instrument(skip_all)]
    pub async fn get_token_upsell(&self) -> Result<Vec<serde_json::Value>, HttpError> {
        self.client.get("/lol-event-hub/v1/token-upsell").await
    }
}