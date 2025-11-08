use crate::shared::{
    http_api::lcu::http::HttpClient,
    types::league_client::game_data::*, // 导入游戏数据相关类型
};
use crate::utils::error::http_error::HttpError;
use tracing::instrument;

/// 游戏数据 HTTP API 封装
/// 提供游戏基础数据的查询接口，包括英雄、物品、符文、地图等信息
#[derive(Debug, Clone)]
pub struct GameDataHttpApi {
    /// 内部使用的 HTTP 客户端实例
    client: HttpClient,
}

impl GameDataHttpApi {
    /// 创建 GameDataHttpApi 实例
    /// - 参数: `client` - 预配置的 HttpClient 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取所有召唤师技能列表
    /// - 返回: 召唤师技能数组（Vec<SummonerSpell>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_summoner_spells(&self) -> Result<Vec<SummonerSpell>, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/summoner-spells.json")
            .await
    }

    /// 获取符文风格配置
    /// - 返回: 符文风格数据（Perkstyles）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_perkstyles(&self) -> Result<Perkstyles, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/perkstyles.json")
            .await
    }

    /// 获取所有物品列表
    /// - 返回: 物品数组（Vec<Item>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_items(&self) -> Result<Vec<Item>, HttpError> {
        self.client.get("/lol-game-data/assets/v1/items.json").await
    }

    /// 获取英雄简要信息列表
    /// - 返回: 英雄简要信息数组（Vec<ChampionSimple>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_champion_summary(&self) -> Result<Vec<ChampionSimple>, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/champion-summary.json")
            .await
    }

    /// 获取所有地图信息
    /// - 返回: 地图信息数组（Vec<GameMap>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_maps(&self) -> Result<Vec<GameMap>, HttpError> {
        self.client.get("/lol-game-data/assets/v1/maps.json").await
    }

    /// 获取所有符文详情
    /// - 返回: 符文数组（Vec<Perk>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_perks(&self) -> Result<Vec<Perk>, HttpError> {
        self.client.get("/lol-game-data/assets/v1/perks.json").await
    }

    /// 获取所有队列信息
    /// - 返回: 队列信息数组（Vec<Queue>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_queues(&self) -> Result<Vec<Queue>, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/queues.json")
            .await
    }

    /// 获取地图资源配置
    /// - 返回: 地图资源数据（GameMapAsset）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_map_assets(&self) -> Result<GameMapAsset, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/map-assets/map-assets.json")
            .await
    }

    /// 获取指定英雄的详细信息
    /// - 参数: `champ_id` - 英雄唯一标识符
    /// - 返回: 英雄详细信息（ChampDetails）或 HTTP 错误
    #[instrument(skip_all, fields(champ_id = champ_id))]
    pub async fn get_champ_details(&self, champ_id: u32) -> Result<ChampDetails, HttpError> {
        let url = format!("/lol-game-data/assets/v1/champions/{}.json", champ_id);
        self.client.get(&url).await
    }

    /// 获取斗魂竞技场（Arena）强化符文列表
    /// - 返回: 强化符文数组（Vec<Augment>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_augments(&self) -> Result<Vec<Augment>, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/cherry-augments.json")
            .await
    }

    /// 获取无尽狂潮（Swarm）中心数据
    /// - 返回: 无尽狂潮数据数组（Vec<StrawberryHub>）或 HTTP 错误
    #[instrument(skip_all)]
    pub async fn get_strawberry_hub(&self) -> Result<Vec<StrawberryHub>, HttpError> {
        self.client
            .get("/lol-game-data/assets/v1/strawberry-hub.json")
            .await
    }
}
