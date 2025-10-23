use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailabilityType {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "dnd")]
    Dnd,
    #[serde(rename = "away")]
    Away,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "spectating")]
    Spectating,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub body: String,
    pub from_id: String,
    pub from_obfuscated_summoner_id: u64,
    pub from_pid: String,
    pub from_summoner_id: u64,
    pub id: String,
    pub is_historical: bool,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String, // 可能的值: information | chat | groupchat | celebration | system
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatPerson {
    pub availability: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: u32,
    pub id: String,
    pub last_seen_online_timestamp: Option<serde_json::Value>, // 可能为任意类型或 null
    pub lol: LOL,
    pub name: String,
    pub obfuscated_summoner_id: u64,
    pub patchline: String,
    pub pid: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub puuid: String,
    pub status_message: String,
    pub summary: String,
    pub summoner_id: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LOL {
    pub banner_id_selected: String,
    pub challenge_crystal_level: String,
    pub challenge_title_selected: String,
    pub challenge_tokens_selected: String,
    pub champion_id: String,
    pub companion_id: String,
    pub damage_skin_id: String,
    pub game_id: String,
    pub game_mode: String,
    pub game_queue_type: String,
    pub game_status: String,
    pub icon_override: String,
    pub init_rank_stat: String,
    pub init_summoner: String,
    pub is_observable: String,
    pub map_id: String,
    pub map_skin_id: String,
    pub pty: String,
    pub queue_id: String,
    pub regalia: String,
    pub skin_variant: String,
    pub skinname: String,
    pub time_stamp: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    pub game_name: String,
    pub game_tag: String,
    pub id: String,
    pub inviter_id: String,
    pub is_muted: bool,
    pub last_message: Option<serde_json::Value>, // 可能为任意类型或 null
    pub multi_user_chat_jwt: String,
    pub name: String,
    pub password: String,
    pub pid: String,
    pub target_region: String,
    #[serde(rename = "type")]
    pub conv_type: String,
    pub unread_message_count: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatState {
    pub status_message: String,
    pub availability: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: u32,
    pub id: String,
    pub last_seen_online_timestamp: Option<serde_json::Value>, // 可能为任意类型或 null
    pub lol: ChatLol,
    pub name: String,
    pub obfuscated_summoner_id: u64,
    pub patchline: String,
    pub pid: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub puuid: String,
    pub summary: String,
    pub summoner_id: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatLol {
    pub banner_id_selected: String,
    pub challenge_crystal_level: String,
    pub challenge_title_selected: String,
    pub challenge_tokens_selected: String,
    pub champion_id: String,
    pub companion_id: String,
    pub damage_skin_id: String,
    pub game_id: String,
    pub game_mode: String,
    pub game_queue_type: String,
    pub game_status: String,
    pub icon_override: String,
    pub init_rank_stat: String,
    pub init_summoner: String,
    pub is_observable: String,
    pub map_id: String,
    pub map_skin_id: String,
    pub pty: String,
    pub queue_id: String,
    pub regalia: String,
    pub skin_variant: String,
    pub skinname: String,
    pub time_stamp: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Friend {
    pub availability: String,
    pub display_group_id: i32,
    pub display_group_name: String,
    pub game_name: String,
    pub game_tag: String,
    pub group_id: i32,
    pub group_name: String,
    pub icon: u32,
    pub id: String,
    pub is_p2_p_conversation_muted: bool,
    pub last_seen_online_timestamp: Option<serde_json::Value>, // 可能为任意类型或 null
    pub lol: Lol,
    pub name: String,
    pub note: String,
    pub patchline: String,
    pub pid: String,
    pub platform_id: String,
    pub product: String,
    pub product_name: String,
    pub puuid: String,
    pub status_message: String,
    pub summary: String,
    pub summoner_id: u64,
    pub time: u64,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lol {
    pub banner_id_selected: String,
    pub challenge_crystal_level: String,
    pub challenge_points: String,
    pub challenge_tokens_selected: String,
    pub champion_id: String,
    pub companion_id: String,
    pub damage_skin_id: String,
    pub game_id: String,
    pub game_mode: String,
    pub game_queue_type: String,
    pub game_status: String,
    pub icon_override: String,
    pub init_summoner: String,
    pub is_observable: String,
    pub map_id: String,
    pub map_skin_id: String,
    pub profile_icon: String,
    pub queue_id: String,
    pub regalia: String,
    pub skin_variant: String,
    pub skinname: String,
    pub time_stamp: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FriendGroup {
    pub collapsed: bool,
    pub id: i32,
    pub is_localized: bool,
    pub is_meta_group: bool,
    pub name: String,
    pub priority: i32,
}
