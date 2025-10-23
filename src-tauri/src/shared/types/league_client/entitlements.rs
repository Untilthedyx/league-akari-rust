use serde::{Deserialize, Serialize};

/// 权限令牌接口定义
/// 包含访问令牌、权限列表及发行者等信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsToken {
    /// 访问令牌字符串
    pub access_token: String,

    /// 权限列表（类型为任意 JSON 数据的数组）
    pub entitlements: Vec<serde_json::Value>,

    /// 发行者标识
    pub issuer: String,

    /// 主题标识（通常为用户相关信息）
    pub subject: String,

    /// 权限令牌字符串
    pub token: String,
}
