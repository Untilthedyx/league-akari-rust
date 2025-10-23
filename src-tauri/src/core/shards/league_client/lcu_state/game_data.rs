use crate::shared::types::league_client::game_data::*;
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::RwLock;

#[derive(Default, Debug, Clone, Serialize)]
pub struct GameDataState {
    pub summoner_spells: HashMap<i32, SummonerSpell>,
    pub items: HashMap<i32, Item>,
    pub queues: HashMap<i32, Queue>,
    pub perks: HashMap<i32, Perk>,
    pub perk_styles: PerkStylesData,
    pub augments: HashMap<i32, Augment>,
    pub champions: HashMap<i32, ChampionSimple>,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct PerkStylesData {
    pub schema_version: i32,
    pub styles: HashMap<i32, Perkstyles>,
}

// 应用状态包装
#[derive(Default, Debug)]
pub struct GameDataStateLock {
    state: RwLock<GameDataState>,
}

impl GameDataStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    // 获取冠军名称
    pub async fn champion_name(&self, id: i32) -> String {
        let state = self.state.write().await;
        state
            .champions
            .get(&id)
            .map(|champ| champ.name.clone())
            .unwrap_or_else(|| id.to_string())
    }

    // 设置召唤师技能
    pub async fn set_summoner_spells(&self, value: HashMap<i32, SummonerSpell>) {
        let mut state = self.state.write().await;
        state.summoner_spells = value.clone();
    }

    // 设置物品
    pub async fn set_items(&self, value: HashMap<i32, Item>) {
        let mut state = self.state.write().await;
        state.items = value.clone();
    }

    // 设置队列
    pub async fn set_queues(&self, value: HashMap<i32, Queue>) {
        let mut state = self.state.write().await;
        state.queues = value.clone();
    }

    // 设置符文
    pub async fn set_perks(&self, value: HashMap<i32, Perk>) {
        let mut state = self.state.write().await;
        state.perks = value.clone();
    }

    // 设置符文样式
    pub async fn set_perk_styles(&self, value: PerkStylesData) {
        let mut state = self.state.write().await;
        state.perk_styles = value.clone();
    }

    // 设置强化
    pub async fn set_augments(&self, value: HashMap<i32, Augment>) {
        let mut state = self.state.write().await;
        state.augments = value.clone();
    }

    // 设置英雄
    pub async fn set_champions(&self, value: HashMap<i32, ChampionSimple>) {
        let mut state = self.state.write().await;
        state.champions = value.clone();
    }

    // 获取完整状态（用于前端初始化）
    pub async fn get_full_state(&self) -> GameDataState {
        self.state.write().await.clone()
    }
}
