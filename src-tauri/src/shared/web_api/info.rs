use crate::shared::init::lcu::get_lcu_client;
use crate::shared::{http_api::lcu::LcuApi, init::game_data::get_champion_info_cache};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub puuid: String,
    pub game_name: String,
    pub game_level: i64,
    pub profile_icon_id: i64,
    pub highest_rank: RankInfo,
    pub solo_rank: RankInfo,
    pub flex_rank: RankInfo,
    pub favorite_heroes: Vec<FavoriteHero>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RankInfo {
    pub rank: String,
    pub division: String,
    pub lp: Option<i64>,
    pub wins: Option<i64>,
    pub losses: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteHero {
    pub champion_id: i64,
    pub champion_name: String,
    pub matches: i64,
}

pub async fn get_summoner_info(client: &LcuApi, info: &mut Info, puuid: &str) {
    let summoner_info = client.summoner.get_summoner_by_puuid(puuid).await.unwrap();
    info.puuid = summoner_info.puuid;
    info.game_name = format!("{}#{}", summoner_info.game_name, summoner_info.tag_line);
    info.game_level = summoner_info.summoner_level;
    info.profile_icon_id = summoner_info.profile_icon_id;
}

/// must make sure the client is initialized
pub async fn get_rank_info(client: &LcuApi, info: &mut Info, puuid: &str) {
    let ranked = client.ranked.get_ranked_stats(puuid).await.unwrap();
    info.highest_rank = RankInfo {
        rank: ranked.highest_ranked_entry.highest_tier,
        division: ranked.highest_ranked_entry.highest_division,
        lp: Some(ranked.highest_ranked_entry.league_points),
        wins: Some(ranked.highest_ranked_entry.wins),
        losses: Some(ranked.highest_ranked_entry.losses),
    };
    info.solo_rank = RankInfo {
        rank: ranked.queue_map.ranked_solo_5x5.tier,
        division: ranked.queue_map.ranked_solo_5x5.division,
        lp: Some(ranked.queue_map.ranked_solo_5x5.league_points),
        wins: Some(ranked.queue_map.ranked_solo_5x5.wins),
        losses: Some(ranked.queue_map.ranked_solo_5x5.losses),
    };
    info.flex_rank = RankInfo {
        rank: ranked.queue_map.ranked_flex_sr.tier,
        division: ranked.queue_map.ranked_flex_sr.division,
        lp: Some(ranked.queue_map.ranked_flex_sr.league_points),
        wins: Some(ranked.queue_map.ranked_flex_sr.wins),
        losses: Some(ranked.queue_map.ranked_flex_sr.losses),
    };
}

pub async fn get_champion_mastery_info(client: &LcuApi, info: &mut Info, puuid: &str) {
    let champion_info_cache = get_champion_info_cache().await;
    let champion_mastery = client
        .champion_mastery
        .get_player_champion_mastery(puuid)
        .await
        .unwrap();

    for mastery in champion_mastery {
        info.favorite_heroes.push(FavoriteHero {
            champion_id: mastery.champion_id,
            champion_name: champion_info_cache
                .get(&mastery.champion_id)
                .unwrap()
                .clone()
                .name,
            matches: mastery.champion_points,
        });
    }
}

pub async fn get_info(puuid: &str) -> Result<Info, String> {
    let client = get_lcu_client().await.unwrap();
    let mut info = Info::default();
    get_summoner_info(&client, &mut info, puuid).await;
    get_rank_info(&client, &mut info, puuid).await;
    get_champion_mastery_info(&client, &mut info, puuid).await;
    Ok(info)
}
