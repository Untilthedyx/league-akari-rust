use std::any::Any;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::fmt;
use std::sync::{Arc, AtomicBool, Mutex};
use tokio::sync::{CancellationToken, Notify, Semaphore};
use tokio::task::JoinSet;
use futures::future::join_all;
use crate::utils::error::task_runner_error::TaskRunnerError;
use std::pin::Pin;
use std::future::Future;


// 事件 payload 定义
#[derive(Debug, Clone)]
struct TaskStartPayload {
    id: String,
}

#[derive(Debug)]
enum TaskCompletePayload {
    Success { id: String, value: Box<dyn Any + Send> },
    Error { id: String, error: Box<dyn Error + Send> },
}

#[derive(Debug)]
// 任务定义（包装异步函数）
struct Task {
    id: String,
    func: Box<dyn FnOnce() -> Pin<Box<dyn std::future::Future<Output = Result<Box<dyn Any + Send>, Box<dyn Error + Send>>> + Send>> + Send>,
    priority: u32,
    group: String,
}

// 带优先级的任务（用于优先队列）
#[derive(Debug)]
struct PrioritizedTask(Task);

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.priority.cmp(&self.0.priority) // 高优先级先执行
    }
}

impl PartialOrd for PrioritizedTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PrioritizedTask {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority == other.0.priority
    }
}

impl Eq for PrioritizedTask {}

#[derive(Debug)]
// 任务组定义
struct TaskGroup {
    concurrency: usize,
    dependencies: Vec<String>,
    queue: Arc<Mutex<BinaryHeap<PrioritizedTask>>>, // 优先队列
    semaphore: Arc<Semaphore>, // 控制并发
    join_set: Arc<Mutex<JoinSet<()>>>, // 跟踪运行中的任务
    notify: Arc<Notify>, // 用于通知依赖组当前组已完成
}

// 任务组配置
#[derive(Default)]
struct TaskGroupOptions {
    concurrency: Option<usize>,
    after_group: Option<Vec<String>>, // 依赖的组
}

// 任务注册配置
#[derive(Default)]
struct RegisterOptions {
    priority: Option<u32>,
    group: Option<String>,
}

// 任务运行器
#[derive(Debug, Default)]
pub struct TaskRunner {
    groups: Arc<Mutex<HashMap<String, TaskGroup>>>,
    tasks: Arc<Mutex<HashMap<String, Task>>>,
    is_running: Arc<AtomicBool>,
    cancel_token: Mutex<Option<CancellationToken>>,
    // 事件回调
    on_task_start: Arc<Mutex<Vec<Box<dyn Fn(TaskStartPayload) + Send + Sync>>>>,
    on_task_complete: Arc<Mutex<Vec<Box<dyn Fn(TaskCompletePayload) + Send + Sync>>>>,
    on_start: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
    on_stop: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
    default_concurrency: usize,
}


impl TaskRunner {
    // 创建新的任务运行器
    pub fn new(default_concurrency: usize) -> Self {
        let groups = Arc::new(Mutex::new(HashMap::new()));
        // 初始化默认组
        let default_group = TaskGroup {
            concurrency: default_concurrency,
            dependencies: Vec::new(),
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(default_concurrency)),
            join_set: Arc::new(Mutex::new(JoinSet::new())),
            notify: Arc::new(Notify::new()),
        };
        groups.lock().unwrap().insert("default".to_string(), default_group);

