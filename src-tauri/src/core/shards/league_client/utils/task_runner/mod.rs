use crate::utils::error::task_runner_error::TaskRunnerError;
use futures::future::join_all;
use std::any::Any;
use std::collections::{BinaryHeap, HashMap};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use tokio::sync::{Notify, Semaphore};
use tokio::task::JoinSet;
use tracing::{error, info, trace, warn}; // 引入tracing宏

// 当任务开始执行 → 触发 on_task_start 回调，携带 TaskStartPayload
#[derive(Debug, Clone)]
pub struct TaskStartPayload {
    #[allow(dead_code)]
    id: String,
}

// 当任务结束 → 触发 on_task_complete 回调，TaskCompletePayload 成功或失败。
#[derive(Debug)]
pub enum TaskCompletePayload {
    Success {
        id: String,
        value: Box<dyn Any + Send>,
    },
    Error {
        id: String,
        error: Box<dyn Error + Send>,
    },
}

/// 注意：不能用 `Box<dyn FnOnce>` 作为 trait object（object-safe 问题）
/// 我把 Task.func 统一改为 `Box<dyn Fn() -> Pin<Box<dyn Future<...>>> + Send + Sync>`  ？？？ 这里为什么要加入 Sync
/// 要求注册时传入的闭包实现 `Fn()`（可重复调用）——这是常见的处理方式。
struct Task {
    id: String,
    func: Box<
        dyn Fn() -> Pin<
                Box<dyn Future<Output = Result<Box<dyn Any + Send>, Box<dyn Error + Send>>> + Send>,
            > + Send
            + Sync,
    >,
    priority: u32,
    group: String,
}

impl std::fmt::Debug for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("priority", &self.priority)
            .field("group", &self.group)
            .finish()
    }
}

/// 带优先级的任务（用于优先队列） 未来使用 BinaryHeap 封装
#[derive(Debug)]
struct PrioritizedTask(Arc<Task>);

impl Ord for PrioritizedTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // BinaryHeap 默认是 max-heap，cmp 倒过来使 higher priority (大数) 先出
        other.0.priority.cmp(&self.0.priority)
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

/// 任务组定义
#[derive(Debug, Clone)]
struct TaskGroup {
    dependencies: Vec<String>,                      // 依赖的其他任务组（必须先完成它们）
    queue: Arc<Mutex<BinaryHeap<PrioritizedTask>>>, // 优先队列
    semaphore: Arc<Semaphore>,                      // 控制并发（限制该组同时运行的任务数， default_concurrency）
    join_set: Arc<Mutex<JoinSet<()>>>,              // 跟踪运行中的任务
    notify: Arc<Notify>,                            // 用于通知依赖组当前组已完成
}

/// 任务组配置
#[derive(Default)]
pub struct TaskGroupOptions {
    concurrency: Option<usize>,
    after_group: Option<Vec<String>>, // 依赖的组
}

/// 任务注册配置
#[derive(Default)]
pub struct RegisterOptions {
    priority: Option<u32>,
    group: Option<String>,
}

/// 任务运行器
#[derive(Default)]
pub struct TaskRunner {
    groups: Arc<Mutex<HashMap<String, TaskGroup>>>, // 所有任务组（每个组内部有自己的队列、信号量、notify）。<组名：内容>
    tasks: Arc<Mutex<HashMap<String, Task>>>,       // 所有注册但未开始的任务。 <>
    is_running: Arc<AtomicBool>,                    // 是否正在运行。
    global_join_set: Arc<Mutex<JoinSet<()>>>, // 全局任务集：管理所有顶级任务（用于统一取消）管理所有异步任务生命周期（可统一取消）。
    default_concurrency: usize,               // 默认并发数。

    // 四类事件回调
    on_task_start: Arc<Mutex<Vec<Box<dyn Fn(Arc<TaskStartPayload>) + Send + Sync>>>>,
    on_task_complete: Arc<Mutex<Vec<Box<dyn Fn(Arc<TaskCompletePayload>) + Send + Sync>>>>,
    on_start: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
    on_stop: Arc<Mutex<Vec<Box<dyn Fn() + Send + Sync>>>>,
}

