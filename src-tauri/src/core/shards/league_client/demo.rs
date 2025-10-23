use serde::Serialize;
use std::collections::HashMap;
use tauri::{State, Manager};
use std::sync::Mutex;

// 这些类型需要根据你的实际结构定义
#[derive(Clone, Serialize)]
pub struct SummonerSpell {
    // 字段定义
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Serialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    // 其他字段
}

#[derive(Clone, Serialize)]
pub struct Queue {
    pub id: i32,
    pub description: String,
    // 其他字段
}

#[derive(Clone, Serialize)]
pub struct Perk {
    pub id: i32,
    pub name: String,
    // 其他字段
}

#[derive(Clone, Serialize)]
pub struct PerkStyle {
    pub id: i32,
    pub name: String,
    // 其他字段
}

#[derive(Clone, Serialize)]
pub struct Augment {
    pub id: i32,
    pub name: String,
    // 其他字段
}

#[derive(Clone, Serialize)]
pub struct ChampionSimple {
    pub id: i32,
    pub name: String,
    // 其他字段
}

// 主状态结构
#[derive(Default, Clone, Serialize)]
pub struct GameDataState {
    pub summoner_spells: HashMap<i32, SummonerSpell>,
    pub items: HashMap<i32, Item>,
    pub queues: HashMap<i32, Queue>,
    pub perks: HashMap<i32, Perk>,
    pub perk_styles: PerkStylesData,
    pub augments: HashMap<i32, Augment>,
    pub champions: HashMap<i32, ChampionSimple>,
}

#[derive(Default, Clone, Serialize)]
pub struct PerkStylesData {
    pub schema_version: i32,
    pub styles: HashMap<i32, PerkStyle>,
}

// 事件类型
#[derive(Clone, Serialize)]
pub enum GameDataEvent {
    SummonerSpellsUpdated { data: HashMap<i32, SummonerSpell> },
    ItemsUpdated { data: HashMap<i32, Item> },
    QueuesUpdated { data: HashMap<i32, Queue> },
    PerksUpdated { data: HashMap<i32, Perk> },
    PerkStylesUpdated { data: PerkStylesData },
    AugmentsUpdated { data: HashMap<i32, Augment> },
    ChampionsUpdated { data: HashMap<i32, ChampionSimple> },
}

// 应用状态包装
pub struct AppGameDataState {
    state: Mutex<GameDataState>,
}

impl AppGameDataState {
    pub fn new() -> Self {
        Self {
            state: Mutex::new(GameDataState::default()),
        }
    }

    // 辅助方法：发射事件
    fn emit_event(&self, app_handle: &tauri::AppHandle, event: GameDataEvent) {
        let _ = app_handle.emit_all("game-data-updated", event);
    }

    // 获取冠军名称
    pub fn champion_name(&self, id: i32) -> String {
        let state = self.state.lock().unwrap();
        state.champions
            .get(&id)
            .map(|champ| champ.name.clone())
            .unwrap_or_else(|| id.to_string())
    }

    // 设置召唤师技能
    pub fn set_summoner_spells(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, SummonerSpell>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.summoner_spells = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::SummonerSpellsUpdated { data: value }
        );
    }

    // 设置物品
    pub fn set_items(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, Item>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.items = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::ItemsUpdated { data: value }
        );
    }

    // 设置队列
    pub fn set_queues(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, Queue>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.queues = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::QueuesUpdated { data: value }
        );
    }

    // 设置符文
    pub fn set_perks(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, Perk>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.perks = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::PerksUpdated { data: value }
        );
    }

    // 设置符文样式
    pub fn set_perk_styles(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: PerkStylesData
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.perk_styles = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::PerkStylesUpdated { data: value }
        );
    }

    // 设置强化
    pub fn set_augments(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, Augment>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.augments = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::AugmentsUpdated { data: value }
        );
    }

    // 设置英雄
    pub fn set_champions(
        &self, 
        app_handle: &tauri::AppHandle, 
        value: HashMap<i32, ChampionSimple>
    ) {
        {
            let mut state = self.state.lock().unwrap();
            state.champions = value.clone();
        }
        
        self.emit_event(
            app_handle, 
            GameDataEvent::ChampionsUpdated { data: value }
        );
    }

    // 获取完整状态（用于前端初始化）
    pub fn get_full_state(&self) -> GameDataState {
        self.state.lock().unwrap().clone()
    }
}

// Tauri 命令
#[tauri::command]
async fn set_summoner_spells(
    spells: HashMap<i32, SummonerSpell>,
    state: State<'_, AppGameDataState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    state.set_summoner_spells(&app_handle, spells);
    Ok(())
}

#[tauri::command]
async fn set_champions(
    champions: HashMap<i32, ChampionSimple>,
    state: State<'_, AppGameDataState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    state.set_champions(&app_handle, champions);
    Ok(())
}

#[tauri::command]
async fn get_champion_name(
    id: i32,
    state: State<'_, AppGameDataState>,
) -> Result<String, String> {
    Ok(state.champion_name(id))
}

#[tauri::command]
async fn get_game_data_state(
    state: State<'_, AppGameDataState>,
) -> Result<GameDataState, String> {
    Ok(state.get_full_state())
}

// 其他命令...