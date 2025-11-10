use crate::core::app_init::init_and_clear::{clear_state, init_state};
use crate::shared::process::is_running;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::watch;
use tokio::time::{interval, Duration};

// Shared application state managed by Tauri
#[derive(Debug)]
pub struct AppState {
    pub open: AtomicBool,
    /// 状态变化通知通道的发送端（线程安全，不需要 Mutex）
    open_sender: Arc<watch::Sender<bool>>,
    /// 状态变化通知通道的接收端（线程安全，不需要 Mutex）
    /// 注意：如果需要多个监听者，应该克隆 receiver 而不是共享
    open_receiver: Arc<watch::Receiver<bool>>,
}

pub fn app_setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    app.manage(AppState::default());
    let app_handle = app.handle().clone();
    tauri::async_runtime::spawn(async move {
        let app_handle_for_init = app_handle.clone();
        let app_state = app_handle_for_init.state::<AppState>();
        app_state.init(app_handle).await;
    });
    Ok(())
}

impl Default for AppState {
    fn default() -> Self {
        let (sender, receiver) = watch::channel(false);
        Self {
            open: AtomicBool::new(false),
            open_sender: Arc::new(sender),
            open_receiver: Arc::new(receiver),
        }
    }
}

impl AppState {
    /// 设置 open 状态（会触发状态变化通知）
    pub async fn set_open(&self, open: bool) {
        let current = self.open.load(Ordering::Acquire);
        if current != open {
            self.open.store(open, Ordering::Release);

            // 发送状态变化通知（watch::Sender::send 只需要 &self，线程安全）
            let _ = self.open_sender.send(open);
        }
    }

    /// 初始化应用状态监控
    pub async fn init(&self, app_handle: AppHandle) {
        // 克隆 receiver 以便在异步任务中使用
        let mut rx = self.open_receiver.as_ref().clone();
        let app_handle_clone = app_handle.clone();

        tauri::async_runtime::spawn(async move {
            loop {
                // 等待状态变化
                if rx.changed().await.is_err() {
                    // 发送端关闭，退出循环
                    break;
                }

                if *rx.borrow() {
                    // open 变为 true，启动初始化流程
                    init_state(Some(app_handle_clone.clone())).await;
                } else {
                    // open 变为 false，清空初始化
                    clear_state(Some(app_handle_clone.clone())).await;
                }
            }
        });

        self.monitor_process().await;
    }

    // 进程监控循环
    // 优化：使用动态检查间隔
    // - 进程存在时：检查间隔较长（5秒），因为进程不会频繁变化
    // - 进程不存在时：检查间隔较短（1秒），快速响应进程启动
    pub async fn monitor_process(&self) {
        let mut ticker = interval(Duration::from_secs(1));
        let mut last_running_state = false;

        loop {
            ticker.tick().await;

            let is_running_now = is_running();

            // 动态调整检查间隔
            if is_running_now != last_running_state {
                // 状态变化时，重置检查间隔
                let check_interval = if is_running_now {
                    Duration::from_secs(2) // 进程存在时，2秒检查一次
                } else {
                    Duration::from_secs(1) // 进程不存在时，1秒检查一次
                };
                ticker = interval(check_interval);
                last_running_state = is_running_now;
            }

            // 更新状态（只在状态变化时触发通知）
            if is_running_now {
                self.set_open(true).await;
            } else {
                self.set_open(false).await;
            }
        }
    }
}