        Self {
            groups,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            cancel_token: Mutex::new(None),
            on_task_start: Arc::new(Mutex::new(Vec::new())),
            on_task_complete: Arc::new(Mutex::new(Vec::new())),
            on_start: Arc::new(Mutex::new(Vec::new())),
            on_stop: Arc::new(Mutex::new(Vec::new())),
            default_concurrency,
        }
    }

    // 创建任务组
    pub fn create_group(&self, id: &str, options: TaskGroupOptions) -> Result<(), TaskRunnerError> {
        let mut groups = self.groups.lock().unwrap();
        if groups.contains_key(id) {
            return Err(TaskRunnerError::GroupExists(id.to_string()));
        }

        let deps = options.after_group.unwrap_or_default();
        // 检查依赖是否存在
        for dep in &deps {
            if !groups.contains_key(dep) {
                return Err(TaskRunnerError::DependencyNotFound(dep.clone(), id.to_string()));
            }
        }

        // 检测循环依赖
        let mut dep_graph: HashMap<_, _> = groups.iter()
            .map(|(k, v)| (k.clone(), v.dependencies.clone()))
            .collect();
        dep_graph.insert(id.to_string(), deps.clone());
        if self.detect_cycle(&dep_graph) {
            return Err(TaskRunnerError::CircularDependency(id.to_string()));
        }

        // 创建组
        let concurrency = options.concurrency.unwrap_or(self.default_concurrency);
        groups.insert(id.to_string(), TaskGroup {
            concurrency,
            dependencies: deps,
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(concurrency)),
            join_set: Arc::new(Mutex::new(JoinSet::new())),
            notify: Arc::new(Notify::new()),
        });
        Ok(())
    }

    // 删除任务组
    pub fn remove_group(&self, id: &str) -> Result<(), TaskRunnerError> {
        if id == "default" {
            return Err(TaskRunnerError::CannotRemoveDefault);
        }

        let groups = self.groups.lock().unwrap();
        // 检查是否有其他组依赖当前组
        for (group, info) in groups.iter() {
            if info.dependencies.contains(&id.to_string()) {
                return Err(TaskRunnerError::GroupDependedOn(id.to_string(), group.clone()));
            }
        }
        drop(groups); // 提前释放锁

        self.groups.lock().unwrap().remove(id);
        Ok(())
    }

    // 检测循环依赖（DFS实现）
    fn detect_cycle(&self, dep_graph: &HashMap<String, Vec<String>>) -> bool {
        #[derive(PartialEq, Eq)]
        enum State { New, Visiting, Visited }
        let mut states = HashMap::new();
        for k in dep_graph.keys() {
            states.insert(k.clone(), State::New);
        }

        fn dfs(node: &str, graph: &HashMap<String, Vec<String>>, states: &mut HashMap<String, State>) -> bool {
            states.insert(node.to_string(), State::Visiting);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    match states[neighbor] {
                        State::Visiting => return true, // 发现环
                        State::New => if dfs(neighbor, graph, states) { return true; },
                        State::Visited => continue,
                    }
                }
            }
            states.insert(node.to_string(), State::Visited);
            false
        }

        dep_graph.keys().any(|k| states[k] == State::New && dfs(k, dep_graph, &mut states))
    }

    // 注册任务
    pub fn register<F, Fut, R, E>(
        &self,
        id: &str,
        func: F,
        options: RegisterOptions,
    ) -> Result<(), TaskRunnerError>
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = Result<R, E>> + Send + 'static,
        R: Any + Send + 'static,
        E: Error + Send + 'static,
    {
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(TaskRunnerError::AddingTaskWhileRunning);
        }

        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(id) {
            return Err(TaskRunnerError::TaskExists(id.to_string()));
        }

        let group = options.group.unwrap_or_else(|| "default".to_string());
        let groups = self.groups.lock().unwrap();
        if !groups.contains_key(&group) {
            return Err(TaskRunnerError::GroupNotFound(group));
        }

        // 包装任务函数为统一类型
        let wrapped_func = Box::new(move || {
            let fut = async move {
                func().await
                    .map(|val| Box::new(val) as Box<dyn Any + Send>)
                    .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
            };
            Box::pin(fut)
        });

        tasks.insert(id.to_string(), Task {
            id: id.to_string(),
            func: wrapped_func,
            priority: options.priority.unwrap_or(50),
            group,
        });
        Ok(())
    }

    // 删除任务
    pub fn remove_task(&self, id: &str) -> Result<(), TaskRunnerError> {
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(TaskRunnerError::RemovingTaskWhileRunning);
        }

        let mut tasks = self.tasks.lock().unwrap();
        if tasks.remove(id).is_none() {
            // 可选：如果需要严格检查任务是否存在，可返回错误
            // return Err(TaskRunnerError::TaskNotFound(id.to_string()));
        }
        Ok(())
    }

    // 启动任务运行
    pub async fn start(&self) -> Result<(), TaskRunnerError> {
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(TaskRunnerError::AlreadyRunning);
        }
        self.is_running.store(true, std::sync::atomic::Ordering::SeqCst);
        let cancel_token = CancellationToken::new();
        self.cancel_token.lock().unwrap().replace(cancel_token.clone());

        // 触发start事件（克隆回调列表避免长时间持有锁）
        let on_start = {
            let cbs = self.on_start.lock().unwrap().clone();
            cbs
        };
        on_start.iter().for_each(|cb| cb());

        // 将任务加入对应组的队列
        let tasks = {
            let mut tasks_lock = self.tasks.lock().unwrap();
            tasks_lock.drain().collect::<Vec<_>>()
        };
        for (_, task) in tasks {
            let group_id = task.group.clone();
            let mut groups = self.groups.lock().unwrap();
            let group = groups.get_mut(&group_id).unwrap();
            group.queue.lock().unwrap().push(PrioritizedTask(task));
        }

        // 处理组依赖并启动执行
        let groups = {
            let groups_lock = self.groups.lock().unwrap();
            groups_lock.clone()
        };
        let groups_arc = self.groups.clone();
        let cancel = cancel_token.clone();
        let on_task_start = self.on_task_start.clone();
        let on_task_complete = self.on_task_complete.clone();

        tokio::spawn(async move {
            for (group_id, group) in groups {
                let deps = group.dependencies.clone();
                // 等待依赖组完成（使用Notify替代轮询）
                if !deps.is_empty() {
                    let dep_notifies: Vec<_> = deps.iter()
                        .filter_map(|dep_id| {
                            groups_arc.lock().unwrap().get(dep_id).map(|g| g.notify.clone())
                        })
                        .collect();
                    
                    // 等待所有依赖组的通知
                    let wait_futures = dep_notifies.iter()
                        .map(|notify| notify.notified());
                    join_all(wait_futures).await;
                }

                // 启动当前组任务（检查是否已取消）
                if !cancel.is_cancelled() {
                    Self::run_group_tasks(
                        &group_id, 
                        &group, 
                        cancel.clone(), 
                        on_task_start.clone(), 
                        on_task_complete.clone()
                    ).await;
                }
            }
        });

        // 等待所有任务完成后触发stop事件
        let groups_arc = self.groups.clone();
        let cancel = cancel_token.clone();
        let on_stop = self.on_stop.clone();
        let is_running = self.is_running.clone();
        tokio::spawn(async move {
            // 等待所有组完成
            let groups = groups_arc.lock().unwrap();
            let all_notifies: Vec<_> = groups.values()
                .map(|g| g.notify.clone())
                .collect();
            drop(groups);

            // 等待所有组的完成通知
            join_all(all_notifies.iter().map(|n| n.notified())).await;

            // 触发stop事件
            let cbs = on_stop.lock().unwrap().clone();
            cbs.iter().for_each(|cb| cb());
            
            is_running.store(false, std::sync::atomic::Ordering::SeqCst);
        });

        Ok(())
    }

    // 运行组内任务
    async fn run_group_tasks(
        group_id: &str,
        group: &TaskGroup,
        cancel: CancellationToken,
        on_task_start: Arc<Mutex<Vec<Box<dyn Fn(TaskStartPayload) + Send + Sync>>>>,
        on_task_complete: Arc<Mutex<Vec<Box<dyn Fn(TaskCompletePayload) + Send + Sync>>>>,
    ) {
        let group = group.clone();
        let group_id = group_id.to_string();
        tokio::spawn(async move {
            loop {
                if cancel.is_cancelled() {
                    // 取消时清空队列并通知依赖
                    group.queue.lock().unwrap().clear();
                    group.notify.notify_waiters();
                    break;
                }

                // 从队列取任务（缩短锁持有时间）
                let task = {
                    let mut queue = group.queue.lock().unwrap();
                    queue.pop().map(|pt| pt.0)
                };

                let Some(task) = task else {
                    // 检查是否还有运行中的任务
                    let join_set_empty = {
                        group.join_set.lock().unwrap().is_empty()
                    };
                    if join_set_empty {
                        // 队列空且无运行中任务，通知依赖组
                        group.notify.notify_waiters();
                        break;
                    }
                    // 等待10ms后再检查（避免空轮询）
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                    continue;
                };

                // 控制并发（获取信号量许可）
                let permit = match group.semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_) => break, // 信号量已关闭
                };

                let task_id = task.id.clone();
                let start_cb = on_task_start.clone();
                let complete_cb = on_task_complete.clone();
                let func = task.func;

                // 执行任务（捕获恐慌）
                let mut join_set = group.join_set.lock().unwrap();
                join_set.spawn(async move {
                    // 触发task-start事件（克隆回调避免锁竞争）
                    let start_payload = TaskStartPayload { id: task_id.clone() };
                    let start_cbs = start_cb.lock().unwrap().clone();
                    start_cbs.iter().for_each(|cb| cb(start_payload));

                    // 执行任务函数（捕获恐慌）
                    let result = match tokio::task::spawn(async move {
                        func().await
                    }).await {
                        Ok(Ok(val)) => Ok(val),
                        Ok(Err(e)) => Err(e),
                        Err(panic) => {
                            // 将恐慌转换为错误
                            Err(Box::new(TaskRunnerError::TaskPanicError(task_id.clone())) as Box<dyn Error + Send>)
                        }
                    };

                    // 触发task-complete事件
                    let complete_payload = match result {
                        Ok(val) => TaskCompletePayload::Success { id: task_id.clone(), value: val },
                        Err(e) => TaskCompletePayload::Error { id: task_id.clone(), error: e },
                    };
                    let complete_cbs = complete_cb.lock().unwrap().clone();
                    complete_cbs.iter().for_each(|cb| cb(complete_payload));

                    drop(permit); // 释放信号量许可
                });
            }
        });
    }

    // 停止任务运行
    pub fn stop(&self) -> Result<(), TaskRunnerError> {
        if !self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(TaskRunnerError::NotRunning);
        }
        if let Some(ct) = self.cancel_token.lock().unwrap().take() {
            ct.cancel();
        }
        Ok(())
    }

    // 注册事件回调（task-start）
    pub fn on_task_start<F: Fn(TaskStartPayload) + Send + Sync + 'static>(&self, cb: F) {
        self.on_task_start.lock().unwrap().push(Box::new(cb));
    }

    // 注册事件回调（task-complete）
    pub fn on_task_complete<F: Fn(TaskCompletePayload) + Send + Sync + 'static>(&self, cb: F) {
        self.on_task_complete.lock().unwrap().push(Box::new(cb));
    }

    // 注册事件回调（start）
    pub fn on_start<F: Fn() + Send + Sync + 'static>(&self, cb: F) {
        self.on_start.lock().unwrap().push(Box::new(cb));
    }

    // 注册事件回调（stop）
    pub fn on_stop<F: Fn() + Send + Sync + 'static>(&self, cb: F) {
        self.on_stop.lock().unwrap().push(Box::new(cb));
    }
}