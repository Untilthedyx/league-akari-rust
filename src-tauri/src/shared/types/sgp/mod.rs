use serde::{Deserialize, Serialize};

// ============================================================================
// SGP Match History Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpMatchHistoryLol {
    pub games: Vec<SgpGameSummaryLol>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameSummaryLol {
    pub metadata: SgpGameMetadataLol,
    pub json: SgpGameSummaryJsonLol,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameSummaryJsonLol {
    pub end_of_game_result: String,
    pub game_creation: i64,
    pub game_duration: i64,
    pub game_end_timestamp: i64,
    pub game_id: i64,
    pub game_mode: String,
    pub game_name: String,
    pub game_start_timestamp: i64,
    pub game_type: String,
    pub game_version: String,
    pub map_id: i32,
    pub participants: Vec<SgpParticipantLol>,
    pub platform_id: String,
    pub queue_id: i32,
    pub season_id: i32,
    pub teams: Vec<Team>,
    pub tournament_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub bans: Vec<Ban>,
    pub objectives: Objectives,
    pub team_id: i32,
    pub win: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Objectives {
    pub baron: Baron,
    pub champion: Baron,
    pub dragon: Baron,
    pub horde: Baron,
    pub inhibitor: Baron,
    pub rift_herald: Baron,
    pub tower: Baron,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Baron {
    pub first: bool,
    pub kills: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    pub champion_id: i32,
    pub pick_turn: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpParticipantLol {
    pub all_in_pings: i32,
    pub assist_me_pings: i32,
    pub assists: i32,
    pub baron_kills: i32,
    pub basic_pings: i32,
    pub bounty_level: i32,
    pub challenges: Challenges,
    pub champ_experience: i32,
    pub champ_level: i32,
    pub champion_id: i32,
    pub champion_name: String,
    pub champion_transform: i32,
    pub command_pings: i32,
    pub consumables_purchased: i32,
    pub damage_dealt_to_buildings: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub damage_self_mitigated: i64,
    pub danger_pings: i32,
    pub deaths: i32,
    pub detector_wards_placed: i32,
    pub double_kills: i32,
    pub dragon_kills: i32,
    pub eligible_for_progression: bool,
    pub enemy_missing_pings: i32,
    pub enemy_vision_pings: i32,
    pub first_blood_assist: bool,
    pub first_blood_kill: bool,
    pub first_tower_assist: bool,
    pub first_tower_kill: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub get_back_pings: i32,
    pub gold_earned: i32,
    pub gold_spent: i32,
    pub hold_pings: i32,
    pub individual_position: String,
    pub inhibitor_kills: i32,
    pub inhibitor_takedowns: i32,
    pub inhibitors_lost: i32,
    pub item0: i32,
    pub item1: i32,
    pub item2: i32,
    pub item3: i32,
    pub item4: i32,
    pub item5: i32,
    pub item6: i32,
    pub items_purchased: i32,
    pub killing_sprees: i32,
    pub kills: i32,
    pub lane: String,
    pub largest_critical_strike: i32,
    pub largest_killing_spree: i32,
    pub largest_multi_kill: i32,
    pub longest_time_spent_living: i32,
    pub magic_damage_dealt: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub magic_damage_taken: i64,
    pub missions: SgpMissions,
    pub need_vision_pings: i32,
    pub neutral_minions_killed: i32,
    pub nexus_kills: i32,
    pub nexus_lost: i32,
    pub nexus_takedowns: i32,
    pub objectives_stolen: i32,
    pub objectives_stolen_assists: i32,
    pub on_my_way_pings: i32,
    pub participant_id: i32,
    pub penta_kills: i32,
    pub perks: Perks,
    pub physical_damage_dealt: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub physical_damage_taken: i64,
    pub placement: i32,
    pub player_augment1: i32,
    pub player_augment2: i32,
    pub player_augment3: i32,
    pub player_augment4: i32,
    pub player_augment5: i32,
    pub player_augment6: i32,
    pub player_subteam_id: i32,
    pub profile_icon: i32,
    pub push_pings: i32,
    pub puuid: String,
    pub quadra_kills: i32,
    pub riot_id_game_name: String,
    pub riot_id_tagline: String,
    pub role: String,
    pub sight_wards_bought_in_game: i32,
    pub spell1_casts: i32,
    pub spell1_id: i32,
    pub spell2_casts: i32,
    pub spell2_id: i32,
    pub spell3_casts: i32,
    pub spell4_casts: i32,
    pub subteam_placement: i32,
    pub summoner1_casts: i32,
    pub summoner2_casts: i32,
    pub summoner_id: i64,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_early_surrendered: bool,
    pub team_id: i32,
    pub team_position: String,
    pub time_c_cing_others: i32,
    pub time_played: i32,
    pub total_ally_jungle_minions_killed: i32,
    pub total_damage_dealt: i64,
    pub total_damage_dealt_to_champions: i64,
    pub total_damage_shielded_on_teammates: i64,
    pub total_damage_taken: i64,
    pub total_enemy_jungle_minions_killed: i32,
    pub total_heal: i64,
    pub total_heals_on_teammates: i64,
    pub total_minions_killed: i32,
    pub total_time_cc_dealt: i32,
    pub total_time_spent_dead: i32,
    pub total_units_healed: i32,
    pub triple_kills: i32,
    pub true_damage_dealt: i64,
    pub true_damage_dealt_to_champions: i64,
    pub true_damage_taken: i64,
    pub turret_kills: i32,
    pub turret_takedowns: i32,
    pub turrets_lost: i32,
    pub unreal_kills: i32,
    pub vision_cleared_pings: i32,
    pub vision_score: i32,
    pub vision_wards_bought_in_game: i32,
    pub wards_killed: i32,
    pub wards_placed: i32,
    pub win: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Perks {
    pub stat_perks: SgpStatPerks,
    pub styles: Vec<Style>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub description: String,
    pub selections: Vec<SgpSelection>,
    pub style: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpSelection {
    pub perk: i32,
    pub var1: i32,
    pub var2: i32,
    pub var3: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpStatPerks {
    pub defense: i32,
    pub flex: i32,
    pub offense: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpMissions {
    #[serde(rename = "Missions_ChampionsKilled")]
    pub missions_champions_killed: i32,
    #[serde(rename = "Missions_CreepScore")]
    pub missions_creep_score: i32,
    #[serde(rename = "Missions_GoldFromStructuresDestroyed")]
    pub missions_gold_from_structures_destroyed: i32,
    #[serde(rename = "Missions_GoldFromTurretPlatesTaken")]
    pub missions_gold_from_turret_plates_taken: i32,
    #[serde(rename = "Missions_HealingFromLevelObjects")]
    pub missions_healing_from_level_objects: i32,
    #[serde(rename = "Missions_MinionsKilled")]
    pub missions_minions_killed: i32,
    #[serde(rename = "Missions_TurretPlatesDestroyed")]
    pub missions_turret_plates_destroyed: i32,
    #[serde(rename = "PlayerScore0")]
    pub player_score0: f64,
    #[serde(rename = "PlayerScore1")]
    pub player_score1: f64,
    #[serde(rename = "PlayerScore10")]
    pub player_score10: f64,
    #[serde(rename = "PlayerScore11")]
    pub player_score11: f64,
    #[serde(rename = "PlayerScore2")]
    pub player_score2: f64,
    #[serde(rename = "PlayerScore3")]
    pub player_score3: f64,
    #[serde(rename = "PlayerScore4")]
    pub player_score4: f64,
    #[serde(rename = "PlayerScore5")]
    pub player_score5: f64,
    #[serde(rename = "PlayerScore6")]
    pub player_score6: f64,
    #[serde(rename = "PlayerScore7")]
    pub player_score7: f64,
    #[serde(rename = "PlayerScore8")]
    pub player_score8: f64,
    #[serde(rename = "PlayerScore9")]
    pub player_score9: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Challenges {
    #[serde(rename = "12AssistStreakCount")]
    pub assist_streak_count_12: f64,
    #[serde(rename = "HealFromMapSources")]
    pub heal_from_map_sources: f64,
    pub infernal_scale_pickup: f64,
    pub ability_uses: f64,
    pub aces_before15_minutes: f64,
    pub allied_jungle_monster_kills: f64,
    pub baron_takedowns: f64,
    pub blast_cone_opposite_opponent_count: f64,
    pub bounty_gold: f64,
    pub buffs_stolen: f64,
    pub complete_support_quest_in_time: f64,
    pub control_wards_placed: f64,
    pub damage_per_minute: f64,
    pub damage_taken_on_team_percentage: f64,
    pub danced_with_rift_herald: f64,
    pub deaths_by_enemy_champs: f64,
    pub dodge_skill_shots_small_window: f64,
    pub double_aces: f64,
    pub dragon_takedowns: f64,
    pub early_laning_phase_gold_exp_advantage: Option<f64>,
    pub effective_heal_and_shielding: f64,
    pub elder_dragon_kills_with_opposing_soul: f64,
    pub elder_dragon_multikills: f64,
    pub enemy_champion_immobilizations: f64,
    pub enemy_jungle_monster_kills: f64,
    pub epic_monster_kills_near_enemy_jungler: f64,
    pub epic_monster_kills_within30_seconds_of_spawn: f64,
    pub epic_monster_steals: f64,
    pub epic_monster_stolen_without_smite: f64,
    pub first_turret_killed: f64,
    pub fist_bump_participation: f64,
    pub flawless_aces: f64,
    pub full_team_takedown: f64,
    pub game_length: f64,
    pub get_takedowns_in_all_lanes_early_jungle_as_laner: Option<f64>,
    pub gold_per_minute: f64,
    pub had_open_nexus: f64,
    pub immobilize_and_kill_with_ally: f64,
    pub initial_buff_count: f64,
    pub initial_crab_count: f64,
    pub jungle_cs_before10_minutes: f64,
    pub jungler_takedowns_near_damaged_epic_monster: f64,
    pub k_turrets_destroyed_before_plates_fall: f64,
    pub kda: f64,
    pub kill_after_hidden_with_ally: f64,
    pub kill_participation: f64,
    pub killed_champ_took_full_team_damage_survived: f64,
    pub killing_sprees: f64,
    pub kills_near_enemy_turret: f64,
    pub kills_on_other_lanes_early_jungle_as_laner: Option<f64>,
    pub kills_on_recently_healed_by_aram_pack: f64,
    pub kills_under_own_turret: f64,
    pub kills_with_help_from_epic_monster: f64,
    pub knock_enemy_into_team_and_kill: f64,
    pub land_skill_shots_early_game: f64,
    pub lane_minions_first10_minutes: f64,
    pub laning_phase_gold_exp_advantage: Option<f64>,
    pub legendary_count: f64,
    pub legendary_item_used: Vec<f64>,
    pub lost_an_inhibitor: f64,
    pub max_cs_advantage_on_lane_opponent: Option<f64>,
    pub max_kill_deficit: f64,
    pub max_level_lead_lane_opponent: Option<f64>,
    pub mejais_full_stack_in_time: f64,
    pub more_enemy_jungle_than_opponent: f64,
    pub multi_kill_one_spell: f64,
    pub multi_turret_rift_herald_count: f64,
    pub multikills: f64,
    pub multikills_after_aggressive_flash: f64,
    pub outer_turret_executes_before10_minutes: f64,
    pub outnumbered_kills: f64,
    pub outnumbered_nexus_kill: f64,
    pub perfect_dragon_souls_taken: f64,
    pub perfect_game: f64,
    pub pick_kill_with_ally: f64,
    pub played_champ_select_position: Option<f64>,
    pub poro_explosions: f64,
    pub quick_cleanse: f64,
    pub quick_first_turret: f64,
    pub quick_solo_kills: f64,
    pub rift_herald_takedowns: f64,
    pub save_ally_from_death: f64,
    pub scuttle_crab_kills: f64,
    pub skillshots_dodged: f64,
    pub skillshots_hit: f64,
    pub snowballs_hit: f64,
    pub solo_baron_kills: f64,
    pub solo_kills: f64,
    pub stealth_wards_placed: f64,
    pub survived_single_digit_hp_count: f64,
    pub survived_three_immobilizes_in_fight: f64,
    pub takedown_on_first_turret: f64,
    pub takedowns: f64,
    pub takedowns_after_gaining_level_advantage: f64,
    pub takedowns_before_jungle_minion_spawn: f64,
    pub takedowns_first_x_minutes: f64,
    pub takedowns_in_alcove: f64,
    pub takedowns_in_enemy_fountain: f64,
    pub team_baron_kills: f64,
    pub team_damage_percentage: f64,
    pub team_elder_dragon_kills: f64,
    pub team_rift_herald_kills: f64,
    pub took_large_damage_survived: f64,
    pub turret_plates_taken: f64,
    pub turret_takedowns: f64,
    pub turrets_taken_with_rift_herald: f64,
    pub twenty_minions_in3_seconds_count: f64,
    pub two_wards_one_sweeper_count: f64,
    pub unseen_recalls: f64,
    pub vision_score_advantage_lane_opponent: Option<f64>,
    pub vision_score_per_minute: f64,
    pub void_monster_kill: f64,
    pub ward_takedowns: f64,
    pub ward_takedowns_before20_m: f64,
    pub wards_guarded: f64,
    pub jungler_kills_early_jungle: Option<f64>,
    pub kills_on_laners_early_jungle_as_jungler: Option<f64>,
    pub solo_turrets_lategame: Option<f64>,
    pub control_ward_time_coverage_in_river_or_enemy_half: Option<f64>,
    pub faster_support_quest_completion: Option<f64>,
    pub highest_ward_kills: Option<f64>,
    pub first_turret_killed_time: Option<f64>,
    pub highest_champion_damage: Option<f64>,
    pub shortest_time_to_ace_from_first_takedown: Option<f64>,
    pub earliest_dragon_takedown: Option<f64>,
    pub highest_crowd_control_score: Option<f64>,
    pub baron_buff_gold_advantage_over_threshold: Option<f64>,
    pub earliest_baron: Option<f64>,
    pub earliest_elder_dragon: Option<f64>,
    pub fastest_legendary: Option<f64>,
    pub third_inhibitor_destroyed_time: Option<f64>,
    pub teleport_takedowns: Option<f64>,
    pub had_afk_teammate: Option<f64>,
    
    // SWARM 相关字段
    #[serde(rename = "SWARM_DefeatAatrox")]
    pub swarm_defeat_aatrox: f64,
    #[serde(rename = "SWARM_DefeatBriar")]
    pub swarm_defeat_briar: f64,
    #[serde(rename = "SWARM_DefeatMiniBosses")]
    pub swarm_defeat_mini_bosses: f64,
    #[serde(rename = "SWARM_EvolveWeapon")]
    pub swarm_evolve_weapon: f64,
    #[serde(rename = "SWARM_Have3Passives")]
    pub swarm_have3_passives: f64,
    #[serde(rename = "SWARM_KillEnemy")]
    pub swarm_kill_enemy: f64,
    #[serde(rename = "SWARM_PickupGold")]
    pub swarm_pickup_gold: f64,
    #[serde(rename = "SWARM_ReachLevel50")]
    pub swarm_reach_level50: f64,
    #[serde(rename = "SWARM_Survive15Min")]
    pub swarm_survive15_min: f64,
    #[serde(rename = "SWARM_WinWith5EvolvedWeapons")]
    pub swarm_win_with5_evolved_weapons: f64,
}

impl Default for Challenges {
    fn default() -> Self {
        Challenges {
            assist_streak_count_12: 0.0,
            heal_from_map_sources: 0.0,
            infernal_scale_pickup: 0.0,
            ability_uses: 0.0,
            aces_before15_minutes: 0.0,
            allied_jungle_monster_kills: 0.0,
            baron_takedowns: 0.0,
            blast_cone_opposite_opponent_count: 0.0,
            bounty_gold: 0.0,
            buffs_stolen: 0.0,
            complete_support_quest_in_time: 0.0,
            control_wards_placed: 0.0,
            damage_per_minute: 0.0,
            damage_taken_on_team_percentage: 0.0,
            danced_with_rift_herald: 0.0,
            deaths_by_enemy_champs: 0.0,
            dodge_skill_shots_small_window: 0.0,
            double_aces: 0.0,
            dragon_takedowns: 0.0,
            early_laning_phase_gold_exp_advantage: None,
            effective_heal_and_shielding: 0.0,
            elder_dragon_kills_with_opposing_soul: 0.0,
            elder_dragon_multikills: 0.0,
            enemy_champion_immobilizations: 0.0,
            enemy_jungle_monster_kills: 0.0,
            epic_monster_kills_near_enemy_jungler: 0.0,
            epic_monster_kills_within30_seconds_of_spawn: 0.0,
            epic_monster_steals: 0.0,
            epic_monster_stolen_without_smite: 0.0,
            first_turret_killed: 0.0,
            fist_bump_participation: 0.0,
            flawless_aces: 0.0,
            full_team_takedown: 0.0,
            game_length: 0.0,
            get_takedowns_in_all_lanes_early_jungle_as_laner: None,
            gold_per_minute: 0.0,
            had_open_nexus: 0.0,
            immobilize_and_kill_with_ally: 0.0,
            initial_buff_count: 0.0,
            initial_crab_count: 0.0,
            jungle_cs_before10_minutes: 0.0,
            jungler_takedowns_near_damaged_epic_monster: 0.0,
            k_turrets_destroyed_before_plates_fall: 0.0,
            kda: 0.0,
            kill_after_hidden_with_ally: 0.0,
            kill_participation: 0.0,
            killed_champ_took_full_team_damage_survived: 0.0,
            killing_sprees: 0.0,
            kills_near_enemy_turret: 0.0,
            kills_on_other_lanes_early_jungle_as_laner: None,
            kills_on_recently_healed_by_aram_pack: 0.0,
            kills_under_own_turret: 0.0,
            kills_with_help_from_epic_monster: 0.0,
            knock_enemy_into_team_and_kill: 0.0,
            land_skill_shots_early_game: 0.0,
            lane_minions_first10_minutes: 0.0,
            laning_phase_gold_exp_advantage: None,
            legendary_count: 0.0,
            legendary_item_used: Vec::new(),
            lost_an_inhibitor: 0.0,
            max_cs_advantage_on_lane_opponent: None,
            max_kill_deficit: 0.0,
            max_level_lead_lane_opponent: None,
            mejais_full_stack_in_time: 0.0,
            more_enemy_jungle_than_opponent: 0.0,
            multi_kill_one_spell: 0.0,
            multi_turret_rift_herald_count: 0.0,
            multikills: 0.0,
            multikills_after_aggressive_flash: 0.0,
            outer_turret_executes_before10_minutes: 0.0,
            outnumbered_kills: 0.0,
            outnumbered_nexus_kill: 0.0,
            perfect_dragon_souls_taken: 0.0,
            perfect_game: 0.0,
            pick_kill_with_ally: 0.0,
            played_champ_select_position: None,
            poro_explosions: 0.0,
            quick_cleanse: 0.0,
            quick_first_turret: 0.0,
            quick_solo_kills: 0.0,
            rift_herald_takedowns: 0.0,
            save_ally_from_death: 0.0,
            scuttle_crab_kills: 0.0,
            skillshots_dodged: 0.0,
            skillshots_hit: 0.0,
            snowballs_hit: 0.0,
            solo_baron_kills: 0.0,
            solo_kills: 0.0,
            stealth_wards_placed: 0.0,
            survived_single_digit_hp_count: 0.0,
            survived_three_immobilizes_in_fight: 0.0,
            takedown_on_first_turret: 0.0,
            takedowns: 0.0,
            takedowns_after_gaining_level_advantage: 0.0,
            takedowns_before_jungle_minion_spawn: 0.0,
            takedowns_first_x_minutes: 0.0,
            takedowns_in_alcove: 0.0,
            takedowns_in_enemy_fountain: 0.0,
            team_baron_kills: 0.0,
            team_damage_percentage: 0.0,
            team_elder_dragon_kills: 0.0,
            team_rift_herald_kills: 0.0,
            took_large_damage_survived: 0.0,
            turret_plates_taken: 0.0,
            turret_takedowns: 0.0,
            turrets_taken_with_rift_herald: 0.0,
            twenty_minions_in3_seconds_count: 0.0,
            two_wards_one_sweeper_count: 0.0,
            unseen_recalls: 0.0,
            vision_score_advantage_lane_opponent: None,
            vision_score_per_minute: 0.0,
            void_monster_kill: 0.0,
            ward_takedowns: 0.0,
            ward_takedowns_before20_m: 0.0,
            wards_guarded: 0.0,
            jungler_kills_early_jungle: None,
            kills_on_laners_early_jungle_as_jungler: None,
            solo_turrets_lategame: None,
            control_ward_time_coverage_in_river_or_enemy_half: None,
            faster_support_quest_completion: None,
            highest_ward_kills: None,
            first_turret_killed_time: None,
            highest_champion_damage: None,
            shortest_time_to_ace_from_first_takedown: None,
            earliest_dragon_takedown: None,
            highest_crowd_control_score: None,
            baron_buff_gold_advantage_over_threshold: None,
            earliest_baron: None,
            earliest_elder_dragon: None,
            fastest_legendary: None,
            third_inhibitor_destroyed_time: None,
            teleport_takedowns: None,
            had_afk_teammate: None,
            swarm_defeat_aatrox: 0.0,
            swarm_defeat_briar: 0.0,
            swarm_defeat_mini_bosses: 0.0,
            swarm_evolve_weapon: 0.0,
            swarm_have3_passives: 0.0,
            swarm_kill_enemy: 0.0,
            swarm_pickup_gold: 0.0,
            swarm_reach_level50: 0.0,
            swarm_survive15_min: 0.0,
            swarm_win_with5_evolved_weapons: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SgpGameMetadataLol {
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
    #[serde(rename = "private")]
    pub private: bool,
}

// ============================================================================
// SGP TFT Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpMatchHistoryTft {
    pub games: Vec<SgpGameTft>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameTft {
    pub metadata: SgpGameMetadataLol,
    pub json: SgpGameTftJson,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameTftJson {
    pub end_of_game_result: String,
    pub game_creation: i64,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    pub game_datetime: String,
    #[serde(rename = "game_id")]
    pub game_id_alt: i64,
    pub game_length: f64,
    pub game_version: String,
    pub map_id: i32,
    pub participants: Vec<SgpParticipantTft>,
    #[serde(rename = "queueId")]
    pub queue_id: i32,
    #[serde(rename = "queue_id")]
    pub queue_id_alt: i32,
    pub tft_game_type: String,
    pub tft_set_core_name: String,
    pub tft_set_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpParticipantTft {
    pub augments: Vec<String>,
    pub companion: Companion,
    pub gold_left: i32,
    pub last_round: i32,
    pub level: i32,
    pub missions: SgpMissionsTft,
    pub placement: i32,
    pub players_eliminated: i32,
    pub puuid: String,
    pub time_eliminated: f64,
    pub total_damage_to_players: i32,
    pub traits: Vec<Trait>,
    pub units: Vec<Unit>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
    #[serde(rename = "character_id")]
    pub character_id: String,
    pub item_names: Vec<String>,
    pub name: String,
    pub rarity: i32,
    pub tier: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Trait {
    pub name: String,
    #[serde(rename = "num_units")]
    pub num_units: i32,
    pub style: i32,
    #[serde(rename = "tier_current")]
    pub tier_current: i32,
    #[serde(rename = "tier_total")]
    pub tier_total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpMissionsTft {
    #[serde(rename = "Assists")]
    pub assists: i32,
    #[serde(rename = "DamageDealt")]
    pub damage_dealt: i32,
    #[serde(rename = "DamageDealtToObjectives")]
    pub damage_dealt_to_objectives: i32,
    #[serde(rename = "DamageDealtToTurrets")]
    pub damage_dealt_to_turrets: i32,
    #[serde(rename = "DamageTaken")]
    pub damage_taken: i32,
    #[serde(rename = "Deaths")]
    pub deaths: i32,
    #[serde(rename = "DoubleKills")]
    pub double_kills: i32,
    #[serde(rename = "GoldEarned")]
    pub gold_earned: i32,
    #[serde(rename = "GoldSpent")]
    pub gold_spent: i32,
    #[serde(rename = "InhibitorsDestroyed")]
    pub inhibitors_destroyed: i32,
    #[serde(rename = "KillingSprees")]
    pub killing_sprees: i32,
    #[serde(rename = "Kills")]
    pub kills: i32,
    #[serde(rename = "LargestKillingSpree")]
    pub largest_killing_spree: i32,
    #[serde(rename = "LargestMultiKill")]
    pub largest_multi_kill: i32,
    #[serde(rename = "MagicDamageDealt")]
    pub magic_damage_dealt: i32,
    #[serde(rename = "MagicDamageDealtToChampions")]
    pub magic_damage_dealt_to_champions: i32,
    #[serde(rename = "MagicDamageTaken")]
    pub magic_damage_taken: i32,
    #[serde(rename = "NeutralMinionsKilledTeamJungle")]
    pub neutral_minions_killed_team_jungle: i32,
    #[serde(rename = "PentaKills")]
    pub penta_kills: i32,
    #[serde(rename = "PhysicalDamageDealt")]
    pub physical_damage_dealt: i32,
    #[serde(rename = "PhysicalDamageDealtToChampions")]
    pub physical_damage_dealt_to_champions: i32,
    #[serde(rename = "PhysicalDamageTaken")]
    pub physical_damage_taken: i32,
    #[serde(rename = "PlayerScore0")]
    pub player_score0: i32,
    #[serde(rename = "PlayerScore1")]
    pub player_score1: i32,
    #[serde(rename = "PlayerScore10")]
    pub player_score10: i32,
    #[serde(rename = "PlayerScore11")]
    pub player_score11: i32,
    #[serde(rename = "PlayerScore2")]
    pub player_score2: i32,
    #[serde(rename = "PlayerScore3")]
    pub player_score3: i32,
    #[serde(rename = "PlayerScore4")]
    pub player_score4: i32,
    #[serde(rename = "PlayerScore5")]
    pub player_score5: i32,
    #[serde(rename = "PlayerScore6")]
    pub player_score6: i32,
    #[serde(rename = "PlayerScore9")]
    pub player_score9: i32,
    #[serde(rename = "QuadraKills")]
    pub quadra_kills: i32,
    #[serde(rename = "Spell1Casts")]
    pub spell1_casts: i32,
    #[serde(rename = "Spell2Casts")]
    pub spell2_casts: i32,
    #[serde(rename = "Spell3Casts")]
    pub spell3_casts: i32,
    #[serde(rename = "Spell4Casts")]
    pub spell4_casts: i32,
    #[serde(rename = "SummonerSpell1Casts")]
    pub summoner_spell1_casts: i32,
    #[serde(rename = "TimeCCOthers")]
    pub time_cc_others: i32,
    #[serde(rename = "TotalDamageDealtToChampions")]
    pub total_damage_dealt_to_champions: i32,
    #[serde(rename = "TotalMinionsKilled")]
    pub total_minions_killed: i32,
    #[serde(rename = "TripleKills")]
    pub triple_kills: i32,
    #[serde(rename = "TrueDamageDealt")]
    pub true_damage_dealt: i32,
    #[serde(rename = "TrueDamageDealtToChampions")]
    pub true_damage_dealt_to_champions: i32,
    #[serde(rename = "TrueDamageTaken")]
    pub true_damage_taken: i32,
    #[serde(rename = "UnrealKills")]
    pub unreal_kills: i32,
    #[serde(rename = "VisionScore")]
    pub vision_score: i32,
    #[serde(rename = "WardsKilled")]
    pub wards_killed: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Companion {
    #[serde(rename = "content_ID")]
    pub content_id: String,
    #[serde(rename = "item_ID")]
    pub item_id: i32,
    #[serde(rename = "skin_ID")]
    pub skin_id: i32,
    pub species: String,
}

// ============================================================================
// SGP Game Details Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameDetailsLol {
    pub metadata: SgpGameMetadataLol,
    pub json: SgpGameJsonDetailsLol,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpGameJsonDetailsLol {
    pub end_of_game_result: String,
    pub frame_interval: i64,
    pub frames: Vec<Frame>,
    pub game_id: i64,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub participant_id: i32,
    pub puuid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Frame {
    pub events: Vec<Event>,
    pub participant_frames: std::collections::HashMap<String, ParticipantFrame>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantFrame {
    pub champion_stats: ChampionStats,
    pub current_gold: i32,
    pub damage_stats: DamageStats,
    pub gold_per_second: f64,
    pub jungle_minions_killed: i32,
    pub level: i32,
    pub minions_killed: i32,
    pub participant_id: i32,
    pub position: Position,
    pub time_enemy_spent_controlled: f64,
    pub total_gold: i32,
    pub xp: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DamageStats {
    pub magic_damage_done: i32,
    pub magic_damage_done_to_champions: i32,
    pub magic_damage_taken: i32,
    pub physical_damage_done: i32,
    pub physical_damage_done_to_champions: i32,
    pub physical_damage_taken: i32,
    pub total_damage_done: i32,
    pub total_damage_done_to_champions: i32,
    pub total_damage_taken: i32,
    pub true_damage_done: i32,
    pub true_damage_done_to_champions: i32,
    pub true_damage_taken: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub ability_haste: f64,
    pub ability_power: f64,
    pub armor: f64,
    pub armor_pen: f64,
    pub armor_pen_percent: f64,
    pub attack_damage: f64,
    pub attack_speed: f64,
    pub bonus_armor_pen_percent: f64,
    pub bonus_magic_pen_percent: f64,
    pub cc_reduction: f64,
    pub cooldown_reduction: f64,
    pub health: f64,
    pub health_max: f64,
    pub health_regen: f64,
    pub lifesteal: f64,
    pub magic_pen: f64,
    pub magic_pen_percent: f64,
    pub magic_resist: f64,
    pub movement_speed: f64,
    pub omnivamp: f64,
    pub physical_vamp: f64,
    pub power: f64,
    pub power_max: f64,
    pub power_regen: f64,
    pub spell_vamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub real_timestamp: Option<i64>,
    pub timestamp: i64,
    #[serde(rename = "type")]
    pub event_type: String,
    pub level_up_type: Option<String>,
    pub participant_id: Option<i32>,
    pub skill_slot: Option<i32>,
    pub item_id: Option<i32>,
    pub creator_id: Option<i32>,
    pub ward_type: Option<String>,
    pub level: Option<i32>,
    pub bounty: Option<i32>,
    pub kill_streak_length: Option<i32>,
    pub killer_id: Option<i32>,
    pub position: Option<Position>,
    pub shutdown_bounty: Option<i32>,
    pub victim_damage_received: Option<Vec<VictimDamageReceived>>,
    pub victim_id: Option<i32>,
    pub kill_type: Option<String>,
    pub after_id: Option<i32>,
    pub before_id: Option<i32>,
    pub gold_gain: Option<i32>,
    pub lane_type: Option<String>,
    pub team_id: Option<i32>,
    pub victim_damage_dealt: Option<Vec<VictimDamageReceived>>,
    pub assisting_participant_ids: Option<Vec<i32>>,
    pub killer_team_id: Option<i32>,
    pub monster_type: Option<String>,
    pub monster_sub_type: Option<String>,
    pub multi_kill_length: Option<i32>,
    pub building_type: Option<String>,
    pub tower_type: Option<String>,
    pub game_id: Option<i64>,
    pub winning_team: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct VictimDamageReceived {
    pub basic: bool,
    pub magic_damage: i32,
    pub name: String,
    pub participant_id: i32,
    pub physical_damage: i32,
    pub spell_name: String,
    pub spell_slot: i32,
    pub true_damage: i32,
    #[serde(rename = "type")]
    pub damage_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

// ============================================================================
// SGP Ranked Stats Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpRankedStats {
    pub queues: Vec<Queue>,
    pub highest_previous_season_end_tier: String,
    pub highest_previous_season_end_rank: String,
    pub highest_previous_season_achieved_tier: String,
    pub highest_previous_season_achieved_rank: String,
    pub earned_regalia_reward_ids: Vec<serde_json::Value>,
    pub current_season_split_points: i32,
    pub previous_season_split_points: i32,
    pub seasons: Seasons,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Seasons {
    #[serde(rename = "RANKED_TFT")]
    pub ranked_tft: RankedTft,
    #[serde(rename = "RANKED_TFT_TURBO")]
    pub ranked_tft_turbo: RankedTft,
    #[serde(rename = "CHERRY")]
    pub cherry: RankedTft,
    #[serde(rename = "RANKED_FLEX_SR")]
    pub ranked_flex_sr: RankedTft,
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    pub ranked_tft_double_up: RankedTft,
    #[serde(rename = "RANKED_SOLO_5x5")]
    pub ranked_solo_5x5: RankedTft,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankedTft {
    pub current_season_id: i32,
    pub current_season_end: i64,
    pub next_season_start: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    pub queue_type: String,
    pub provisional_game_threshold: i32,
    pub tier: Option<String>,
    pub rank: Option<String>,
    pub league_points: i32,
    pub cumulative_lp: i32,
    pub wins: i32,
    pub losses: i32,
    pub provisional_games_remaining: i32,
    pub highest_tier: Option<String>,
    pub highest_rank: Option<String>,
    pub previous_season_end_tier: Option<String>,
    pub previous_season_end_rank: Option<String>,
    pub previous_season_highest_tier: Option<String>,
    pub previous_season_highest_rank: Option<String>,
    pub previous_season_achieved_tier: Option<String>,
    pub previous_season_achieved_rank: Option<String>,
    pub rated_rating: i32,
    pub premade_mmr_restricted: bool,
    pub rated_tier: Option<String>,
}

// ============================================================================
// SGP Summoner Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SgpSummoner {
    pub id: i64,
    pub puuid: String,
    pub account_id: i64,
    pub name: String,
    pub internal_name: String,
    pub profile_icon_id: i32,
    pub level: i32,
    pub exp_points: i32,
    pub level_and_xp_version: i32,
    pub revision_id: i64,
    pub revision_date: i64,
    pub last_game_date: i64,
    pub name_change_flag: bool,
    pub unnamed: bool,
    pub privacy: String,
    pub exp_to_next_level: i32,
}

// ============================================================================
// Spectator Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorData {
    pub reconnect_delay: i32,
    pub game_name: String,
    pub game: SpectatorGameflowSession,
    pub player_credentials: SpectatorPlayerCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorPlayerCredentials {
    pub game_id: i64,
    pub queue_id: i32,
    pub player_id: i64,
    pub puuid: String,
    pub server_port: i32,
    pub champion_id: i32,
    pub last_selected_skin_index: i32,
    pub summoner_id: i64,
    pub observer: bool,
    pub game_version: String,
    pub game_mode: String,
    pub observer_encryption_key: String,
    pub observer_server_ip: String,
    pub observer_server_port: i32,
    pub queue_type: String,
    pub game_create_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorGameflowSession {
    pub id: i64,
    pub game_state: String,
    pub queue_type_name: String,
    pub name: String,
    pub pick_turn: i32,
    pub map_id: i32,
    pub game_mode: String,
    pub max_num_players: i32,
    pub game_type: String,
    pub game_queue_config_id: i32,
    pub spectator_delay: i32,
    pub game_version: String,
    pub team_one: Vec<SpectatorGameflowSessionTeam>,
    pub team_two: Vec<SpectatorGameflowSessionTeam>,
    pub player_champion_selections: Vec<SpectatorGameflowSessionPlayerChampionSelection>,
    pub banned_champions: Vec<serde_json::Value>,
    pub observers: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorGameflowSessionPlayerChampionSelection {
    pub summoner_internal_name: String,
    pub champion_id: i32,
    pub selected_skin_index: i32,
    pub spell1_id: i32,
    pub spell2_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SpectatorGameflowSessionTeam {
    pub puuid: String,
    pub summoner_id: i64,
    pub last_selected_skin_index: i32,
    pub team_owner: bool,
    pub profile_icon_id: i32,
    pub team_participant_id: i32,
    pub champion_id: i32,
    pub selected_role: String,
    pub selected_position: String,
    pub summoner_name: String,
    pub summoner_internal_name: String,
}

