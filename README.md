```cmd
npm run tauri dev
npm run tauri build --release
```

## 智能指针

### 1. LazyLock 惰性锁 和 OnceCell

LazyLock 的初始化是 "被动触发" 的（第一次访问时自动执行），而 OnceCell 可以通过 get_or_init 主动控制初始化时机。

### 2. RwLock 读写锁相较于 Mutex

RwLock 可以同时允许多个读操作，但是只允许一个写操作，写操作会阻塞所有的读操作。Mutex 则是无论读写操作都会阻塞。

## 配置文件

Akari 的配置文件存放在 C:\Users\周瑾瑜\AppData\Roaming\league-akari 目录下

Akari 配置文件位置 检索 `app.getPath('userData')`

## 相关

1. [rank-analysis](https://github.com/wnzzer/rank-analysis)
2. [LeagueAkari](https://github.com/Hanxven/LeagueAkari)
3. [League of Legends LCU and Riot Client API Docs](https://github.com/KebsCS/lcu-and-riotclient-api)
4. [Seraphine](https://github.com/Zzaphkiel/Seraphine)
5. [LCU API](https://www.mingweisamuel.com/lcu-schema/tool/#/)



检查项目中是否已有进程监控或状态检查的相关实现：


[4 tools called]

查看监控相关代码：


[2 tools called]

基于项目现有代码，提供一个进程条件监控与操作控制的方案。

## 解决方案设计

方案包含：
1. 进程条件监控器：定期检查进程命令行指标
2. 操作门控：控制是否允许执行下一步操作
3. 条件检查函数：可自定义的检查逻辑

### 方案 1：使用 `CancellationToken` 控制操作（推荐）

```rust
// src-tauri/src/shared/utils/process_monitor.rs
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{OnceCell, RwLock};
use tokio::time::interval;
use tokio_util::sync::CancellationToken;
use crate::utils::process::process_info::{get_process_command_line, get_process_pid_by_name, ProcessInfo};

/// 进程条件检查函数类型
/// 返回 true 表示条件满足，允许执行操作
/// 返回 false 表示条件不满足，应该停止操作
pub type ConditionChecker = Box<dyn Fn(&ProcessInfo) -> bool + Send + Sync>;

/// 进程监控器状态
#[derive(Debug, Clone, PartialEq)]
pub enum MonitorState {
    /// 条件满足，允许操作
    Allowed,
    /// 条件不满足，禁止操作
    Blocked,
    /// 进程不存在
    ProcessNotFound,
}

/// 进程条件监控器
pub struct ProcessConditionMonitor {
    /// 进程名称
    process_name: String,
    /// 条件检查函数
    condition_checker: ConditionChecker,
    /// 当前状态
    state: Arc<RwLock<MonitorState>>,
    /// 操作取消令牌（当条件不满足时，取消正在进行的操作）
    cancellation_token: Arc<RwLock<CancellationToken>>,
    /// 监控间隔
    check_interval: Duration,
}

impl ProcessConditionMonitor {
    /// 创建新的监控器
    ///
    /// # 参数
    /// - `process_name`: 要监控的进程名称（如 "LeagueClientUx.exe"）
    /// - `condition_checker`: 条件检查函数，检查 ProcessInfo 是否符合要求
    /// - `check_interval`: 检查间隔（默认 1 秒）
    pub fn new<F>(process_name: String, condition_checker: F, check_interval: Duration) -> Self
    where
        F: Fn(&ProcessInfo) -> bool + Send + Sync + 'static,
    {
        Self {
            process_name,
            condition_checker: Box::new(condition_checker),
            state: Arc::new(RwLock::new(MonitorState::ProcessNotFound)),
            cancellation_token: Arc::new(RwLock::new(CancellationToken::new())),
            check_interval,
        }
    }

    /// 检查进程条件
    async fn check_condition(&self) -> MonitorState {
        // 获取进程 PID
        let pids = match get_process_pid_by_name(&self.process_name) {
            Ok(pids) if !pids.is_empty() => pids,
            _ => return MonitorState::ProcessNotFound,
        };

        // 获取第一个有效进程的命令行
        for pid in pids {
            if let Ok(cmd_line) = get_process_command_line(pid) {
                if let Ok(process_info) = crate::utils::process::process_info::parse_command_line(&cmd_line) {
                    // 检查条件
                    let condition_met = (self.condition_checker)(&process_info);
                    return if condition_met {
                        MonitorState::Allowed
                    } else {
                        MonitorState::Blocked
                    };
                }
            }
        }

        MonitorState::ProcessNotFound
    }

    /// 启动监控
    pub async fn start(&self) {
        let state = Arc::clone(&self.state);
        let cancellation_token = Arc::clone(&self.cancellation_token);
        let process_name = self.process_name.clone();
        let condition_checker = self.condition_checker.clone();
        let check_interval = self.check_interval;

        tokio::spawn(async move {
            let mut ticker = interval(check_interval);

            loop {
                ticker.tick().await;

                // 检查条件
                let new_state = {
                    // 临时创建一个监控器实例来检查条件
                    // 这里需要重构，或者将检查逻辑提取为独立函数
                    // 简化版：直接在这里实现检查逻辑
                    Self::check_condition_internal(&process_name, &condition_checker).await
                };

                // 更新状态
                let mut state_guard = state.write().await;
                let old_state = state_guard.clone();
                *state_guard = new_state.clone();

                // 如果状态从 Allowed 变为 Blocked，取消正在进行的操作
                if old_state == MonitorState::Allowed && new_state == MonitorState::Blocked {
                    let mut token_guard = cancellation_token.write().await;
                    token_guard.cancel();
                    *token_guard = CancellationToken::new();
                }

                // 如果状态从 Blocked 变为 Allowed，重置取消令牌（允许新操作）
                if old_state == MonitorState::Blocked && new_state == MonitorState::Allowed {
                    let mut token_guard = cancellation_token.write().await;
                    *token_guard = CancellationToken::new();
                }
            }
        });
    }

    async fn check_condition_internal(
        process_name: &str,
        condition_checker: &ConditionChecker,
    ) -> MonitorState {
        let pids = match get_process_pid_by_name(process_name) {
            Ok(pids) if !pids.is_empty() => pids,
            _ => return MonitorState::ProcessNotFound,
        };

        for pid in pids {
            if let Ok(cmd_line) = get_process_command_line(pid) {
                if let Ok(process_info) = crate::utils::process::process_info::parse_command_line(&cmd_line) {
                    return if condition_checker(&process_info) {
                        MonitorState::Allowed
                    } else {
                        MonitorState::Blocked
                    };
                }
            }
        }

        MonitorState::ProcessNotFound
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> MonitorState {
        self.state.read().await.clone()
    }

    /// 获取取消令牌（用于取消正在进行的操作）
    pub async fn get_cancellation_token(&self) -> CancellationToken {
        self.cancellation_token.read().await.clone()
    }

    /// 等待条件满足（阻塞直到条件满足）
    pub async fn wait_for_condition(&self) -> Result<(), String> {
        loop {
            let state = self.get_state().await;
            match state {
                MonitorState::Allowed => return Ok(()),
                MonitorState::Blocked => {
                    // 等待一段时间后重试
                    tokio::time::sleep(self.check_interval).await;
                }
                MonitorState::ProcessNotFound => {
                    return Err("进程不存在".to_string());
                }
            }
        }
    }

    /// 执行操作（带条件检查）
    /// 如果条件不满足，会立即返回错误
    pub async fn execute_with_condition<F, Fut, T>(&self, operation: F) -> Result<T, String>
    where
        F: FnOnce(CancellationToken) -> Fut,
        Fut: std::future::Future<Output = Result<T, String>>,
    {
        // 检查当前状态
        let state = self.get_state().await;
        if state != MonitorState::Allowed {
            return Err(format!("条件不满足，当前状态: {:?}", state));
        }

        // 获取取消令牌
        let cancellation_token = self.get_cancellation_token().await;

        // 执行操作
        operation(cancellation_token).await
    }
}

// 全局监控器实例
static PROCESS_MONITOR: OnceCell<Arc<RwLock<ProcessConditionMonitor>>> = OnceCell::const_new();

/// 初始化进程监控器
pub async fn init_process_monitor<F>(
    process_name: String,
    condition_checker: F,
    check_interval: Duration,
) -> Result<(), String>
where
    F: Fn(&ProcessInfo) -> bool + Send + Sync + 'static,
{
    let monitor = Arc::new(RwLock::new(ProcessConditionMonitor::new(
        process_name,
        condition_checker,
        check_interval,
    )));

    PROCESS_MONITOR
        .set(monitor.clone())
        .map_err(|_| "监控器已初始化".to_string())?;

    monitor.read().await.start().await;
    Ok(())
}

/// 获取监控器实例
pub async fn get_monitor() -> Result<Arc<RwLock<ProcessConditionMonitor>>, String> {
    PROCESS_MONITOR
        .get()
        .ok_or_else(|| "监控器未初始化".to_string())
        .map(Arc::clone)
}
```

### 使用示例

```rust
// 在 lib.rs 的 setup 中初始化
use crate::shared::utils::process_monitor;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // 初始化进程监控器
            tauri::async_runtime::spawn(async move {
                // 示例：检查端口号是否大于 60000
                process_monitor::init_process_monitor(
                    "LeagueClientUx.exe".to_string(),
                    |info| {
                        // 自定义条件：端口号必须大于 60000
                        info.port > 60000
                    },
                    Duration::from_secs(1), // 每秒检查一次
                )
                .await
                .unwrap();
            });
            
            Ok(())
        })
        // ...
}
```

```rust
// 在执行操作时使用
use crate::shared::utils::process_monitor;

#[tauri::command]
pub async fn perform_some_operation() -> Result<String, String> {
    let monitor = process_monitor::get_monitor().await?;
    
    // 方式 1：等待条件满足后执行
    monitor.read().await.wait_for_condition().await?;
    // 执行操作...
    
    // 方式 2：带取消令牌的操作（可以在条件变为不满足时取消）
    monitor.read().await.execute_with_condition(|token| async move {
        // 执行长时间运行的操作
        // 可以在循环中检查 token.is_cancelled() 来提前退出
        for i in 0..100 {
            if token.is_cancelled() {
                return Err("操作被取消：条件不再满足".to_string());
            }
            // 执行操作...
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        Ok("操作完成".to_string())
    }).await
}
```

### 方案 2：使用 `watch` channel 通知状态变化（更灵活）

```rust
use tokio::sync::watch;

pub struct ProcessConditionMonitor {
    // ... 其他字段
    /// 状态变化通知通道
    state_sender: watch::Sender<MonitorState>,
    state_receiver: watch::Receiver<MonitorState>,
}

impl ProcessConditionMonitor {
    pub fn new(...) -> Self {
        let (sender, receiver) = watch::channel(MonitorState::ProcessNotFound);
        Self {
            // ...
            state_sender: sender,
            state_receiver: receiver,
        }
    }

    /// 订阅状态变化
    pub fn subscribe(&self) -> watch::Receiver<MonitorState> {
        self.state_receiver.clone()
    }

    /// 在监控循环中发送状态更新
    async fn update_state(&self, new_state: MonitorState) {
        let _ = self.state_sender.send(new_state);
    }
}
```

## 关键特性

1. 条件检查：自定义函数检查进程命令行指标
2. 状态管理：跟踪 Allowed/Blocked/ProcessNotFound
3. 操作控制：使用 `CancellationToken` 取消不符合条件的操作
4. 自动监控：后台定期检查，状态变化时自动更新
5. 等待机制：`wait_for_condition()` 可阻塞等待条件满足

## 使用建议

1. 初始化：在 `setup` 中初始化监控器
2. 条件函数：根据实际需求定义检查逻辑（如端口、region 等）
3. 操作时检查：在执行关键操作前检查状态或使用 `execute_with_condition`
4. 取消支持：长时间操作中定期检查 `token.is_cancelled()`

需要我基于你的具体指标（如端口、region 等）完善实现吗？