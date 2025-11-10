use crate::shared::init::game_data::get_champion_info_cache;
use crate::shared::init::game_data::get_item_info_cache;
use crate::shared::init::game_data::get_perk_info_cache;
use crate::shared::init::game_data::get_perk_style_info_cache;
use crate::shared::init::game_data::get_spell_info_cache;
use crate::shared::init::sgp::get_sgp_client;
use crate::shared::types::sgp::history::{Games, Participant as SgpParticipant};
use futures::future::join_all;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecordItem {
    pub game_id: String,
    pub puuid: String,
    pub game_creation: i64,
    pub duration: i32,
    pub queue_id: i32,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub puuid: String,
    pub name: String,
    pub tag: String,
    pub team_id: i32,
    pub win: bool,
    pub lane: String,
    pub best: bool,

    pub champion: Item,
    pub spells: Vec<Item>,
    pub perks: Vec<Item>,
    pub items: Vec<Item>,

    pub damage_to_turrets: i64,
    pub damage_to_turrets_percentage: f64,
    pub damage_to_champions: i64,
    pub damage_to_champions_percentage: f64,
    pub damage_taken: i64,
    pub damage_taken_percentage: f64,
    pub heal: i64,
    pub heal_percentage: f64,

    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub kda: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: i64,
    pub name: String,
}

