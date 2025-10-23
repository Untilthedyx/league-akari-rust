use serde::{Deserialize, Serialize};

/// 对应 TypeScript 的 PlayerNotifications 接口
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotifications {
    /// 背景图片 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_url: Option<String>,
    /// 创建时间
    pub created: Option<String>,
    /// 是否为 critical 级别通知
    pub critical: Option<bool>,
    /// 附加数据（任意类型，对应 TypeScript 的 any）
    pub data: Option<NotificationData>,
    /// i18n 详情键（如 pre_translated_details）
    pub detail_key: Option<String>,
    /// 是否可关闭
    pub dismissible: Option<bool>,
    /// 过期时间
    pub expires: Option<String>,
    /// 图标 URL
    pub icon_url: Option<String>,
    /// 通知 ID（POST 时会自动生成）
    pub id: Option<u64>, // 用 u64 而非 i32 更符合 ID 场景，避免负数
    /// 通知来源
    pub source: Option<String>,
    /// 通知状态
    pub state: Option<String>,
    /// i18n 标题键
    pub title_key: Option<String>,
    /// 通知类型
    pub r#type: Option<String>, // 用 r# 转义避免与 Rust 关键字 type 冲突
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NotificationData {
    pub details: String,
    pub title: String,
}