impl std::fmt::Debug for TaskRunner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 对每个 Mutex 字段，先 lock 并处理 Result，再获取长度
        let on_task_start_len = self.on_task_start.lock().unwrap().len();
        let on_task_complete_len = self.on_task_complete.lock().unwrap().len();
        let on_start_len = self.on_start.lock().unwrap().len();
        let on_stop_len = self.on_stop.lock().unwrap().len();

        f.debug_struct("TaskRunner")
            .field("groups", &self.groups)
            .field("tasks", &self.tasks)
            .field("is_running", &self.is_running)
            .field("on_task_start", &on_task_start_len)
            .field("on_task_complete", &on_task_complete_len)
            .field("on_start", &on_start_len)
            .field("on_stop", &on_stop_len)
            .field("global_join_set", &self.global_join_set)
            .finish()
    }
}

impl TaskRunner {
    // 创建新的任务运行器
    pub fn new(default_concurrency: usize) -> Self {
        trace!("Creating new TaskRunner with default concurrency: {}", default_concurrency);
        
        let groups = Arc::new(Mutex::new(HashMap::new()));
        // 初始化默认组
        let default_group = TaskGroup {
            dependencies: Vec::new(),
            queue: Arc::new(Mutex::new(BinaryHeap::new())),
            semaphore: Arc::new(Semaphore::new(default_concurrency)),
            join_set: Arc::new(Mutex::new(JoinSet::new())),
            notify: Arc::new(Notify::new()),
        };
        groups
            .lock()
            .unwrap()
            .insert("default".to_string(), default_group);
        
        info!("TaskRunner created with default group");

        Self {
            groups,
            tasks: Arc::new(Mutex::new(HashMap::new())),
            is_running: Arc::new(AtomicBool::new(false)),
            global_join_set: Arc::new(Mutex::new(JoinSet::new())),
            on_task_start: Arc::new(Mutex::new(Vec::new())),
            on_task_complete: Arc::new(Mutex::new(Vec::new())),
            on_start: Arc::new(Mutex::new(Vec::new())),
            on_stop: Arc::new(Mutex::new(Vec::new())),
            default_concurrency,
        }
    }

    // 创建任务组   ？？？？ 这里依赖是什么东西，为什么要检查循环依赖（需要先完成的组先完成）
    pub fn create_group(&self, id: &str, options: TaskGroupOptions) -> Result<(), TaskRunnerError> {
        trace!("Attempting to create group: {}", id);
        
        let mut groups = self.groups.lock().unwrap();
        if groups.contains_key(id) {
            error!("Failed to create group '{}': already exists", id);
            return Err(TaskRunnerError::GroupExists(id.to_string()));
        }

        let deps = options.after_group.unwrap_or_default(); // 获取当前组需要先完成的组
        trace!("Group '{}' has dependencies: {:?}", id, deps);
        
        // 检查依赖是否存在 判断当前组依赖的组是否在 全局 groups 中存在
        for dep in &deps {
            if !groups.contains_key(dep) {
                error!("Failed to create group '{}': dependency '{}' not found", id, dep);
                return Err(TaskRunnerError::DependencyNotFound(
                    dep.clone(),
                    id.to_string(),
                ));
            }
        }

        // 检测循环依赖
        let mut dep_graph: HashMap<_, _> = groups
            .iter()
            .map(|(k, v)| (k.clone(), v.dependencies.clone()))
            .collect();
        dep_graph.insert(id.to_string(), deps.clone());
        if Self::detect_cycle(&dep_graph) {
            error!("Failed to create group '{}': circular dependency detected", id);
            return Err(TaskRunnerError::CircularDependency(id.to_string()));
        }

        // 创建组
        let concurrency = options.concurrency.unwrap_or(self.default_concurrency);
        groups.insert(
            id.to_string(),
            TaskGroup {
                dependencies: deps,
                queue: Arc::new(Mutex::new(BinaryHeap::new())),
                semaphore: Arc::new(Semaphore::new(concurrency)),
                join_set: Arc::new(Mutex::new(JoinSet::new())),
                notify: Arc::new(Notify::new()),
            },
        );
        
        info!("Created group '{}' with concurrency: {}", id, concurrency);
        Ok(())
    }

