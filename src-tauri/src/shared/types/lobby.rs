use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LobbyMember {
    pub allowed_change_activity: bool,
    pub allowed_invite_others: bool,
    pub allowed_kick_others: bool,
    pub allowed_start_activity: bool,
    pub allowed_toggle_invite: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub bot_champion_id: i32,
    pub bot_difficulty: String,
    pub bot_id: String,
    pub first_position_preference: String,
    pub is_bot: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub puuid: String,
    pub ready: bool,
    pub second_position_preference: String,
    pub show_ghosted_banner: bool,
    pub summoner_icon_id: i32,
    pub summoner_id: i64,
    pub summoner_internal_name: String,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Lobby {
    pub can_start_activity: bool,
    pub game_config: GameConfig,
    pub invitations: Vec<Invitation>,
    pub local_member: LocalMember,
    pub members: Vec<LocalMember>,
    pub muc_jwt_dto: MucJwtDto,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    pub party_id: String,
    pub party_type: String,
    pub restrictions: Vec<serde_json::Value>, // any[]
    pub scarce_positions: Vec<serde_json::Value>, // any[]
    pub warnings: Vec<serde_json::Value>, // any[]
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MucJwtDto {
    pub channel_claim: String,
    pub domain: String,
    pub jwt: String,
    pub target_region: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocalMember {
    pub allowed_change_activity: bool,
    pub allowed_invite_others: bool,
    pub allowed_kick_others: bool,
    pub allowed_start_activity: bool,
    pub allowed_toggle_invite: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub bot_champion_id: i32,
    pub bot_difficulty: String,
    pub bot_id: String,
    pub first_position_preference: String,
    /// 竞技场，在小队中的位置，1 或 2
    pub intra_subteam_position: i32,
    pub is_bot: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub player_slots: Vec<serde_json::Value>, // any[]
    pub puuid: String,
    pub ready: bool,
    pub second_position_preference: String,
    pub show_ghosted_banner: bool,
    /// 竞技场中，属于哪个小队，1 到 4
    pub subteam_index: i32,
    pub summoner_icon_id: i32,
    pub summoner_id: i64,
    pub summoner_internal_name: String,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_id: i32,
    pub tft_npe_queue_bypass: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Invitation {
    pub invitation_id: String,
    pub invitation_type: String,
    pub state: String,
    pub timestamp: String,
    pub to_summoner_id: i64,
    pub to_summoner_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameConfig {
    pub allowable_premade_sizes: Vec<i32>,
    pub custom_lobby_name: String,
    pub custom_mutator_name: String,
    pub custom_rewards_disabled_reasons: Vec<serde_json::Value>, // any[]
    pub custom_spectator_policy: String,
    pub custom_spectators: Vec<serde_json::Value>, // any[]
    pub custom_team100: Vec<CustomTeam100>,
    pub custom_team200: Vec<CustomTeam100>,
    pub game_mode: String,
    pub is_custom: bool,
    pub is_lobby_full: bool,
    pub is_team_builder_managed: bool,
    pub map_id: i32,
    pub max_human_players: i32,
    pub max_lobby_size: i32,
    pub max_team_size: i32,
    pub pick_type: String,
    pub premade_size_allowed: bool,
    pub queue_id: i32,
    pub should_force_scarce_position_selection: bool,
    pub show_position_selector: bool,
    pub show_quick_play_slot_selection: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomTeam100 {
    pub allowed_change_activity: bool,
    pub allowed_invite_others: bool,
    pub allowed_kick_others: bool,
    pub allowed_start_activity: bool,
    pub allowed_toggle_invite: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub bot_champion_id: i32,
    pub bot_difficulty: String,
    pub bot_id: String,
    pub first_position_preference: String,
    pub is_bot: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub puuid: String,
    pub ready: bool,
    pub second_position_preference: String,
    pub show_ghosted_banner: bool,
    pub summoner_icon_id: i32,
    pub summoner_id: i64,
    pub summoner_internal_name: String,
    pub summoner_level: i32,
    pub summoner_name: String,
    pub team_id: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AvailableBot {
    pub active: bool,
    pub bot_difficulties: Vec<String>,
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EogStatus {
    pub eog_players: Vec<String>,
    pub left_players: Vec<String>,
    pub party_size: i32,
    pub ready_players: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueueEligibility {
    pub eligible: bool,
    pub queue_id: i32,
    pub restrictions: Vec<Restriction>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Restriction {
    pub expired_timestamp: i64,
    pub restriction_args: RestrictionArgs,
    pub restriction_code: String,
    pub summoner_ids: Vec<serde_json::Value>, // any[]
    pub summoner_ids_string: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RestrictionArgs; // 空结构体（原定义为空）

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReceivedInvitation {
    pub can_accept_invitation: bool,
    pub from_summoner_id: i64,
    pub from_summoner_name: String,
    pub game_config: InvitationGameConfig,
    pub invitation_id: String,
    pub invitation_type: String,
    pub restrictions: Vec<serde_json::Value>, // any[]
    pub state: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvitationGameConfig {
    pub game_mode: String,
    pub invite_game_type: String,
    pub map_id: i32,
    pub queue_id: i32,
}