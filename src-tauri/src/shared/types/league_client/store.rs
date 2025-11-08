use serde::{Deserialize, Serialize};

/// 可赠送礼物的好友信息接口（对应 TypeScript 的 GiftableFriend）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GiftableFriend {
    /// 成为好友的时间（字符串格式，通常为 ISO 时间戳）
    pub friends_since: String,
    /// 好友昵称
    pub nick: String,
    /// 是否为老好友（可能表示长期好友关系）
    pub old_friends: bool,
    /// 召唤师 ID（游戏内唯一标识）
    pub summoner_id: u64, // 召唤师 ID 通常为大整数，用 u64 更合适
}
