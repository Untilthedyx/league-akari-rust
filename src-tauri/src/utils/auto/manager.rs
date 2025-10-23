use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, LazyLock, Mutex};

use tokio::sync::watch;
use tokio::task::JoinHandle;

#[derive(Debug)]
struct AutomationTask {
    handle: Option<JoinHandle<()>>,
    shutdown_tx: Option<watch::Sender<bool>>,
}

#[derive(Debug)]
pub struct AutomationManager {
    tasks: Arc<Mutex<HashMap<String, AutomationTask>>>,
}

impl AutomationManager {
    fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn wait_for_showdown(mut shutdown_rx: watch::Receiver<bool>) {
        loop {
            if *shutdown_rx.borrow() {
                break;
            }
            if shutdown_rx.changed().await.is_err() {
                break;
            }
        }
    }

    pub fn start_task(&self, name: &str, task: impl Future<Output = ()> + Send + 'static) {
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let task_name = name.to_string();
        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = task => {},
                _ = Self::wait_for_showdown(shutdown_rx) => {}
            }
        });

        let mut tasks = self.tasks.lock().unwrap();
        if let Some(existing_task) = tasks.get_mut(name) {
            if let Some(tx) = existing_task.shutdown_tx.take() {
                let _ = tx.send(true);
            }
            if let Some(handle) = existing_task.handle.take() {
                handle.abort();
            }
        }

        tasks.insert(
            task_name,
            AutomationTask {
                handle: Some(handle),
                shutdown_tx: Some(shutdown_tx),
            },
        );
    }

    pub fn stop_task(&self, name: &str) {
        let mut tasks = self.tasks.lock().unwrap();
        if let Some(task) = tasks.get_mut(name) {
            if let Some(tx) = task.shutdown_tx.take() {
                let _ = tx.send(true);
            }
            if let Some(handle) = task.handle.take() {
                handle.abort();
            }
        }
        tasks.remove(name);
    }
}

pub static AUTOMATION_MANAGER: LazyLock<AutomationManager> = LazyLock::new(AutomationManager::new);