    // 删除任务组
    pub fn remove_group(&self, id: &str) -> Result<(), TaskRunnerError> {
        trace!("Attempting to remove group: {}", id);
        
        if id == "default" {
            error!("Attempted to remove default group");
            return Err(TaskRunnerError::CannotRemoveDefault);
        }

        let groups = self.groups.lock().unwrap();
        // 检查是否有其他组依赖当前组
        for (group, info) in groups.iter() {
            if info.dependencies.contains(&id.to_string()) {
                error!("Failed to remove group '{}': still depended on by '{}'", id, group);
                return Err(TaskRunnerError::GroupDependedOn(
                    id.to_string(),
                    group.clone(),
                ));
            }
        }
        drop(groups); // 提前释放锁

        self.groups.lock().unwrap().remove(id);
        info!("Removed group '{}'", id);
        Ok(())
    }

    // 检测循环依赖
    fn detect_cycle(dep_graph: &HashMap<String, Vec<String>>) -> bool {
        trace!("Checking for circular dependencies in graph: {:?}", dep_graph);
        
        #[derive(PartialEq, Eq, Clone, Copy)]
        enum State {
            New,
            Visiting,
            Visited,
        }
        let mut states: HashMap<String, State> = HashMap::new();
        for k in dep_graph.keys() {
            states.insert(k.clone(), State::New);
        }

        fn dfs(
            node: &str,
            graph: &HashMap<String, Vec<String>>,
            states: &mut HashMap<String, State>,
        ) -> bool {
            trace!("DFS checking node: {}", node);
            states.insert(node.to_string(), State::Visiting);
            if let Some(neighbors) = graph.get(node) {
                for neighbor in neighbors {
                    // neighbor might be absent in states map if graph is inconsistent; handle robustly
                    match states.get(neighbor).copied() {
                        Some(State::Visiting) => {
                            trace!("Cycle detected between {} and {}", node, neighbor);
                            return true;
                        }
                        Some(State::New) => {
                            if dfs(neighbor, graph, states) {
                                return true;
                            }
                        }
                        Some(State::Visited) => continue,
                        None => {
                            // treat missing as New and continue DFS
                            if dfs(neighbor, graph, states) {
                                return true;
                            }
                        }
                    }
                }
            }
            states.insert(node.to_string(), State::Visited);
            false
        }

        let has_cycle = dep_graph
            .keys()
            .any(|k| states.get(k) == Some(&State::New) && dfs(k, dep_graph, &mut states));
            
        if has_cycle {
            trace!("Circular dependency detected in graph");
        } else {
            trace!("No circular dependencies found");
        }
        has_cycle
    }

    // 注册任务：注意这里要求 F: Fn() -> Fut（而不是 FnOnce）
    pub fn register<F, Fut, R, E>(
        &self,
        id: &str,
        func: F,
        options: RegisterOptions,
    ) -> Result<(), TaskRunnerError>
    where
        F: Fn() -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<R, E>> + Send + 'static,
        R: Any + Send + 'static,
        E: Error + Send + 'static,
    {
        trace!("Attempting to register task: {}", id);
        
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            error!("Failed to register task '{}': runner is running", id);
            return Err(TaskRunnerError::AddingTaskWhileRunning);
        }

        let mut tasks = self.tasks.lock().unwrap();
        if tasks.contains_key(id) {
            error!("Failed to register task '{}': already exists", id);
            return Err(TaskRunnerError::TaskExists(id.to_string()));
        }

        let group = options.group.unwrap_or_else(|| "default".to_string());
        let groups = self.groups.lock().unwrap();
        if !groups.contains_key(&group) {
            error!("Failed to register task '{}': group '{}' not found", id, group);
            return Err(TaskRunnerError::GroupNotFound(group));
        }

        // 包装任务函数为统一类型（Fn -> boxed future returning Box<dyn Any>）
        let wrapped_func = {
            let func = std::sync::Arc::new(func);
            Box::new(move || {
                let func = func.clone();
                let fut: Pin<
                    Box<
                        dyn Future<Output = Result<Box<dyn Any + Send>, Box<dyn Error + Send>>>
                            + Send,
                    >,
                > = Box::pin(async move {
                    (func)()
                        .await
                        .map(|val| Box::new(val) as Box<dyn Any + Send>)
                        .map_err(|e| Box::new(e) as Box<dyn Error + Send>)
                });
                fut
            })
                as Box<
                    dyn Fn() -> Pin<
                            Box<
                                dyn Future<
                                        Output = Result<Box<dyn Any + Send>, Box<dyn Error + Send>>,
                                    > + Send,
                            >,
                        > + Send
                        + Sync,
                >
        };

