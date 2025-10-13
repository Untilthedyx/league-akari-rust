/// 获取召唤师信息

use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

use crate::lcu::https::lcu_get;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Summoner {
    pub game_name: String,
    pub tag_line: String,
    pub summoner_id: String,
    pub profile_icon_id: i32,
    pub puuid: String,
}

/// 缓存 summoner
pub static SUMMONER_PUUID_CACHE: LazyLock<Cache<String, Summoner>> =
    LazyLock::new(|| Cache::builder().max_capacity(500).build());

static SUMMONER_NAME_CACHE: LazyLock<Cache<String, Summoner>> =
    LazyLock::new(|| Cache::builder().max_capacity(500).build());

impl Summoner {
    /// 通过 puuid 获取 summoner 信息
    pub async fn get_summoner_by_puuid(puuid: &str) -> Result<Self, String> {
        if let Some(summoner) = SUMMONER_PUUID_CACHE.get(puuid).await {
            return Ok(summoner.clone());
        }

        let uri = format!("lol-summoner/v2/summoners/puuid/{}", puuid);
        let summoner = lcu_get::<Self>(&uri).await?;
        SUMMONER_PUUID_CACHE
            .insert(puuid.to_string(), summoner.clone())
            .await;
        Ok(summoner)
    }

    /// 通过 name 获取 summoner 信息
    pub async fn get_summoner_by_name(name: &str) -> Result<Self, String> {
        let url_encoding = urlencoding::encode(name);
        if let Some(summoner) = SUMMONER_NAME_CACHE.get(name).await {
            return Ok(summoner.clone());
        }
        let uri = format!("lol-summoner/v1/summoners/by-name/{}", url_encoding);
        let summoner = lcu_get::<Self>(&uri).await?;
        SUMMONER_NAME_CACHE
            .insert(name.to_string(), summoner.clone())
            .await;
        Ok(summoner)
    }

    /// 获取当前 summoner 信息
    pub async fn get_current_summoner() -> Result<Self, String> {
        let uri = format!("lol-summoner/v1/current-summoner");
        let summoner = lcu_get::<Self>(&uri).await?;
        Ok(summoner)
    }
}
