// checked

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 排名统计信息主结构，对应完整的JSON响应
/// 包含当前赛季、历史赛季的排名数据以及各游戏模式的详细排名信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")] // 自动将JSON的驼峰命名转换为Rust的蛇形命名
pub struct RankedStats {
    /// 当前赛季的分段积分
    pub current_season_split_points: i32,
    /// 已获得的荣誉奖励ID列表
    pub earned_regalia_reward_ids: Vec<String>,
    /// 当前赛季达到的最高召唤师峡谷排名段位
    #[serde(rename = "highestCurrentSeasonReachedTierSR")] // 保持与JSON字段名一致
    pub highest_current_season_reached_tier_sr: String,
    /// 上一赛季结束时的最高 division（如IV、III等）
    pub highest_previous_season_end_division: String,
    /// 上一赛季结束时的最高 tier（如GOLD、SILVER等）
    pub highest_previous_season_end_tier: String,
    /// 最高排名记录（综合所有模式）
    pub highest_ranked_entry: RankedEntry,
    /// 召唤师峡谷模式的最高排名记录
    #[serde(rename = "highestRankedEntrySR")]
    pub highest_ranked_entry_sr: RankedEntry,
    /// 上一赛季的分段积分
    pub previous_season_split_points: i32,
    /// 以游戏模式为键的排名信息映射表
    /// 键为队列类型（如"RANKED_FLEX_SR"），值为对应模式的排名详情
    pub queue_map: HashMap<String, RankedEntry>,
    /// 所有排名模式的列表形式
    pub queues: Vec<RankedEntry>,
    /// 排名荣誉等级
    pub ranked_regalia_level: i32,
    /// 各游戏模式的赛季信息映射表
    pub seasons: HashMap<String, SeasonInfo>,
    /// 是否显示排名指示器
    pub should_show_indicator: bool,
    /// 分段进度信息（键为分段标识，值为进度值）
    pub splits_progress: HashMap<String, i32>,
}

/// 赛季信息结构，包含赛季时间范围和ID
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SeasonInfo {
    /// 当前赛季结束的时间戳（毫秒）
    pub current_season_end: i64,
    /// 当前赛季的ID
    pub current_season_id: i32,
    /// 下一个赛季开始的时间戳（毫秒，0表示未确定）
    pub next_season_start: i64,
}

/// 单种游戏模式的排名详情条目
/// 包含当前段位、胜率、历史最高段位等信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedEntry {
    /// 当前赛季用于计算奖励的胜场数
    pub current_season_wins_for_rewards: i32,
    /// 当前段位的division（如IV、III、II、I）
    pub division: String,
    /// 历史最高division
    pub highest_division: String,
    /// 历史最高tier（如GOLD、SILVER等）
    pub highest_tier: String,
    /// 是否处于定级赛阶段
    pub is_provisional: bool,
    /// 当前段位的联赛积分（LP）
    pub league_points: i32,
    /// 败场数
    pub losses: i32,
    /// 晋级赛进度（如"W-W-L"表示两胜一负，空字符串表示非晋级赛阶段）
    pub mini_series_progress: String,
    /// 上一赛季结束时的division
    pub previous_season_end_division: String,
    /// 上一赛季结束时的tier
    pub previous_season_end_tier: String,
    /// 上一赛季的历史最高division
    pub previous_season_highest_division: String,
    /// 上一赛季的历史最高tier
    pub previous_season_highest_tier: String,
    /// 上一赛季用于计算奖励的胜场数
    pub previous_season_wins_for_rewards: i32,
    /// 定级赛总场次阈值
    pub provisional_game_threshold: i32,
    /// 剩余定级赛场次
    pub provisional_games_remaining: i32,
    /// 游戏模式类型（如"RANKED_SOLO_5x5"表示单排，"RANKED_FLEX_SR"表示灵活组排）
    pub queue_type: String,
    /// 评级分数（部分模式使用）
    pub rated_rating: i32,
    /// 评级段位（部分模式使用）
    pub rated_tier: String,
    /// 当前段位的tier（如BRONZE、SILVER、GOLD等）
    pub tier: String,
    /// 警告信息（可选，无警告时为None）
    pub warnings: Option<()>,
    /// 胜场数
    pub wins: i32,
}