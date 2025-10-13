/// https://riot:6YY78o1__fXs6enCsdleeQ@127.0.0.1:49991/lol-ranked/v1/ranked-stats/55cc79c4-3d20-535a-9bff-00b1867534d8
use crate::lcu::constant::{get_queue_type_to_cn, get_tier_en_to_cn};
use crate::lcu::https::lcu_get;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")] // Apply camelCase deserialization to all fields
pub struct QueueInfo {
    #[serde(default)]
    pub queue_type_cn: String, // QueueType 表示队列类型，例如 "RANKED_SOLO_5x5"。
    pub queue_type: String,

    #[serde(default)]
    pub tier_cn: String,
    pub tier: String,

    pub division: String, // Division 表示玩家当前段位的分段，例如 "I"、"II"。
    pub highest_division: String, // HighestDivision 表示玩家历史最高的分段。
    pub highest_tier: String, // HighestTier 表示玩家历史最高的段位，例如 "Diamond"、"Master"。
    pub is_provisional: bool, // IsProvisional 表示该队列是否处于定级赛阶段。
    pub league_points: i32, // LeaguePoints 表示玩家当前的段位点数（LP）。
    pub losses: i32,      // Losses 表示玩家在该队列的失败场次。
    pub wins: i32,        // Wins 表示玩家在该队列的胜利场次。
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct QueueMap {
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: QueueInfo,
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: QueueInfo,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub queue_map: QueueMap,
}

impl Rank {
    pub async fn get_rank_by_puuid(puuid: &str) -> Result<Self, String> {
        let uri = format!("lol-ranked/v1/ranked-stats/{}", puuid);
        let rank: Self = lcu_get(&uri).await?;
        Ok(rank)
    }

    pub fn enrich_cn_info(&mut self) {
        self.queue_map.ranked_solo_5x5.queue_type_cn =
            get_queue_type_to_cn(&self.queue_map.ranked_solo_5x5.queue_type)
                .unwrap_or("其他")
                .to_string();
        self.queue_map.ranked_flex_sr.queue_type_cn =
            get_queue_type_to_cn(&self.queue_map.ranked_flex_sr.queue_type)
                .unwrap_or("其他")
                .to_string();
        self.queue_map.ranked_solo_5x5.tier_cn =
            get_tier_en_to_cn(&self.queue_map.ranked_solo_5x5.tier)
                .unwrap_or("无")
                .to_string();
        self.queue_map.ranked_flex_sr.tier_cn =
            get_tier_en_to_cn(&self.queue_map.ranked_flex_sr.tier)
                .unwrap_or("无")
                .to_string();
    }
}

// Rank {
//     queue_map: QueueMap {
//         ranked_solo_5x5: QueueInfo {
//             queue_type_cn: "单双排",
//             queue_type: "RANKED_SOLO_5x5",
//             tier_cn: "荣耀黄金",
//             tier: "GOLD",
//             division: "IV",
//             highest_division: "III",
//             highest_tier: "GOLD",
//             is_provisional: false,
//             league_points: 79,
//             losses: 31,
//             wins: 32,
//         },
//         ranked_flex_sr: QueueInfo {
//             queue_type_cn: "灵活组排",
//             queue_type: "RANKED_FLEX_SR",
//             tier_cn: "荣耀黄金",
//             tier: "GOLD",
//             division: "IV",
//             highest_division: "II",
//             highest_tier: "GOLD",
//             is_provisional: false,
//             league_points: 0,
//             losses: 195,
//             wins: 185,
//         },
//     },
// }
