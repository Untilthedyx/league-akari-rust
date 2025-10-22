use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// 事件中心事件结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventHubEvents {
    /// 事件唯一标识
    pub event_id: String,
    /// 事件详细信息
    pub event_info: EventInfo,
}

/// 事件详细信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventInfo {
    /// 当前令牌余额
    pub current_token_balance: i64,
    /// 事件结束日期（字符串格式）
    pub end_date: String,
    /// 事件图标路径
    pub event_icon: String,
    /// 事件唯一标识
    pub event_id: String,
    /// 事件名称
    pub event_name: String,
    /// 事件通行证捆绑包列表
    pub event_pass_bundles: Vec<EventPassBundle>,
    /// 事件令牌图片路径
    pub event_token_image: String,
    /// 事件类型
    pub event_type: String,
    /// 是否已购买通行证
    pub is_pass_purchased: bool,
    /// 锁定的令牌数量
    pub locked_token_count: i64,
    /// 导航栏图标路径
    pub nav_bar_icon: String,
    /// 进度结束日期
    pub progress_end_date: String,
    /// 事件开始日期
    pub start_date: String,
    /// 最后一次未领取奖励的时间戳
    pub time_of_last_unclaimed_reward: i64,
    /// 令牌捆绑包列表（任意类型）
    pub token_bundles: Vec<JsonValue>,
    /// 未领取的奖励数量
    pub unclaimed_reward_count: i64,
}

/// 事件通行证捆绑包
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventPassBundle {
    /// 内容ID
    pub content_id: String,
    /// 物品ID
    pub item_id: i64,
    /// 优惠ID
    pub offer_id: String,
    /// 类型ID
    pub type_id: String,
}

/// 事件章节集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventChapters {
    /// 章节列表
    pub chapters: Vec<Chapter>,
    /// 当前章节索引
    pub current_chapter: i64,
}

/// 事件章节详情
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Chapter {
    /// 背景图片路径
    pub background_image: String,
    /// 背景视频路径
    pub background_video: String,
    /// 卡片图片路径
    pub card_image: String,
    /// 章节结束时间戳
    pub chapter_end: i64,
    /// 章节编号
    pub chapter_number: i64,
    /// 章节开始时间戳
    pub chapter_start: i64,
    /// 前景图片路径
    pub foreground_image: String,
    /// 聚焦等级
    pub level_focus: i64,
    /// 本地化描述
    pub localized_description: String,
    /// 本地化标题
    pub localized_title: String,
    /// 目标横幅图片路径
    pub objective_banner_image: String,
}

/// 事件详情数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventDetailsData {
    /// 事件图标路径
    pub event_icon_path: String,
    /// 事件名称
    pub event_name: String,
    /// 事件开始日期
    pub event_start_date: String,
    /// 标题图片路径
    pub header_title_image_path: String,
    /// 帮助模态框图片路径
    pub help_modal_image_path: String,
    /// 入选者名称
    pub inductee_name: String,
    /// 进度结束日期
    pub progress_end_date: String,
    /// 推广横幅图片路径
    pub promotion_banner_image: String,
    /// 商店结束日期
    pub shop_end_date: String,
}

/// 事件目标横幅数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventObjectivesBanner {
    /// 当前章节信息
    pub current_chapter: CurrentChapter,
    /// 事件名称
    pub event_name: String,
    /// 轨道进度信息
    pub track_progress: TrackProgress,
}

/// 轨道进度信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TrackProgress {
    /// 当前等级
    pub current_level: i64,
    /// 当前经验值
    pub current_xp: i64,
    /// 下一级所需经验值
    pub next_level_xp: i64,
    /// 下一个奖励信息
    pub next_reward: NextReward,
}

/// 下一个奖励信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextReward {
    /// 奖励描述
    pub description: String,
    /// 奖励等级
    pub level: String,
    /// 奖励名称
    pub name: String,
    /// 奖励状态
    pub state: String,
    /// 缩略图标路径
    pub thumb_icon_path: String,
}

/// 当前章节信息（与 Chapter 结构一致，复用定义）
pub type CurrentChapter = Chapter;

/// 事件奖励轨道未领取奖励信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventRewardTrackUnclaimedRewards {
    /// 锁定的令牌数量
    pub locked_tokens_count: i64,
    /// 奖励总数
    pub rewards_count: i64,
    /// 最后一次未领取奖励的时间戳
    pub time_of_last_unclaimed_reward: i64,
}

