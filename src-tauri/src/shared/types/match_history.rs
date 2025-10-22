use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MatchHistory {
    pub account_id: i64,
    pub games: Games,
    pub platform_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    pub game_begin_date: String,
    pub game_count: i32,
    pub game_end_date: String,
    pub game_index_begin: i32,
    pub game_index_end: i32,
    pub games: Vec<Game>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub end_of_game_result: String,
    pub game_creation: i64,
    pub game_creation_date: String,
    pub game_duration: i32,
    pub game_id: i64,
    pub game_mode: String,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub participant_identities: Vec<ParticipantIdentity>,
    pub participants: Vec<Participant>,
    pub platform_id: String,
    pub queue_id: i32,
    pub season_id: i32,
    pub teams: Vec<Team>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<serde_json::Value>, // any[]
    pub baron_kills: i32,
    pub dominion_victory_score: i32,
    pub dragon_kills: i32,
    pub first_baron: bool,
    pub first_blood: bool,
    #[serde(rename = "firstDargon")] // 保持与原接口拼写一致（可能是笔误）
    pub first_dargon: bool,
    pub first_inhibitor: bool,
    pub first_tower: bool,
    pub inhibitor_kills: i32,
    pub rift_herald_kills: i32,
    pub team_id: i32,
    pub tower_kills: i32,
    pub vilemaw_kills: i32,
    pub win: String, // 可根据需要改为枚举："Win" | "Fail"
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub champion_id: i32,
    pub highest_achieved_season_tier: String,
    pub participant_id: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
    pub stats: Stats,
    pub team_id: i32,
    pub timeline: Timeline,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub creeps_per_min_deltas: CreepsPerMinDeltas,
    pub cs_diff_per_min_deltas: CsDiffPerMinDeltas,
    pub damage_taken_diff_per_min_deltas: CsDiffPerMinDeltas,
    pub damage_taken_per_min_deltas: CreepsPerMinDeltas,
    pub gold_per_min_deltas: CreepsPerMinDeltas,
    pub lane: String,
    pub participant_id: i32,
    pub role: String,
    pub xp_diff_per_min_deltas: CsDiffPerMinDeltas,
    pub xp_per_min_deltas: CreepsPerMinDeltas,
}

/// 键为时间区间（如 "0-10"、"10-20" 等），值为对应数值
pub type CsDiffPerMinDeltas = HashMap<String, f64>;

