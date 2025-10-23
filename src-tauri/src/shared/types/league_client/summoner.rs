use serde::{Deserialize, Serialize};

/// 召唤师基本信息（对应 TypeScript 的 SummonerInfo）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummonerInfo {
    /// 账号 ID
    pub account_id: u64, // 大整数用 u64 更安全

    /// 显示名称
    pub display_name: String,

    /// 旧 ID 系统名称（通常为空）
    pub game_name: String,

    /// 内部名称
    pub internal_name: String,

    /// 是否有改名标记
    pub name_change_flag: bool,

    /// 距离下一级的完成百分比
    pub percent_complete_for_next_level: f64, // 百分比可能为浮点数

    /// 隐私设置（支持 PUBLIC/PRIVATE 及其他扩展字符串）
    pub privacy: Privacy,

    /// 头像图标 ID
    pub profile_icon_id: u32,

    /// 全局唯一标识符
    pub puuid: String,

    /// 重roll 点数信息
    pub reroll_points: RerollPoints,

    /// 标签行（新 ID 系统的标签）
    pub tag_line: String,

    /// 召唤师 ID
    pub summoner_id: u64,

    /// 召唤师等级
    pub summoner_level: u32,

    /// 是否未命名
    pub unnamed: bool,

    /// 当前等级以来获得的 XP
    pub xp_since_last_level: u64,

    /// 距离下一级所需的 XP
    pub xp_until_next_level: u64,
}

/// 隐私设置枚举（对应 TypeScript 的 'PUBLIC' | 'PRIVATE' | (string & {})）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)] // 支持未标记的联合类型解析
pub enum Privacy {
    /// 公开
    #[default]
    Public,
    /// 私有
    Private,
    /// 其他扩展字符串（兼容未来可能的新值）
    Other(String),
}

/// 重roll 点数信息（对应 TypeScript 的 RerollPoints）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RerollPoints {
    /// 当前点数
    pub current_points: u32,

    /// 最大重roll 次数
    pub max_rolls: u32,

    /// 当前重roll 次数
    pub number_of_rolls: u32,

    /// 每次重roll 消耗的点数
    pub points_cost_to_roll: u32,

    /// 重roll 所需的点数
    pub points_to_reroll: u32,
}

/// 召唤师资料信息（对应 TypeScript 的 SummonerProfile）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummonerProfile {
    /// 背景皮肤增强（可能是皮肤特效标识）
    pub background_skin_augments: String,

    /// 背景皮肤 ID
    pub background_skin_id: u32,

    /// 徽章/荣誉标识（对应 Regalia 相关数据）
    pub regalia: String,
}