/// 事件奖励轨道物品
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventRewardTrackItem {
    /// 所需进度
    pub progress_required: i64,
    /// 奖励选项列表
    pub reward_options: Vec<RewardOption>,
    /// 奖励标签（任意类型）
    pub reward_tags: Vec<JsonValue>,
    /// 状态
    pub state: String,
    /// 阈值
    pub threshold: String,
}

/// 奖励选项
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardOption {
    /// 卡片尺寸
    pub card_size: String,
    /// 庆祝类型
    pub celebration_type: String,
    /// 头部类型
    pub header_type: String,
    /// 覆盖底部文本
    pub override_footer: String,
    /// 奖励描述
    pub reward_description: String,
    /// 奖励组ID
    pub reward_group_id: String,
    /// 奖励名称
    pub reward_name: String,
    /// 是否已选择
    pub selected: bool,
    ///  splash 图片路径
    pub splash_image_path: String,
    /// 状态
    pub state: String,
    /// 缩略图标路径
    pub thumb_icon_path: String,
}

/// 事件奖励轨道额外物品（与 EventRewardTrackItem 结构一致，复用定义）
pub type EventRewardTrackBonusItem = EventRewardTrackItem;

/// 事件通行证捆绑包2（扩展定义）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventPassBundle2 {
    /// 捆绑物品列表
    pub bundled_items: Vec<BundledItem>,
    /// 详情物品
    pub details: BundledItem,
    /// 折扣百分比
    pub discount_percentage: i64,
    /// 最终价格
    pub final_price: f64,
    /// 未来余额
    pub future_balance: i64,
    /// 初始价格
    pub initial_price: f64,
    /// 是否可购买
    pub is_purchasable: bool,
}

/// 捆绑物品信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BundledItem {
    /// 装饰徽章URL
    pub decorator_badge_url: String,
    /// 描述
    pub description: String,
    /// 库存类型
    pub inventory_type: String,
    /// 物品ID
    pub item_id: i64,
    /// 名称
    pub name: String,
    /// 是否已拥有
    pub owned: bool,
    /// 数量
    pub quantity: i64,
    /// splash 图片路径
    pub splash_image: String,
    /// 子库存类型
    pub sub_inventory_type: String,
}

/// 事件进度信息数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventProgressInfoData {
    /// 事件通行证捆绑包目录条目
    pub event_pass_bundles_catalog_entry: Vec<EventPassBundlesCatalogEntry>,
    /// 是否已购买通行证
    pub pass_purchased: bool,
    /// 令牌图片路径
    pub token_image: String,
}

/// 事件通行证捆绑包目录条目
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventPassBundlesCatalogEntry {
    /// 内容ID
    pub content_id: String,
    /// 物品ID
    pub item_id: i64,
    /// 优惠ID
    pub offer_id: String,
    /// 类型ID
    pub type_id: String,
}

/// 事件奖励轨道额外进度
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventRewardTrackBonusProgress {
    /// 当前等级经验值
    pub current_level_xp: i64,
    /// 未来等级进度
    pub future_level_progress: i64,
    /// 迭代次数
    pub iteration: i64,
    /// 当前等级
    pub level: i64,
    /// 等级进度
    pub level_progress: i64,
    /// 通行证进度
    pub pass_progress: i64,
    /// 总等级经验值
    pub total_level_xp: i64,
    /// 总等级数
    pub total_levels: i64,
}

/// 事件进度购买数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventProgressionPurchaseData {
    /// 优惠ID
    pub offer_id: String,
    /// 每级价格
    pub price_per_level: f64,
    /// RP余额
    pub rp_balance: i64,
}

/// 事件奖励轨道经验值
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventRewardTrackXP {
    /// 当前等级
    pub current_level: i64,
    /// 当前等级经验值
    pub current_level_xp: i64,
    /// 是否为额外阶段
    pub is_bonus_phase: bool,
    /// 迭代次数
    pub iteration: i64,
    /// 总等级经验值
    pub total_level_xp: i64,
}

/// 事件叙事按钮数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EventNarrativeButtonData {
    /// 活跃事件ID
    pub active_event_id: String,
    /// 事件名称
    pub event_name: String,
    /// 图标路径
    pub icon_path: String,
    /// 是否显示发光效果
    pub show_glow: bool,
    /// 是否显示小标记
    pub show_pip: bool,
}