/// 键为时间区间（如 "0-10"、"10-20" 等），值为对应数值
pub type CreepsPerMinDeltas = HashMap<String, f64>;

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub assists: i32,
    pub caused_early_surrender: bool,
    pub champ_level: i32,
    pub combat_player_score: i32,
    pub damage_dealt_to_objectives: i32,
    pub damage_dealt_to_turrets: i32,
    pub damage_self_mitigated: i32,
    pub deaths: i32,
    pub double_kills: i32,
    pub early_surrender_accomplice: bool,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_inhibitor_assist: bool,
    pub first_inhibitor_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub gold_earned: i32,
    pub gold_spent: i32,
    pub inhibitor_kills: i32,
    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
    pub player_augment1: i32,
    pub player_augment2: i32,
    pub player_augment3: i32,
    pub player_augment4: i32,
    pub player_augment5: i32,
    pub player_augment6: i32,
    pub killing_sprees: i32,
    pub kills: i32,
    pub largest_critical_strike: i32,
    pub largest_killing_spree: i32,
    pub largest_multi_kill: i32,
    pub longest_time_spent_living: i32,
    pub magic_damage_dealt: i32,
    pub magic_damage_dealt_to_champions: i32,
    pub magical_damage_taken: i32,
    pub neutral_minions_killed: i32,
    pub neutral_minions_killed_enemy_jungle: i32,
    pub neutral_minions_killed_team_jungle: i32,
    pub objective_player_score: i32,
    pub participant_id: i32,
    pub penta_kills: i32,
    pub perk0: i32,
    pub perk0_var1: i32,
    pub perk0_var2: i32,
    pub perk0_var3: i32,
    pub perk1: i32,
    pub perk1_var1: i32,
    pub perk1_var2: i32,
    pub perk1_var3: i32,
    pub perk2: i32,
    pub perk2_var1: i32,
    pub perk2_var2: i32,
    pub perk2_var3: i32,
    pub perk3: i32,
    pub perk3_var1: i32,
    pub perk3_var2: i32,
    pub perk3_var3: i32,
    pub perk4: i32,
    pub perk4_var1: i32,
    pub perk4_var2: i32,
    pub perk4_var3: i32,
    pub perk5: i32,
    pub perk5_var1: i32,
    pub perk5_var2: i32,
    pub perk5_var3: i32,
    pub perk_primary_style: i32,
    pub perk_sub_style: i32,
    pub physical_damage_dealt: i32,
    pub physical_damage_dealt_to_champions: i32,
    pub physical_damage_taken: i32,
    pub player_score0: i32,
    pub player_score1: i32,
    pub player_score2: i32,
    pub player_score3: i32,
    pub player_score4: i32,
    pub player_score5: i32,
    pub player_score6: i32,
    pub player_score7: i32,
    pub player_score8: i32,
    pub player_score9: i32,
    pub quadra_kills: i32,
    pub sight_wards_bought_in_game: i32,
    pub subteam_placement: i32,
    pub team_early_surrendered: bool,
    pub time_ccing_others: i32,
    pub total_damage_dealt: i32,
    pub total_damage_dealt_to_champions: i32,
    pub total_damage_taken: i32,
    pub total_heal: i32,
    pub total_minions_killed: i32,
    pub total_player_score: i32,
    pub total_score_rank: i32,
    pub total_time_crowd_control_dealt: i32,
    pub total_units_healed: i32,
    pub triple_kills: i32,
    pub true_damage_dealt: i32,
    pub true_damage_dealt_to_champions: i32,
    pub true_damage_taken: i32,
    pub turret_kills: i32,
    pub unreal_kills: i32,
    pub vision_score: i32,
    pub vision_wards_bought_in_game: i32,
    pub wards_killed: i32,
    pub wards_placed: i32,
    pub win: bool,
    pub player_subteam_id: i32,

    // SGP 转换的额外字段
    pub individual_position: Option<String>, // "Invalid" | "TOP" | "JUNGLE" | "MIDDLE" | "BOTTOM" | "UTILITY"
    pub lane: Option<String>,
    pub team_position: Option<String>,
    pub push_pings: Option<i32>,
    pub vision_cleared_pings: Option<i32>,
    pub all_in_pings: Option<i32>,
    pub assist_me_pings: Option<i32>,
    pub basic_pings: Option<i32>,
    pub command_pings: Option<i32>,
    pub danger_pings: Option<i32>,
    pub enemy_missing_pings: Option<i32>,
    pub enemy_vision_pings: Option<i32>,
    pub get_back_pings: Option<i32>,
    pub hold_pings: Option<i32>,
    pub need_vision_pings: Option<i32>,
    pub on_my_way_pings: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantIdentity {
    pub participant_id: i32,
    pub player: Player,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub account_id: i64,
    pub current_account_id: i64,
    pub current_platform_id: String,
    pub match_history_uri: String,
    pub platform_id: String,
    pub profile_icon: i32,
    pub summoner_id: i64,
    pub puuid: String,
    pub game_name: String,
    pub tag_line: String,
    pub summoner_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameTimeline {
    pub frames: Vec<GameTimelineFrame>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameTimelineFrame {
    pub events: Vec<Event>,
    pub participant_frames: HashMap<i32, ParticipantFrame>, // 键为 participantId
    pub timestamp: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrame {
    pub current_gold: i32,
    pub dominion_score: i32,
    pub jungle_minions_killed: i32,
    pub level: i32,
    pub minions_killed: i32,
    pub participant_id: i32,
    pub position: Position,
    pub team_score: i32,
    pub total_gold: i32,
    pub xp: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub assisting_participant_ids: Vec<i32>,
    pub building_type: Option<String>,
    pub item_id: Option<i32>,
    pub killer_id: Option<i32>,
    pub lane_type: Option<String>,
    pub monster_sub_type: Option<String>,
    pub monster_type: Option<String>,
    pub participant_id: Option<i32>,
    pub position: Option<Position>,
    pub skill_slot: Option<i32>,
    pub team_id: Option<i32>,
    pub timestamp: i64,
    pub tower_type: Option<String>,
    pub r#type: String, // 转义关键字 type
    pub victim_id: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// 判断是否为 PvE 队列
pub fn is_pve_queue(queue_id: i32) -> bool {
    // PvE 队列 ID 集合（与 TypeScript 中的 pveQueues 保持一致）
    static PVE_QUEUES: [i32; 58] = [
        // 人机模式和新手教程
        31, 32, 33, 34, 35, 36, 52, 800, 801, 810, 820, 830, 831, 832, 840, 841, 842, 850, 851, 852,
        860, 870, 880, 890, 2000, 2010, 2020,
        // 大提莫节，应该是新版的末日人机
        90, 91, 92, 950, 951, 960, 961,
        // 怪兽入侵模式，没打过不知道是不是
        981, 982, 990,
        // 奥德赛系列
        1030, 1031, 1032, 1040, 1041, 1050, 1051, 1060, 1061, 1070, 1071,
        // STRAWBERRY 模式
        1800, 1810, 1820, 1830, 1840, 1850, 1860, 1870, 1880, 1890,
    ];
    PVE_QUEUES.contains(&queue_id)
}
