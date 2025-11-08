use crate::shared::init::lcu::get_lcu_client;
use crate::shared::types::league_client::match_history::Game;
use crate::shared::{
    http_api::lcu::LcuApi,
    types::league_client::match_history::{Participant, ParticipantIdentity},
};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

/// KDA 数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Kda {
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
}

/// 物品数据结构（装备或眼位）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub name: String,
}

/// 召唤师技能数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Spell {
    pub id: String,
    pub name: String,
}

/// 符文数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    pub id: String,
    pub name: String,
}

/// 玩家统计数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub damage: i32,             // 伤害数值
    pub damage_share: f64,       // 伤害占比
    pub damage_taken: i32,       // 抗伤数值
    pub damage_taken_share: f64, // 抗伤占比
    pub healing: i32,            // 治疗数值
    pub healing_share: f64,      // 治疗占比
}

/// 玩家信息（用于 champion、teammates、enemies）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    pub id: String,      // puuid
    pub name: String,    // 玩家名称 + tag
    pub hero: String,    // 英雄名称
    pub hero_id: String, // 英雄ID
    pub score: i32,      // 评分
    pub kda: Kda,
}

/// 战绩记录数据结构
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecordItem {
    pub id: String,
    pub champion: PlayerInfo,
    pub is_win: bool,       // 是否胜利
    pub is_best: bool,      // 是否是最佳
    pub date: String,       // 游戏时间（毫秒时间戳字符串）
    pub duration: i32,      // 游戏时长（秒）
    pub queue_id: i32,      // 队列ID，表示游戏模式
    pub items: Vec<Item>,   // 6装备 + 1个眼位
    pub spells: Vec<Spell>, // 2个召唤师技能
    pub perks: Vec<Perk>,   // 2个符文
    pub stats: PlayerStats,
    pub teammates: Vec<PlayerInfo>, // 队友列表 TODO: 换名字 蓝队
    pub enemies: Vec<PlayerInfo>,   // 敌方列表 TODO: 换名字 红队
}