pub async fn get_record_list(
    puuid: &str,
    beg_index: i32,
    end_index: i32,
) -> Result<Vec<RecordItem>, String> {
    let client = match get_sgp_client().await {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    let match_history = client
        .get_match_history(puuid, beg_index, end_index - beg_index + 1)
        .await
        .expect("获取比赛历史失败");

    let futures: Vec<_> = match_history
        .games
        .iter()
        .map(|game| parse_game(game, puuid))
        .collect();

    let record_list = join_all(futures).await;
    Ok(record_list)
}

pub async fn parse_game(game: &Games, puuid: &str) -> RecordItem {
    let mut record = RecordItem::default();
    record.game_id = game.metadata.match_id.clone();
    record.puuid = puuid.to_string();
    record.duration = game.json.game_duration as i32;
    record.queue_id = game.json.queue_id as i32;
    record.game_creation = game.json.game_creation;

    let participants = join_all(
        game.json
            .participants
            .iter()
            .map(|sgp_participant| parse_participant(sgp_participant)),
    )
    .await;
    record.participants = participants;

    // TODO: 计算 best

    // 计算 percentage
    // 根据 team_id 计算 total damage to turrets, damage to champions, damage taken, heal
    let mut team1_total_damage_to_turrets = 0;
    let mut team1_total_damage_to_champions = 0;
    let mut team1_total_damage_taken = 0;
    let mut team1_total_heal = 0;
    let mut team2_total_damage_to_turrets = 0;
    let mut team2_total_damage_to_champions = 0;
    let mut team2_total_damage_taken = 0;
    let mut team2_total_heal = 0;

    for participant in &record.participants {
        if participant.team_id == 100 {
            team1_total_damage_to_turrets += participant.damage_to_turrets;
            team1_total_damage_to_champions += participant.damage_to_champions;
            team1_total_damage_taken += participant.damage_taken;
            team1_total_heal += participant.heal;
        } else if participant.team_id == 200 {
            team2_total_damage_to_turrets += participant.damage_to_turrets;
            team2_total_damage_to_champions += participant.damage_to_champions;
            team2_total_damage_taken += participant.damage_taken;
            team2_total_heal += participant.heal;
        } else {
            panic!("team_id 不是 100 或 200");
        }
    }

    for participant in &mut record.participants {
        if participant.team_id == 100 {
            participant.damage_to_turrets_percentage = if team1_total_damage_to_turrets > 0 {
                participant.damage_to_turrets as f64 / team1_total_damage_to_turrets as f64
            } else {
                0.0
            };
            participant.damage_to_champions_percentage = if team1_total_damage_to_champions > 0 {
                participant.damage_to_champions as f64 / team1_total_damage_to_champions as f64
            } else {
                0.0
            };
            participant.damage_taken_percentage = if team1_total_damage_taken > 0 {
                participant.damage_taken as f64 / team1_total_damage_taken as f64
            } else {
                0.0
            };
            participant.heal_percentage = if team1_total_heal > 0 {
                participant.heal as f64 / team1_total_heal as f64
            } else {
                0.0
            };
        } else if participant.team_id == 200 {
            participant.damage_to_turrets_percentage = if team2_total_damage_to_turrets > 0 {
                participant.damage_to_turrets as f64 / team2_total_damage_to_turrets as f64
            } else {
                0.0
            };
            participant.damage_to_champions_percentage = if team2_total_damage_to_champions > 0 {
                participant.damage_to_champions as f64 / team2_total_damage_to_champions as f64
            } else {
                0.0
            };
            participant.damage_taken_percentage = if team2_total_damage_taken > 0 {
                participant.damage_taken as f64 / team2_total_damage_taken as f64
            } else {
                0.0
            };
            participant.heal_percentage = if team2_total_heal > 0 {
                participant.heal as f64 / team2_total_heal as f64
            } else {
                0.0
            };
        }
    }

    record
}

pub async fn parse_participant(sgp_participant: &SgpParticipant) -> Participant {
    let champion_info = get_champion_info_cache().await;
    let item_info = get_item_info_cache().await;
    let spell_info = get_spell_info_cache().await;
    let perk_info = get_perk_info_cache().await;
    let perk_style_info = get_perk_style_info_cache().await;

    // 辅助函数：安全获取物品名称，如果不存在则返回默认值
    let get_item_name = |item_id: i64| -> String {
        if item_id == 0 {
            return "无装备".to_string(); // 空物品槽位
        }
        item_info
            .get(&item_id)
            .map(|item| item.name.clone())
            .unwrap_or_else(|| format!("未知物品({})", item_id))
    };

    // 辅助函数：安全获取英雄名称
    let get_champion_name = |champion_id: i64| -> String {
        champion_info
            .get(&champion_id)
            .map(|champion| champion.name.clone())
            .unwrap_or_else(|| format!("未知英雄({})", champion_id))
    };

    // 辅助函数：安全获取技能名称
    let get_spell_name = |spell_id: i64| -> String {
        spell_info
            .get(&spell_id)
            .map(|spell| spell.name.clone())
            .unwrap_or_else(|| format!("未知技能({})", spell_id))
    };

    // 辅助函数：安全获取符文名称
    let get_perk_name = |perk_id: i64| -> String {
        perk_info
            .get(&perk_id)
            .map(|perk| perk.name.clone())
            .unwrap_or_else(|| format!("未知符文({})", perk_id))
    };

    // 辅助函数：安全获取符文风格名称
    let get_perk_style_name = |style_id: i64| -> String {
        perk_style_info
            .get(&style_id)
            .map(|style| style.name.clone())
            .unwrap_or_else(|| format!("未知符文风格({})", style_id))
    };

    let mut participant = Participant::default();
    participant.puuid = sgp_participant.puuid.clone();
    participant.name = sgp_participant.riot_id_game_name.clone();
    participant.tag = sgp_participant.riot_id_tagline.clone();
    participant.team_id = sgp_participant.team_id as i32;
    participant.win = sgp_participant.win;
    participant.lane = sgp_participant.lane.clone();
    // best 在这里无法判断，需要在外面判断
    participant.champion = Item {
        id: sgp_participant.champion_id,
        name: get_champion_name(sgp_participant.champion_id),
    };
    participant.spells = vec![
        Item {
            id: sgp_participant.spell1id,
            name: get_spell_name(sgp_participant.spell1id),
        },
        Item {
            id: sgp_participant.spell2id,
            name: get_spell_name(sgp_participant.spell2id),
        },
    ];

    participant.perks = vec![
        Item {
            id: sgp_participant.perks.styles[0].selections[0].perk,
            name: get_perk_name(sgp_participant.perks.styles[0].selections[0].perk),
        },
        Item {
            id: sgp_participant.perks.styles[1].selections[0].perk,
            name: get_perk_style_name(sgp_participant.perks.styles[1].selections[0].perk),
        },
    ];

    participant.items = vec![
        Item {
            id: sgp_participant.item0,
            name: get_item_name(sgp_participant.item0),
        },
        Item {
            id: sgp_participant.item1,
            name: get_item_name(sgp_participant.item1),
        },
        Item {
            id: sgp_participant.item2,
            name: get_item_name(sgp_participant.item2),
        },
        Item {
            id: sgp_participant.item3,
            name: get_item_name(sgp_participant.item3),
        },
        Item {
            id: sgp_participant.item4,
            name: get_item_name(sgp_participant.item4),
        },
        Item {
            id: sgp_participant.item5,
            name: get_item_name(sgp_participant.item5),
        },
        Item {
            id: sgp_participant.item6,
            name: get_item_name(sgp_participant.item6),
        },
    ];

    participant.damage_to_turrets = sgp_participant.damage_dealt_to_turrets;
    participant.damage_to_champions = sgp_participant.total_damage_dealt_to_champions;
    participant.damage_taken = sgp_participant.total_damage_taken;
    participant.heal = sgp_participant.total_heal;
    // percentage 需要后续计算

    participant.kills = sgp_participant.kills;
    participant.deaths = sgp_participant.deaths;
    participant.assists = sgp_participant.assists;

    if participant.deaths == 0 {
        participant.kda = (participant.kills + participant.assists) as f64 / 1.0;
    } else {
        participant.kda =
            (participant.kills + participant.assists) as f64 / participant.deaths as f64;
    }

    participant
}
