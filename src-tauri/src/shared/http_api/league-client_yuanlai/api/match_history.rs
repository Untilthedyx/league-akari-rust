use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::{sync::LazyLock, time::Duration};

use crate::lcu::api::game_detail::GameDetail;
use crate::lcu::api::model::{Participant, ParticipantIdentity};
use crate::lcu::https::lcu_get;
use crate::lcu::{constant::get_queue_id_to_cn};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Game {
    #[serde(rename = "mvp", default)] // 计算出的是否是本局的MVp
    pub mvp: String,
    #[serde(rename = "queueName", default)]
    pub queue_name: String, // 中文名，对queueId的中文翻译

    #[serde(rename = "gameDetail", default)]
    pub game_detail: GameDetail,
    #[serde(rename = "gameId")]
    pub game_id: i64,
    #[serde(rename = "gameCreationDate")]
    pub game_creation_date: String,
    #[serde(rename = "gameDuration")]
    pub game_duration: i32,
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameType")]
    pub game_type: String,
    #[serde(rename = "mapId")]
    pub map_id: i32,
    #[serde(rename = "queueId")]
    pub queue_id: i32,

    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "participantIdentities")]
    pub participant_identities: Vec<ParticipantIdentity>,
    pub participants: Vec<Participant>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GamesWrapper {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MatchHistory {
    #[serde(rename = "platformId")]
    pub platform_id: String,
    #[serde(rename = "begIndex", default)] // 手动添加的字段，便于后续筛序
    pub beg_index: i32,
    #[serde(rename = "endIndex", default)] // 手动添加的字段，便于后续筛序
    pub end_index: i32,
    pub games: GamesWrapper,
}

impl MatchHistory {
    /// https://riot:6YY78o1__fXs6enCsdleeQ@127.0.0.1:49991/lol-match-history/v1/products/lol/55cc79c4-3d20-535a-9bff-00b1867534d8/matches?begIndex=0&endIndex=20
    pub async fn get_current_match_history(
        begin_index: i32,
        end_index: i32,
    ) -> Result<Self, String> {
        let uri = format!(
            "lol-match-history/v1/products/lol/me/matches?beginIndex={}%26endIndex={}",
            begin_index, end_index
        );

        let match_history = lcu_get::<Self>(&uri).await?;
        Ok(match_history)
    }

    /// 在这里不使用缓存，原来的逻辑太乱了
    pub async fn get_match_history_by_puuid(
        puuid: &str,
        begin_index: i32,
        end_index: i32,
    ) -> Result<Self, String> {
        let uri = format!(
            "lol-match-history/v1/products/lol/{}/matches?begIndex={}&endIndex={}",
            puuid, begin_index, end_index
        );
        let match_history = lcu_get::<Self>(&uri).await?;
        Ok(match_history)
    }

    pub async fn enrich_game_detail(&mut self) {
        for game in &mut self.games.games {
            game.game_detail = GameDetail::get_game_detail_by_id(&game.game_id)
                .await
                .unwrap();
            game.queue_name = match get_queue_id_to_cn(game.queue_id as u32) {
                Some(s) => s.into(),
                None => "未知".to_string(),
            };
        }
    }

    pub fn calculate(&mut self) {
        for game in &mut self.games.games {
            if game.participants.is_empty() || game.game_detail.participants.is_empty() {
                continue;
            }

            let team_id = game.participants[0].team_id;
            let mut total_gold_earned: i64 = 0;
            let mut total_damage_dealt_to_champions: i64 = 0;
            let mut total_damage_taken: i64 = 0;
            let mut total_heal: i64 = 0;

            for p in &game.game_detail.participants {
                if p.team_id == team_id {
                    total_gold_earned += p.stats.gold_earned as i64;
                    total_damage_dealt_to_champions +=
                        p.stats.total_damage_dealt_to_champions as i64;
                    total_damage_taken += p.stats.total_damage_taken as i64;
                    total_heal += p.stats.total_heal as i64;
                }
            }

            if total_gold_earned == 0 {
                total_gold_earned = 1;
            }
            if total_damage_dealt_to_champions == 0 {
                total_damage_dealt_to_champions = 1;
            }
            if total_damage_taken == 0 {
                total_damage_taken = 1;
            }
            if total_heal == 0 {
                total_heal = 1;
            }

            let my_stats = &mut game.participants[0].stats;
            let my_gold = my_stats.gold_earned as f64;
            let my_damage_dealt = my_stats.total_damage_dealt_to_champions as f64;
            let my_damage_taken = my_stats.total_damage_taken as f64;
            let my_heal = my_stats.total_heal as f64;

            my_stats.gold_earned_rate = ((my_gold / total_gold_earned as f64) * 100.0) as i32;
            my_stats.damage_dealt_to_champions_rate =
                ((my_damage_dealt / total_damage_dealt_to_champions as f64) * 100.0) as i32;
            my_stats.damage_taken_rate =
                ((my_damage_taken / total_damage_taken as f64) * 100.0) as i32;
            my_stats.heal_rate = ((my_heal / total_heal as f64) * 100.0) as i32;
        }
    }
}

/// TODO
static MATCH_HISTORY_CACHE: LazyLock<Cache<String, MatchHistory>> = LazyLock::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(60))
        .max_capacity(50)
        .build()
});