pub async fn parse_game(client: &LcuApi, game: &Game, puuid: &str) -> RecordItem {
    fn parse_participant(
        participant: &Participant,
        pariticipant_identity: &ParticipantIdentity,
    ) -> PlayerInfo {
        PlayerInfo {
            id: pariticipant_identity.player.puuid.clone(),
            name: format!(
                "{}#{}",
                pariticipant_identity.player.game_name, pariticipant_identity.player.tag_line
            ),
            hero: participant.champion_id.to_string(),
            hero_id: participant.champion_id.to_string(),
            score: 0,
            kda: Kda {
                kills: participant.stats.kills,
                deaths: participant.stats.deaths,
                assists: participant.stats.assists,
            },
        }
    }

    let game_detail = client
        .match_history
        .get_game(game.game_id)
        .await
        .expect("获取游戏详情失败");

    let mut record = RecordItem::default();
    record.id = game.game_id.to_string();
    record.queue_id = game.queue_id;
    record.date = game_detail.game_creation.to_string();
    record.duration = game_detail.game_duration;

    // 先找到当前玩家的 team_id
    let mut current_player_team_id = None;
    for pariticipant_identity in &game_detail.participant_identities {
        if pariticipant_identity.player.puuid == puuid {
            let participant = game_detail
                .participants
                .iter()
                .find(|p| p.participant_id == pariticipant_identity.participant_id)
                .expect("获取玩家信息失败");
            current_player_team_id = Some(participant.team_id);
            break;
        }
    }

    let current_team_id = match current_player_team_id {
        Some(id) => id,
        None => {
            // 如果找不到当前玩家，返回空记录
            return record;
        }
    };

    // 累加同队玩家的总数据（用于计算占比）
    let mut team_total_damage = 0.0;
    let mut team_total_damage_taken = 0.0;
    let mut team_total_healing = 0.0;

    for pariticipant_identity in &game_detail.participant_identities {
        let participant = game_detail
            .participants
            .iter()
            .find(|p| p.participant_id == pariticipant_identity.participant_id)
            .expect("获取玩家信息失败");

        // 只累加同队玩家的数据
        if participant.team_id == current_team_id {
            team_total_damage += participant.stats.total_damage_dealt_to_champions as f64;
            team_total_damage_taken += participant.stats.total_damage_taken as f64;
            team_total_healing += participant.stats.total_heal as f64;
        }

        if pariticipant_identity.player.puuid == puuid {
            record.champion = parse_participant(participant, pariticipant_identity);
            record.is_win = participant.stats.win;
            record.is_best = false; // TODO: 计算是否是最佳
            record.items = vec![
                Item {
                    id: participant.stats.item0.to_string(),
                    name: participant.stats.item0.to_string(),
                },
                Item {
                    id: participant.stats.item1.to_string(),
                    name: participant.stats.item1.to_string(),
                },
                Item {
                    id: participant.stats.item2.to_string(),
                    name: participant.stats.item2.to_string(),
                },
                Item {
                    id: participant.stats.item3.to_string(),
                    name: participant.stats.item3.to_string(),
                },
                Item {
                    id: participant.stats.item4.to_string(),
                    name: participant.stats.item4.to_string(),
                },
                Item {
                    id: participant.stats.item5.to_string(),
                    name: participant.stats.item5.to_string(),
                },
                Item {
                    id: participant.stats.item6.to_string(),
                    name: participant.stats.item6.to_string(),
                },
            ];
            record.spells = vec![
                Spell {
                    id: participant.spell1_id.to_string(),
                    name: participant.spell1_id.to_string(),
                },
                Spell {
                    id: participant.spell2_id.to_string(),
                    name: participant.spell2_id.to_string(),
                },
            ];
            record.perks = vec![
                Perk {
                    id: participant.stats.perk0.to_string(),
                    name: participant.stats.perk0.to_string(),
                },
                Perk {
                    id: participant.stats.perk1.to_string(),
                    name: participant.stats.perk1.to_string(),
                },
            ];
            // 当前玩家的数据
            record.stats.damage = participant.stats.total_damage_dealt_to_champions;
            record.stats.damage_taken = participant.stats.total_damage_taken;
            record.stats.healing = participant.stats.total_heal;
        }

        // 分类队友和敌人
        if participant.team_id == 100 {
            record
                .teammates
                .push(parse_participant(participant, pariticipant_identity));
        } else {
            record
                .enemies
                .push(parse_participant(participant, pariticipant_identity));
        }
    }

    // 计算占比：当前玩家数值 / 同队总数值（0~1之间的小数）
    if team_total_damage > 0.0 {
        record.stats.damage_share = record.stats.damage as f64 / team_total_damage;
    } else {
        record.stats.damage_share = 0.0;
    }

    if team_total_damage_taken > 0.0 {
        record.stats.damage_taken_share =
            record.stats.damage_taken as f64 / team_total_damage_taken;
    } else {
        record.stats.damage_taken_share = 0.0;
    }

    if team_total_healing > 0.0 {
        record.stats.healing_share = record.stats.healing as f64 / team_total_healing;
    } else {
        record.stats.healing_share = 0.0;
    }

    record
}

pub async fn get_record_list(
    puuid: &str,
    beg_index: Option<i32>,
    end_index: Option<i32>,
) -> Result<Vec<RecordItem>, String> {
    let client = match get_lcu_client().await {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    let match_history = client
        .match_history
        .get_match_history(puuid, beg_index, end_index)
        .await
        .expect("获取比赛历史失败");

    let futures: Vec<_> = match_history
        .games
        .games
        .iter()
        .map(|game| parse_game(&client, game, puuid))
        .collect();

    let record_list = join_all(futures).await;

    Ok(record_list)
}
