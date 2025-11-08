use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameDetailsLol {
    pub metadata: Metadata,
    pub json: Json,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub product: String,
    pub tags: Vec<String>,
    pub participants: Vec<String>,
    pub timestamp: String,
    #[serde(rename = "data_version")]
    pub data_version: String,
    #[serde(rename = "info_type")]
    pub info_type: String,
    #[serde(rename = "match_id")]
    pub match_id: String,
    pub private: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Json {
    pub end_of_game_result: String,
    pub frame_interval: i64,
    pub frames: Vec<Frames>,
    pub game_id: i64,
    pub participants: Vec<Participant>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Frames {
    pub events: Vec<Event>,
    pub participant_frames: ParticipantFrames,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub game_id: Option<i64>,
    pub real_timestamp: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub winning_team: Option<i64>,
    pub item_id: Option<i64>,
    pub participant_id: Option<i64>,
    pub level_up_type: Option<String>,
    pub skill_slot: Option<i64>,
    #[serde(default)]
    pub assisting_participant_ids: Vec<i64>,
    pub bounty: Option<i64>,
    pub building_type: Option<String>,
    pub killer_id: Option<i64>,
    pub lane_type: Option<String>,
    pub position: Option<Position>,
    pub team_id: Option<i64>,
    pub tower_type: Option<String>,
    pub level: Option<i64>,
    pub kill_type: Option<String>,
    pub multi_kill_length: Option<i64>,
    pub kill_streak_length: Option<i64>,
    pub shutdown_bounty: Option<i64>,
    #[serde(default)]
    pub victim_damage_dealt: Vec<VictimDamageDealt>,
    #[serde(default)]
    pub victim_damage_received: Vec<VictimDamageDealt>,
    pub victim_id: Option<i64>,
    pub creator_id: Option<i64>,
    pub ward_type: Option<String>,
    pub killer_team_id: Option<i64>,
    pub monster_type: Option<String>,
    pub monster_sub_type: Option<String>,
    pub after_id: Option<i64>,
    pub before_id: Option<i64>,
    pub gold_gain: Option<i64>,
    pub actual_start_time: Option<i64>,
    pub feat_type: Option<i64>,
    pub feat_value: Option<i64>,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VictimDamageDealt {
    pub basic: bool,
    pub magic_damage: i64,
    pub name: String,
    pub participant_id: i64,
    pub physical_damage: i64,
    pub spell_name: String,
    pub spell_slot: i64,
    pub true_damage: i64,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrames {
    #[serde(rename = "1")]
    pub n1: N1,
    #[serde(rename = "10")]
    pub n10: N1,
    #[serde(rename = "2")]
    pub n2: N1,
    #[serde(rename = "3")]
    pub n3: N1,
    #[serde(rename = "4")]
    pub n4: N1,
    #[serde(rename = "5")]
    pub n5: N1,
    #[serde(rename = "6")]
    pub n6: N1,
    #[serde(rename = "7")]
    pub n7: N1,
    #[serde(rename = "8")]
    pub n8: N1,
    #[serde(rename = "9")]
    pub n9: N1,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct N1 {
    pub champion_stats: ChampionStats,
    pub current_gold: i64,
    pub damage_stats: DamageStats,
    pub gold_per_second: i64,
    pub jungle_minions_killed: i64,
    pub level: i64,
    pub minions_killed: i64,
    pub participant_id: i64,
    pub position: Position,
    pub time_enemy_spent_controlled: i64,
    pub total_gold: i64,
    pub xp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_haste: i64,
    pub ability_power: i64,
    pub armor: i64,
    pub armor_pen: i64,
    pub armor_pen_percent: i64,
    pub attack_damage: i64,
    pub attack_speed: i64,
    pub bonus_armor_pen_percent: i64,
    pub bonus_magic_pen_percent: i64,
    pub cc_reduction: i64,
    pub cooldown_reduction: i64,
    pub health: i64,
    pub health_max: i64,
    pub health_regen: i64,
    pub lifesteal: i64,
    pub magic_pen: i64,
    pub magic_pen_percent: i64,
    pub magic_resist: i64,
    pub movement_speed: i64,
    pub omnivamp: i64,
    pub physical_vamp: i64,
    pub power: i64,
    pub power_max: i64,
    pub power_regen: i64,
    pub spell_vamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    pub magic_damage_done: i64,
    pub magic_damage_done_to_champions: i64,
    pub magic_damage_taken: i64,
    pub physical_damage_done: i64,
    pub physical_damage_done_to_champions: i64,
    pub physical_damage_taken: i64,
    pub total_damage_done: i64,
    pub total_damage_done_to_champions: i64,
    pub total_damage_taken: i64,
    pub true_damage_done: i64,
    pub true_damage_done_to_champions: i64,
    pub true_damage_taken: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i64,
    pub puuid: String,
}
