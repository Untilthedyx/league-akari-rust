/// 获取对局信息
use serde::{Deserialize, Serialize};

use crate::lcu::https::lcu_get;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub champion_id: i32,
    pub puuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    #[serde(rename = "type")] // 'type' is a Rust keyword, so explicitly rename
    pub queue_type: String,
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub game_id: i64,
    pub is_custom_game: bool,
    pub queue: Queue,
    pub team_one: Vec<Player>,
    pub team_two: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub game_data: GameData,
    pub phase: String,
}

impl Session {
    pub async fn get_session() -> Result<Self, String> {
        let uri = format!("lol-gameflow/v1/session");
        let session = lcu_get::<Self>(&uri).await?;
        Ok(session)
    }
}
