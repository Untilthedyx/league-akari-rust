use crate::shared::types::league_client::champ_select::{
    ChampSelectSession, ChampSelectSummoner, OngoingTrade,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ChampSelectState {
    session: Option<ChampSelectSession>,
    current_champion: Option<u32>,
    current_pickable_champion_id_array: Vec<u32>,
    current_bannable_champion_id_array: Vec<u32>,
    disabled_champion_id_array: Vec<u32>,
    self_summoner: Option<ChampSelectSummoner>,
    ongoing_trade: Option<OngoingTrade>,
}

#[derive(Debug, Default)]
pub struct ChampSelectStateLock {
    pub state: RwLock<ChampSelectState>,
}

impl ChampSelectStateLock {
    pub fn new() -> Self {
        Self::default()
    }

    // 获取当前可选英雄ID的集合
    pub async fn current_pickable_champion_ids(&self) -> HashSet<u32> {
        self.state
            .read()
            .await
            .current_pickable_champion_id_array
            .iter()
            .cloned()
            .collect()
    }

    // 获取当前可禁用英雄ID的集合
    pub async fn current_bannable_champion_ids(&self) -> HashSet<u32> {
        self.state
            .read()
            .await
            .current_bannable_champion_id_array
            .iter()
            .cloned()
            .collect()
    }

    // 获取已禁用英雄ID的集合
    pub async fn disabled_champion_ids(&self) -> HashSet<u32> {
        self.state
            .read()
            .await
            .disabled_champion_id_array
            .iter()
            .cloned()
            .collect()
    }

    // 设置会话信息
    pub async fn set_session(&mut self, session: Option<ChampSelectSession>) {
        self.state.write().await.session = session;
    }

    // 设置当前可选英雄数组
    pub async fn set_current_pickable_champion_array(&mut self, array: Vec<u32>) {
        self.state.write().await.current_pickable_champion_id_array = array;
    }

    // 设置当前可禁用英雄数组
    pub async fn set_current_bannable_champion_array(&mut self, array: Vec<u32>) {
        self.state.write().await.current_bannable_champion_id_array = array;
    }

    // 设置自身召唤师信息
    pub async fn set_self_summoner(&mut self, summoner: Option<ChampSelectSummoner>) {
        self.state.write().await.self_summoner = summoner;
    }

    // 设置当前选择的英雄
    pub async fn set_current_champion(&mut self, champion: Option<u32>) {
        self.state.write().await.current_champion = champion;
    }

    // 设置已禁用英雄ID数组
    pub async fn set_disabled_champion_ids(&mut self, ids: Vec<u32>) {
        self.state.write().await.disabled_champion_id_array = ids;
    }

    // 设置正在进行的交易
    pub async fn set_ongoing_trade(&mut self, trade: Option<OngoingTrade>) {
        self.state.write().await.ongoing_trade = trade;
    }
}
