// use futures::channel::mpsc;
// use futures::StreamExt;
// use serde::{Deserialize, Serialize};
// use std::collections::{BinaryHeap, HashMap};
// use std::future::Future;
// use std::pin::Pin;
// use std::sync::atomic::{AtomicBool, Ordering};
// use std::sync::Arc;
// use tokio::sync::{Mutex, Notify, Semaphore};

// /// 任务开始事件载荷
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TaskStartPayload {
//     pub id: String,
// }

// /// 任务成功完成事件载荷
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TaskSuccessPayload {
//     pub id: String,
//     pub status: String, // "fulfilled"
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub value: Option<serde_json::Value>,
// }

// /// 任务错误事件载荷
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct TaskErrorPayload {
//     pub id: String,
//     pub status: String, // "rejected"
//     pub error: String,
// }

// /// 任务完成事件载荷（成功或失败）
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(tag = "status")]
// pub enum TaskCompletePayload {
//     #[serde(rename = "fulfilled")]
//     Success(TaskSuccessPayload),
//     #[serde(rename = "rejected")]
//     Error(TaskErrorPayload),
// }

// /// 已调度的任务
// pub struct ScheduledTask {
//     pub id: String,
//     pub func: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<serde_json::Value, String>> + Send>> + Send + Sync>,
//     pub priority: Option<i32>,
//     pub group: String,
// }

// /// 任务组选项
// #[derive(Debug, Clone)]
// pub struct TaskGroupOptions {
//     pub concurrency: Option<usize>,
//     pub after_group: Option<Vec<String>>,
// }

// /// 任务组
// struct TaskGroup {
//     queue: Arc<Mutex<BinaryHeap<PrioritizedTask>>>,
//     semaphore: Arc<Semaphore>,
//     dependencies: Vec<String>,
//     notify: Arc<Notify>,
//     paused: Arc<AtomicBool>,
// }

// /// 带优先级的任务（用于优先队列）
// #[derive(Clone)]
// struct PrioritizedTask {
//     task: Arc<ScheduledTask>,
// }

// impl Ord for PrioritizedTask {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         // BinaryHeap 是最大堆，priority 越大越先执行
//         // 如果 priority 为 None，则默认为 0
//         let self_priority = self.task.priority.unwrap_or(0);
//         let other_priority = other.task.priority.unwrap_or(0);
//         other_priority.cmp(&self_priority)
//     }
// }

// impl PartialOrd for PrioritizedTask {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for PrioritizedTask {
//     fn eq(&self, other: &Self) -> bool {
//         self.task.priority.unwrap_or(0) == other.task.priority.unwrap_or(0)
//     }
// }

// impl Eq for PrioritizedTask {}

// /// 任务运行器事件类型
// #[derive(Debug, Clone)]
// pub enum TaskRunnerEvent {
//     TaskStart(TaskStartPayload),
//     TaskComplete(TaskCompletePayload),
//     Start,
//     Stop,
// }

// /// 批量任务运行工具
// pub struct TaskRunner {
//     queues: Arc<Mutex<HashMap<String, TaskGroup>>>,
//     group_deps: Arc<Mutex<HashMap<String, Vec<String>>>>,
//     tasks: Arc<Mutex<HashMap<String, ScheduledTask>>>,
//     controller: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
//     is_running: Arc<AtomicBool>,
//     event_tx: mpsc::UnboundedSender<TaskRunnerEvent>,
//     event_rx: Arc<Mutex<mpsc::UnboundedReceiver<TaskRunnerEvent>>>,
//     default_concurrency: usize,
// }

// impl TaskRunner {
//     /// 创建新的任务运行器
//     pub fn new(default_concurrency: usize) -> Self {
//         let (tx, rx) = mpsc::unbounded();
        
//         let queues = Arc::new(Mutex::new(HashMap::new()));
//         let group_deps = Arc::new(Mutex::new(HashMap::new()));
        
//         // 创建默认组
//         let default_group = TaskGroup {
//             queue: Arc::new(Mutex::new(BinaryHeap::new())),
//             semaphore: Arc::new(Semaphore::new(default_concurrency)),
//             dependencies: Vec::new(),
//             notify: Arc::new(Notify::new()),
//             paused: Arc::new(AtomicBool::new(false)),
//         };
        
//         let mut queues_guard = queues.blocking_lock();
//         queues_guard.insert("default".to_string(), default_group);
//         group_deps.blocking_lock().insert("default".to_string(), Vec::new());
//         drop(queues_guard);
        
//         Self {
//             queues,
//             group_deps,
//             tasks: Arc::new(Mutex::new(HashMap::new())),
//             controller: Arc::new(Mutex::new(None)),
//             is_running: Arc::new(AtomicBool::new(false)),
//             event_tx: tx,
//             event_rx: Arc::new(Mutex::new(rx)),
//             default_concurrency,
//         }
//     }

//     /// 创建任务组
//     pub async fn create_group(&self, id: &str, options: TaskGroupOptions) -> Result<(), String> {
//         let mut queues = self.queues.lock().await;
//         if queues.contains_key(id) {
//             return Err(format!("Task group \"{}\" already exists", id));
//         }

//         let deps = options.after_group.unwrap_or_default();
//         let mut group_deps = self.group_deps.lock().await;

//         // 检查依赖是否存在
//         for dep in &deps {
//             if !queues.contains_key(dep) {
//                 return Err(format!("Dependency group \"{}\" not found for group \"{}\"", dep, id));
//             }
//         }

//         // 检查循环依赖
//         let mut tmp_deps = group_deps.clone();
//         tmp_deps.insert(id.to_string(), deps.clone());
//         if Self::detect_cycle(&tmp_deps) {
//             return Err(format!("Circular dependency detected when adding group \"{}\"", id));
//         }

//         // 创建队列并根据是否有依赖暂停
//         let concurrency = options.concurrency.unwrap_or(self.default_concurrency);
//         let group = TaskGroup {
//             queue: Arc::new(Mutex::new(BinaryHeap::new())),
//             semaphore: Arc::new(Semaphore::new(concurrency)),
//             dependencies: deps.clone(),
//             notify: Arc::new(Notify::new()),
//             paused: Arc::new(AtomicBool::new(!deps.is_empty())),
//         };

//         queues.insert(id.to_string(), group);
//         group_deps.insert(id.to_string(), deps);
//         Ok(())
//     }

//     /// 删除任务组
//     pub async fn remove_group(&self, id: &str) -> Result<bool, String> {
//         if id == "default" {
//             return Err("Cannot remove the default group".to_string());
//         }

//         let group_deps = self.group_deps.lock().await;
//         // 检查是否有其他组依赖于此组
//         for (g, deps) in group_deps.iter() {
//             if deps.contains(&id.to_string()) {
//                 return Err(format!(
//                     "Cannot remove group \"{}\", it is depended upon by group \"{}\"",
//                     id, g
//                 ));
//             }
//         }
//         drop(group_deps);

//         let mut queues = self.queues.lock().await;
//         let mut group_deps = self.group_deps.lock().await;
//         group_deps.remove(id);
//         Ok(queues.remove(id).is_some())
//     }

//     /// 注册任务
//     pub async fn register<F, Fut>(
//         &self,
//         id: &str,
//         func: F,
//         options: Option<TaskRegisterOptions>,
//     ) -> Result<(), String>
//     where
//         F: Fn() -> Fut + Send + Sync + 'static,
//         Fut: Future<Output = Result<serde_json::Value, String>> + Send + 'static,
//     {
//         if self.is_running.load(Ordering::SeqCst) {
//             return Err("Cannot add tasks while running".to_string());
//         }

//         let mut tasks = self.tasks.lock().await;
//         if tasks.contains_key(id) {
//             return Err(format!("Task with id \"{}\" already exists", id));
//         }

//         let opts = options.unwrap_or_default();
//         let group = opts.group.unwrap_or_else(|| "default".to_string());

//         let queues = self.queues.lock().await;
//         if !queues.contains_key(&group) {
//             return Err(format!("No task group \"{}\" found", group));
//         }
//         drop(queues);

//         let func_boxed: Arc<dyn Fn() -> Pin<Box<dyn Future<Output = Result<serde_json::Value, String>> + Send>> + Send + Sync> =
//             Arc::new(move || {
//                 let fut = func();
//                 Box::pin(fut)
//             });

//         tasks.insert(
//             id.to_string(),
//             ScheduledTask {
//                 id: id.to_string(),
//                 func: func_boxed,
//                 priority: opts.priority,
//                 group,
//             },
//         );

//         Ok(())
//     }

//     /// 删除任务
//     pub async fn remove(&self, id: &str) -> Result<bool, String> {
//         if self.is_running.load(Ordering::SeqCst) {
//             return Err("Cannot remove tasks while running".to_string());
//         }

//         let mut tasks = self.tasks.lock().await;
//         Ok(tasks.remove(id).is_some())
//     }

//     /// 启动任务执行
//     pub async fn start(&self) -> Result<(), String> {
//         if self.is_running.load(Ordering::SeqCst) {
//             return Err("TaskRunner is already running".to_string());
//         }

//         self.is_running.store(true, Ordering::SeqCst);
//         self.emit_event(TaskRunnerEvent::Start).await;

//         let queues = self.queues.clone();
//         let group_deps = self.group_deps.clone();
//         let tasks = self.tasks.clone();
//         let is_running = self.is_running.clone();
//         let event_tx = self.event_tx.clone();

//         // 启动任务执行循环
//         let handle = tokio::spawn(async move {
//             // 将所有任务分发到对应的组队列
//             {
//                 let tasks_guard = tasks.lock().await;
//                 let queues_guard = queues.lock().await;

//                 for task in tasks_guard.values() {
//                     if let Some(group) = queues_guard.get(&task.group) {
//                         let prioritized = PrioritizedTask {
//                             task: Arc::new(ScheduledTask {
//                                 id: task.id.clone(),
//                                 func: task.func.clone(),
//                                 priority: task.priority,
//                                 group: task.group.clone(),
//                             }),
//                         };
//                         group.queue.lock().await.push(prioritized);
//                     }
//                 }
//             }

//             // 为每个组启动处理任务
//             let mut group_handles = Vec::new();
//             {
//                 let queues_guard = queues.lock().await;

//                 for (_group_id, group) in queues_guard.iter() {
//                     let group_clone = TaskGroup {
//                         queue: group.queue.clone(),
//                         semaphore: group.semaphore.clone(),
//                         dependencies: group.dependencies.clone(),
//                         notify: group.notify.clone(),
//                         paused: group.paused.clone(),
//                     };
//                     let queues_clone = queues.clone();
//                     let is_running_clone = is_running.clone();
//                     let event_tx_clone = event_tx.clone();

//                     let handle = tokio::spawn(async move {
//                         // 等待依赖组完成
//                         if !group_clone.dependencies.is_empty() {
//                             let mut wait_handles = Vec::new();
                            
//                             {
//                                 let queues_guard = queues_clone.lock().await;
//                                 for dep_id in &group_clone.dependencies {
//                                     if let Some(dep_group) = queues_guard.get(dep_id) {
//                                         let notify = dep_group.notify.clone();
//                                         wait_handles.push(tokio::spawn(async move {
//                                             notify.notified().await;
//                                         }));
//                                     }
//                                 }
//                             }

//                             // 等待所有依赖组完成
//                             futures::future::join_all(wait_handles).await;
//                         }

//                         // 启动当前组的任务处理
//                         group_clone.paused.store(false, Ordering::SeqCst);
                        
//                         // 跟踪正在执行的任务数量
//                         let active_tasks = Arc::new(tokio::sync::Mutex::new(0u32));
//                         let active_tasks_clone = active_tasks.clone();
//                         let notify_clone = group_clone.notify.clone();

//                         // 处理队列中的任务
//                         loop {
//                             if !is_running_clone.load(Ordering::SeqCst) {
//                                 break;
//                             }

//                             // 从队列中取任务
//                             let maybe_task = {
//                                 let mut queue = group_clone.queue.lock().await;
//                                 queue.pop().map(|pt| pt.task.clone())
//                             };

//                             if let Some(task) = maybe_task {
//                                 let semaphore_clone = group_clone.semaphore.clone();
                                
//                                 // 获取信号量许可
//                                 let permit = semaphore_clone.acquire().await.unwrap();
                                
//                                 // 增加活跃任务计数
//                                 *active_tasks_clone.lock().await += 1;

//                                 let task_id = task.id.clone();
//                                 let task_func = task.func.clone();
//                                 let event_tx_task = event_tx_clone.clone();
//                                 let active_tasks_task = active_tasks_clone.clone();
//                                 let notify_task = notify_clone.clone();
//                                 let queue_clone = group_clone.queue.clone();

//                                 // 发送任务开始事件
//                                 let _ = event_tx_task.unbounded_send(TaskRunnerEvent::TaskStart(
//                                     TaskStartPayload { id: task_id.clone() },
//                                 ));

//                                 // 执行任务
//                                 tokio::spawn(async move {
//                                     let result = (task_func)().await;

//                                     let payload = match result {
//                                         Ok(value) => TaskRunnerEvent::TaskComplete(
//                                             TaskCompletePayload::Success(TaskSuccessPayload {
//                                                 id: task_id.clone(),
//                                                 status: "fulfilled".to_string(),
//                                                 value: Some(value),
//                                             }),
//                                         ),
//                                         Err(error) => TaskRunnerEvent::TaskComplete(
//                                             TaskCompletePayload::Error(TaskErrorPayload {
//                                                 id: task_id.clone(),
//                                                 status: "rejected".to_string(),
//                                                 error,
//                                             }),
//                                         ),
//                                     };

//                                     let _ = event_tx_task.unbounded_send(payload);
                                    
//                                     // 减少活跃任务计数
//                                     let mut active = active_tasks_task.lock().await;
//                                     *active -= 1;
                                    
//                                     // 如果任务完成且队列为空，检查是否需要通知
//                                     let queue_empty = {
//                                         let queue = queue_clone.lock().await;
//                                         queue.is_empty()
//                                     };
                                    
//                                     if *active == 0 && queue_empty {
//                                         notify_task.notify_waiters();
//                                     }
                                    
//                                     drop(permit);
//                                 });
//                             } else {
//                                 // 队列为空，检查是否所有任务都完成了
//                                 let active = *active_tasks_clone.lock().await;
//                                 if active == 0 {
//                                     // 通知依赖此组的其他组
//                                     notify_clone.notify_waiters();
//                                     break;
//                                 }
                                
//                                 // 等待一小段时间后重试
//                                 tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
//                             }
//                         }
//                     });

//                     group_handles.push(handle);
//                 }
//             }

//             // 等待所有组完成
//             futures::future::join_all(group_handles).await;

//             is_running.store(false, Ordering::SeqCst);
//             let _ = event_tx.unbounded_send(TaskRunnerEvent::Stop);
//         });

//         *self.controller.lock().await = Some(handle);
//         Ok(())
//     }

//     /// 停止任务执行
//     pub async fn stop(&self) -> Result<(), String> {
//         if !self.is_running.load(Ordering::SeqCst) {
//             return Err("TaskRunner is not running".to_string());
//         }

//         self.is_running.store(false, Ordering::SeqCst);

//         if let Some(handle) = self.controller.lock().await.take() {
//             handle.abort();
//         }

//         Ok(())
//     }

//     /// 获取是否正在运行
//     pub fn is_running(&self) -> bool {
//         self.is_running.load(Ordering::SeqCst)
//     }

//     /// 获取任务列表
//     pub async fn get_tasks(&self) -> Vec<String> {
//         let tasks = self.tasks.lock().await;
//         tasks.keys().cloned().collect()
//     }

//     /// 监听事件
//     pub async fn next_event(&self) -> Option<TaskRunnerEvent> {
//         let mut rx = self.event_rx.lock().await;
//         rx.next().await
//     }

//     /// 发送事件
//     async fn emit_event(&self, event: TaskRunnerEvent) {
//         let _ = self.event_tx.unbounded_send(event);
//     }

//     /// 检测循环依赖
//     fn detect_cycle(dep_graph: &HashMap<String, Vec<String>>) -> bool {
//         enum VisitState {
//             White,  // 未访问
//             Gray,   // 正在访问（DFS 中）
//             Black,  // 已访问完成
//         }

//         let mut colors: HashMap<String, VisitState> = dep_graph
//             .keys()
//             .map(|k| (k.clone(), VisitState::White))
//             .collect();

//         fn dfs(node: &str, dep_graph: &HashMap<String, Vec<String>>, colors: &mut HashMap<String, VisitState>) -> bool {
//             colors.insert(node.to_string(), VisitState::Gray);

//             if let Some(deps) = dep_graph.get(node) {
//                 for dep in deps {
//                     match colors.get(dep) {
//                         Some(VisitState::Gray) => return true, // 发现环
//                         Some(VisitState::White) => {
//                             if dfs(dep, dep_graph, colors) {
//                                 return true;
//                             }
//                         }
//                         _ => {}
//                     }
//                 }
//             }

//             colors.insert(node.to_string(), VisitState::Black);
//             false
//         }

//         for key in dep_graph.keys() {
//             if matches!(colors.get(key), Some(VisitState::White)) {
//                 if dfs(key, dep_graph, &mut colors) {
//                     return true;
//                 }
//             }
//         }

//         false
//     }
// }

// /// 任务注册选项
// #[derive(Debug, Default, Clone)]
// pub struct TaskRegisterOptions {
//     pub priority: Option<i32>,
//     pub group: Option<String>,
// }

