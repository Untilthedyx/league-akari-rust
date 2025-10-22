use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::{AppHandle, Emitter};
use tokio::sync::OnceCell;
use tokio::sync::RwLock;
use tokio::time::interval;

use crate::lcu::api::phase::get_phase;
use crate::lcu::api::summoner::Summoner;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GameStateEvent {
    pub connected: bool,
    pub phase: Option<String>,
    pub summoner: Option<Summoner>,
}

pub struct GameStateMonitor {
    app_handle: AppHandle,
    last_state: GameStateEvent,
    last_update: SystemTime,
}

impl GameStateMonitor {
    fn new(app_handle: AppHandle) -> Self {
        // 这里直接拿所有权吗？？ 是不是需要引用
        Self {
            app_handle,
            last_state: GameStateEvent {
                connected: false,
                phase: None,
                summoner: None,
            },
            last_update: SystemTime::now(),
        }
    }

    async fn check_and_emit(&mut self) -> Result<(), String> {
        let summoner = Summoner::get_current_summoner().await;
        let phase = get_phase().await;
        let new_state = GameStateEvent {
            connected: summoner.is_ok(),
            phase: phase.ok(),
            summoner: summoner.ok(),
        };

        let state_changed = new_state != self.last_state;
        let now = SystemTime::now();
        let diff_time = now.duration_since(self.last_update).unwrap();

        if state_changed || diff_time > Duration::from_secs(5) {
            if let Err(e) = self.app_handle.emit("game-state-changed", &new_state) {
                return Err(e.to_string());
            }
            self.last_state = new_state;
        }
        Ok(())
    }
}

/// 这里感觉没必要定义为全局的
static GAME_STATE_MONITOR: OnceCell<Arc<RwLock<GameStateMonitor>>> = OnceCell::const_new();

pub async fn start_game_state_monitor(app_handle: AppHandle) {
    GAME_STATE_MONITOR
        .get_or_init(|| async move { Arc::new(RwLock::new(GameStateMonitor::new(app_handle))) })
        .await;

    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(2));

        loop {
            ticker.tick().await;
            let mut monitor = GAME_STATE_MONITOR.get().unwrap().write().await;
            monitor.check_and_emit().await.expect("错误");
        }
    });
}
