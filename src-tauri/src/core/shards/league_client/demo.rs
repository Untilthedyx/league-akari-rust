pub mod champ_select;
pub mod chat;
pub mod entitlements;
pub mod game_data;
pub mod gameflow;
pub mod honor;
pub mod league_session;
pub mod lobby;
pub mod lobby_team_builder;
pub mod login;
pub mod matchmaking;
pub mod summoner;

use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use crate::core::shards::league_client::lcu_state::champ_select::ChampSelectStateLock;
use crate::core::shards::league_client::lcu_state::chat::ChatStateLock;
use crate::core::shards::league_client::lcu_state::entitlements::EntitlementsStateLock;
use crate::core::shards::league_client::lcu_state::game_data::GameDataStateLock;
use crate::core::shards::league_client::lcu_state::gameflow::GameflowStateLock;
use crate::core::shards::league_client::lcu_state::honor::HonorStateLock;
use crate::core::shards::league_client::lcu_state::league_session::LeagueSessionStateLock;
use crate::core::shards::league_client::lcu_state::lobby::LobbyStateLock;
use crate::core::shards::league_client::lcu_state::lobby_team_builder::LobbyTeamBuilderStateLock;
use crate::core::shards::league_client::lcu_state::login::LoginStateLock;
use crate::core::shards::league_client::lcu_state::matchmaking::MatchmakingStateLock;
use crate::core::shards::league_client::lcu_state::summoner::SummonerStateLock;
use crate::core::shards::league_client::utils::task_runner::{
    TaskCompletePayload, TaskGroupOptions, TaskRunner,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct InitializationProgress {
    pub current_id: Option<String>,
    pub finished: Vec<String>,
    pub all: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct InitializationState {
    pub progress: Option<InitializationProgress>,
}

impl InitializationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_progress(&mut self, progress: Option<InitializationProgress>) {
        self.progress = progress;
    }
}

#[derive(Debug, Default)]
pub struct LeagueClientData {
    pub _state_initializer: Arc<Mutex<TaskRunner>>,
    pub initialization: Arc<Mutex<InitializationState>>,
    pub gameflow: GameflowStateLock,
    pub chat: ChatStateLock,
    pub honor: HonorStateLock,
    pub champ_select: ChampSelectStateLock,
    pub login: LoginStateLock,
    pub lobby: LobbyStateLock,
    pub summoner: SummonerStateLock,
    pub matchmaking: MatchmakingStateLock,
    pub game_data: GameDataStateLock,
    pub entitlements: EntitlementsStateLock,
    pub league_session: LeagueSessionStateLock,
    pub lobby_team_builder: LobbyTeamBuilderStateLock,
}

impl LeagueClientData {
    pub fn new() -> Self {
        // 不需要用 Arc 包装 TaskRunner，直接创建
        let mut _state_initializer = Arc::new(Mutex::new(TaskRunner::new(10)));
        let initialization = Arc::new(Mutex::new(InitializationState::new()));
        // 使用 Arc<Mutex<HashSet>> 实现线程安全的集合共享和修改
        let finished_tasks = Arc::new(Mutex::new(HashSet::new()));

        // 其他状态初始化保持不变
        let gameflow = GameflowStateLock::new();
        let chat = ChatStateLock::new();
        let honor = HonorStateLock::new();
        let champ_select = ChampSelectStateLock::new();
        let login = LoginStateLock::new();
        let lobby = LobbyStateLock::new();
        let summoner = SummonerStateLock::new();
        let matchmaking = MatchmakingStateLock::new();
        let game_data = GameDataStateLock::new();
        let entitlements = EntitlementsStateLock::new();
        let league_session = LeagueSessionStateLock::new();
        let lobby_team_builder = LobbyTeamBuilderStateLock::new();

        // 克隆初始化状态供闭包使用
        let initialization_start = initialization.clone();
        _state_initializer
            .lock()
            .unwrap()
            .create_group(
                "game-data",
                TaskGroupOptions {
                    concurrency: Some(3),
                    after_group: None,
                },
            )
            .unwrap();

        // 处理启动回调
        _state_initializer.lock().unwrap().on_start(move || {
            initialization_start
                .lock()
                .unwrap()
                .set_progress(Some(InitializationProgress {
                    current_id: None,
                    finished: vec![],
                    all: vec![],
                }));
        });

        let finished_tasks_clone = finished_tasks.clone();
        let initialization_clone = initialization.clone();
        let state_initializer_clone = _state_initializer.clone();

        // 处理任务完成回调
        _state_initializer
            .lock()
            .unwrap()
            .on_task_complete(move |payload| match &*payload {
                TaskCompletePayload::Success { id, value: _ } => {
                    // 锁定集合并插入完成的任务 ID
                    let mut finished = finished_tasks_clone.lock().unwrap();
                    finished.insert(id.clone());

                    // 更新初始化进度
                    let mut init_state = initialization_clone.lock().unwrap();
                    init_state.set_progress(Some(InitializationProgress {
                        current_id: Some(id.clone()),
                        // 将 HashSet 转换为 Vec（注意解引用顺序）
                        finished: finished.iter().cloned().collect(),
                        // 获取所有任务 ID 并转换为 Vec
                        all: state_initializer_clone
                            .lock()
                            .unwrap()
                            .tasks
                            .lock()
                            .unwrap()
                            .keys()
                            .cloned()
                            .collect(),
                    }));
                }
                TaskCompletePayload::Error { id: _, error: _ } => {
                    // 可以在这里处理错误情况
                }
            });

        let finished_tasks_clone = finished_tasks.clone();
        let initialization_clone = initialization.clone();

        _state_initializer.lock().unwrap().on_stop(move || {
            finished_tasks_clone.lock().unwrap().clear();
            initialization_clone.lock().unwrap().set_progress(None);
        });

        Self {
            _state_initializer,
            initialization,
            gameflow,
            chat,
            honor,
            champ_select,
            login,
            lobby,
            summoner,
            matchmaking,
            game_data,
            entitlements,
            league_session,
            lobby_team_builder,
        }
    }

    // pub async fn _lcu_game_flow(&self){
    //     self.
    // }
}
