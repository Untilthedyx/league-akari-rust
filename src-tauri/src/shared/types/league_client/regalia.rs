use serde::{Deserialize, Serialize};

/// 对应 TypeScript 的 Regalia 接口
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Regalia {
    /// 横幅类型（对应 TypeScript 的 bannerType）
    pub banner_type: String,
    /// 徽章类型（对应 TypeScript 的 crestType）
    pub crest_type: String,
    /// 最高排名记录（固定为 null，用 Option 表示）
    pub highest_ranked_entry: Option<()>,
    /// 上赛季最高排名（固定为 null，用 Option 表示）
    pub last_season_highest_rank: Option<()>,
    /// 首选横幅类型（对应 TypeScript 的 preferredBannerType）
    pub preferred_banner_type: String,
    /// 首选徽章类型（对应 TypeScript 的 preferredCrestType）
    pub preferred_crest_type: String,
    /// 头像图标 ID（对应 TypeScript 的 profileIconId）
    pub profile_icon_id: u32,
    /// 选中的 prestige 徽章（对应 TypeScript 的 selectedPrestigeCrest）
    pub selected_prestige_crest: u32,
    /// 召唤师等级（对应 TypeScript 的 summonerLevel）
    pub summoner_level: u32,
}