        tasks.insert(
            id.to_string(),
            Task {
                id: id.to_string(),
                func: wrapped_func,
                priority: options.priority.unwrap_or(50),
                group: group.clone(),
            },
        );
        
        info!("Registered task '{}' in group '{}' with priority {}", id, group, options.priority.unwrap_or(50));
        Ok(())
    }

    // 删除任务
    pub fn remove_task(&self, id: &str) -> Result<(), TaskRunnerError> {
        trace!("Attempting to remove task: {}", id);
        
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            error!("Failed to remove task '{}': runner is running", id);
            return Err(TaskRunnerError::RemovingTaskWhileRunning);
        }

        let mut tasks = self.tasks.lock().unwrap();
        if tasks.remove(id).is_some() {
            info!("Removed task '{}'", id);
        } else {
            warn!("Task '{}' not found when trying to remove", id);
        }
        Ok(())
    }

    // 启动任务运行
    pub async fn start(&self) -> Result<(), TaskRunnerError> {
        trace!("Attempting to start TaskRunner");
        
        if self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            error!("Failed to start TaskRunner: already running");
            return Err(TaskRunnerError::AlreadyRunning);
        }
        self.is_running
            .store(true, std::sync::atomic::Ordering::SeqCst);

        // 清空之前的全局任务集（防止重复启动）
        {
            let mut g = self.global_join_set.lock().unwrap();
            g.abort_all();
            // create new JoinSet to ensure no stale tasks remain tracked
            *g = JoinSet::new();
            trace!("Cleared global join set");
        }

        // 触发start事件
        {
            let cbs = self.on_start.lock().unwrap();
            trace!("Triggering on_start callbacks (count: {})", cbs.len());
            for cb in cbs.iter() {
                cb();
            }
        }

        // 将任务加入对应组的队列（把 tasks drain 出来）
        let tasks_vec: Vec<_> = {
            let mut tasks_lock = self.tasks.lock().unwrap();
            let count = tasks_lock.len();
            let tasks = tasks_lock.drain().map(|(_, v)| Arc::new(v)).collect();
            trace!("Drained {} tasks from registry", count);
            tasks
        };
        {
            let mut groups = self.groups.lock().unwrap();
            for task in &tasks_vec {
                let group_id = &task.group;
                if let Some(group) = groups.get_mut(group_id) {
                    group.queue.lock().unwrap().push(PrioritizedTask(task.clone()));
                    trace!("Added task '{}' to group '{}' queue", task.id, group_id);
                } else {
                    warn!("Group '{}' not found for task '{}'", task.group, task.id);
                }
            }
            info!("Distributed {} tasks to their respective groups", tasks_vec.len());
        }

        // 收集组的克隆（避免持有 MutexGuard 跨 await）
        let groups_snapshot: Vec<(String, TaskGroup)> = {
            let groups = self.groups.lock().unwrap();
            let groups: Vec<_> = groups.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            trace!("Captured snapshot of {} groups", groups.len());
            groups
        };

        let on_task_start = self.on_task_start.clone();
        let on_task_complete = self.on_task_complete.clone();
        let is_running = self.is_running.clone();
        let global_join_set = self.global_join_set.clone();
        let groups_arc = self.groups.clone();

        // 启动每个组的执行逻辑（把组处理任务放入 global_join_set）
        {
            let mut g = global_join_set.lock().unwrap();
            for (group_id, group) in groups_snapshot.into_iter() {
                let on_task_start = on_task_start.clone();
                let on_task_complete = on_task_complete.clone();
                let is_running = is_running.clone();
                let groups_arc_inner = groups_arc.clone();

                trace!("Spawning task processor for group '{}'", group_id);
                // 为每个组 spawn 一个处理任务到全局 JoinSet
                g.spawn(async move {
                    // 先等待依赖组完成（如果有）
                    if !group.dependencies.is_empty() {
                        trace!("Group '{}' waiting on dependencies: {:?}", group_id, group.dependencies);
                        // 为每个依赖组获取其 notify，并等待 notified
                        let dep_notifies: Vec<_> = {
                            let groups_map = groups_arc_inner.lock().unwrap();
                            group
                                .dependencies
                                .iter()
                                .filter_map(|dep_id| {
                                    groups_map.get(dep_id).map(|g| g.notify.clone())
                                })
                                .collect()
                        };
                        // 等待全部依赖组 notify
                        let wait_futs = dep_notifies.iter().map(|n| n.notified());
                        join_all(wait_futs).await;
                        trace!("Group '{}' dependencies completed", group_id);
                    }

                    // 开始执行当前组任务
                    Self::run_group_tasks(
                        &group_id,
                        &group,
                        on_task_start,
                        on_task_complete,
                        is_running,
                    )
                    .await;
                });
            }
            info!("Started processing for all groups");
        }

        // 等待所有组完成后触发 stop 事件（放到 global_join_set）
        {
            let on_stop = self.on_stop.clone();
            let is_running = self.is_running.clone();
            let groups_for_wait = self.groups.clone();
            let mut g = self.global_join_set.lock().unwrap();

            trace!("Spawning global completion monitor");
            g.spawn(async move {
                // 等待所有组的 notify（每个组 notify 一次代表该组完成）
                let notifies: Vec<_> = {
                    let groups_map = groups_for_wait.lock().unwrap();
                    groups_map.values().map(|g| g.notify.clone()).collect()
                };

                // wait all
                let wait_futs = notifies.iter().map(|n| n.notified());
                join_all(wait_futs).await;

                trace!("All groups completed, triggering on_stop callbacks");
                // 触发 stop 回调
                let cbs = on_stop.lock().unwrap();
                for cb in cbs.iter() {
                    cb();
                }
                is_running.store(false, std::sync::atomic::Ordering::SeqCst);
                info!("TaskRunner stopped normally");
            });
        }

        info!("TaskRunner started successfully");
        Ok(())
    }

    /// 运行组内任务
    /// 新增参数 is_running: 用于检查全局停止标志
    async fn run_group_tasks(
        group_id: &str,
        group: &TaskGroup,
        on_task_start: Arc<Mutex<Vec<Box<dyn Fn(Arc<TaskStartPayload>) + Send + Sync>>>>,
        on_task_complete: Arc<Mutex<Vec<Box<dyn Fn(Arc<TaskCompletePayload>) + Send + Sync>>>>,
        is_running: Arc<AtomicBool>,
    ) {
        let group = group.clone();
        let group_id = group_id.to_string();
        trace!("Starting task processing loop for group '{}'", group_id);

        // 我们在这里开启一个独立的 tokio task 来循环消费队列中的任务
        tokio::spawn(async move {
            loop {
                // 检查全局停止标志
                if !is_running.load(std::sync::atomic::Ordering::SeqCst) {
                    trace!("Group '{}' detected runner stop, clearing queue", group_id);
                    // 取消时清空队列并通知依赖组
                    group.queue.lock().unwrap().clear();
                    group.notify.notify_waiters();
                    break;
                }

                // 从队列取任务（只在临界区短暂持有锁）
                let maybe_task = {
                    let mut queue = group.queue.lock().unwrap();
                    queue.pop().map(|pt| pt.0)
                };

                let task = match maybe_task {
                    Some(t) => t,
                    None => {
                        // 如果队列为空，检查是否还有运行中的任务（通过 join_set 的活动数）
                        let join_len = {
                            let js = group.join_set.lock().unwrap();
                            js.len()
                        };
                        if join_len == 0 {
                            trace!("Group '{}' has empty queue and no running tasks, completing", group_id);
                            // 队列空且无运行中任务，通知依赖组并退出
                            group.notify.notify_waiters();
                            break;
                        }
                        // 否则等待一段时间再继续检查
                        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                        continue;
                    }
                };

                // 获取信号量许可控制并发
                let permit = match group.semaphore.acquire().await {
                    Ok(p) => p,
                    Err(_) => {
                        error!("Semaphore error for group '{}', stopping task processing", group_id);
                        // Semaphore 已关闭或出错，退出循环
                        break;
                    }
                };

                let task_id = task.id.clone();
                let start_cb = on_task_start.clone();
                let complete_cb = on_task_complete.clone();

                trace!("Scheduling task '{}' in group '{}'", task_id, group_id);
                // 将任务放入组的 join_set，以便可以统一 cancel/track
                {
                    let mut join_set = group.join_set.lock().unwrap();
                    join_set.spawn(async move {
                        trace!("Starting execution of task '{}'", task_id);
                        // 触发 task-start 事件
                        let start_payload = Arc::new(TaskStartPayload {
                            id: task_id.clone(),
                        });
                        {
                            let start_cbs = start_cb.lock().unwrap();
                            trace!("Triggering on_task_start for '{}' ({} callbacks)", task_id, start_cbs.len());
                            for cb in start_cbs.iter() {
                                cb(start_payload.clone());
                            }
                        }

                        // 执行任务函数（捕获 panic）
                        let result = match tokio::task::spawn(async move { (task.func)().await }).await {
                            Ok(Ok(val)) => Ok(val),
                            Ok(Err(e)) => {
                                error!("Task '{}' failed with error: {}", task_id, e);
                                Err(e)
                            }
                            Err(_panic) => {
                                let err = Box::new(TaskRunnerError::TaskPanicError(task_id.clone()))
                                    as Box<dyn Error + Send>;
                                error!("Task '{}' panicked", task_id);
                                Err(err)
                            }
                        };

                        // 触发 task-complete 事件
                        let complete_payload = match result {
                            Ok(val) => {
                                trace!("Task '{}' completed successfully", task_id);
                                Arc::new(TaskCompletePayload::Success {
                                    id: task_id.clone(),
                                    value: val,
                                })
                            }
                            Err(e) => {
                                warn!("Task '{}' completed with error", task_id);
                                Arc::new(TaskCompletePayload::Error {
                                    id: task_id.clone(),
                                    error: e,
                                })
                            }
                        };
                        {
                            let complete_cbs = complete_cb.lock().unwrap();
                            trace!("Triggering on_task_complete for '{}' ({} callbacks)", task_id, complete_cbs.len());
                            for cb in complete_cbs.iter() {
                                cb(complete_payload.clone());
                            }
                        }

                    });
                }
                
                // drop permit -> 释放信号量许可（permit 在此函数返回时自动 drop）
                drop(permit);
            }
            info!("Group '{}' task processing loop exited", group_id);
        });
    }

    // 停止任务运行（通过取消 global joinsets & 各组 joinset）
    pub fn stop(&self) -> Result<(), TaskRunnerError> {
        trace!("Attempting to stop TaskRunner");
        
        if !self.is_running.load(std::sync::atomic::Ordering::SeqCst) {
            error!("Failed to stop TaskRunner: not running");
            return Err(TaskRunnerError::NotRunning);
        }

        // 1. 取消所有全局任务
        {
            let mut global = self.global_join_set.lock().unwrap();
            let task_count = global.len();
            global.abort_all();
            // 重置为空的 JoinSet（使得下一次 start 能创建干净的 JoinSet）
            *global = JoinSet::new();
            trace!("Aborted {} global tasks", task_count);
        }

        // 2. 取消所有组内运行中的任务
        {
            let groups = self.groups.lock().unwrap();
            for (group_id, group) in groups.iter() {
                let mut join_set = group.join_set.lock().unwrap();
                let task_count = join_set.len();
                join_set.abort_all();
                // 清空队列
                group.queue.lock().unwrap().clear();
                // 通知依赖组
                group.notify.notify_waiters();
                trace!("Aborted {} tasks in group '{}'", task_count, group_id);
            }
        }

        // 3. 标记状态
        self.is_running
            .store(false, std::sync::atomic::Ordering::SeqCst);
        
        info!("TaskRunner stopped successfully");
        Ok(())
    }

    // 注册事件回调
    pub fn on_task_start<F: Fn(Arc<TaskStartPayload>) + Send + Sync + 'static>(&self, cb: F) {
        trace!("Registering on_task_start callback");
        self.on_task_start.lock().unwrap().push(Box::new(cb));
    }

    pub fn on_task_complete<F: Fn(Arc<TaskCompletePayload>) + Send + Sync + 'static>(&self, cb: F) {
        trace!("Registering on_task_complete callback");
        self.on_task_complete.lock().unwrap().push(Box::new(cb));
    }

    pub fn on_start<F: Fn() + Send + Sync + 'static>(&self, cb: F) {
        trace!("Registering on_start callback");
        self.on_start.lock().unwrap().push(Box::new(cb));
    }

    pub fn on_stop<F: Fn() + Send + Sync + 'static>(&self, cb: F) {
        trace!("Registering on_stop callback");
        self.on_stop.lock().unwrap().push(Box::new(cb));
    }
}