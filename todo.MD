使用 Rust 重构 LeagueAkari 项目需要结合原项目的架构特点、功能模块及 Rust 生态的优势，分阶段有序推进。以下是详细的重构流程分析：

### **一、重构前的准备阶段**

#### 1. 需求与功能梳理

- **核心功能确认**：基于原项目分析，明确必须保留的核心功能，包括：

  - LCU API 交互（匹配状态监听、游戏数据获取）。
  - 比赛数据分析（KDA 计算、预组队分析、战绩查询）。
  - 客户端交互（窗口管理、WMI 修复、游戏内消息发送）。
  - 数据持久化（SQLite 数据库操作，替代原 Local Storage）。
  - 自动更新机制（检查更新、下载、安装包处理）。
  - 跨进程通信（主进程与渲染进程的数据同步）。

- **非核心功能评估**：如 UI 渲染逻辑、第三方依赖集成（如 OP.GG 页面嵌入），确定是否需要完全重构或保留现有前端技术栈。

#### 2. 技术栈选型

根据 Rust 生态和项目需求，选择适配的工具链：

- **核心框架**：
  - 替代 Electron 主进程：使用 `tauri`（轻量跨平台框架，Rust 作为后端，前端可复用原 Web 技术栈）。
  - 纯 Rust 渲染（可选）：若完全重构 UI，可使用 `yew` 或 `dioxus`（WebAssembly 前端框架）。
- **网络通信**：
  - HTTP 客户端：`reqwest`（替代 Axios，处理 LCU API 请求）。
  - WebSocket：`tokio-tungstenite`（监听 LCU 实时事件）。
- **数据处理**：
  - JSON 序列化：`serde + serde_json`（替代 TypeScript 类型系统）。
  - 数据库：`rusqlite`（替代原 SQLite3 操作，兼容原 `LeagueAkari.db`）。
  - 算法库：`petgraph`（处理预组队分析的图结构，替代原并查集逻辑）。
- **系统交互**：
  - 进程管理：`tokio-process`（替代 Node.js 的 `child_process`，处理 WMI 脚本执行）。
  - 文件操作：`tokio-fs`（处理更新包解压、文件复制，替代原 C++ updater）。
  - IPC 通信：`tauri-ipc` 或 `tokio-ipc`（替代 Electron 的 IPC 机制）。
- **工具链**：
  - 构建工具：`cargo`（替代 `yarn`，管理依赖和编译）。
  - 打包工具：`tauri-bundler`（替代 `electron-builder`，生成跨平台可执行文件）。

#### 3. 架构设计

参考原项目的模块化拆分（主进程/渲染进程/共享逻辑），设计 Rust 架构：

```
LeagueAkari-rust/
├── src/
│   ├── main.rs          // 入口，初始化 Tauri 或核心服务
│   ├── core/            // 核心模块（对应原主进程 shards）
│   │   ├── lcu/         // LCU API 交互（状态监听、请求封装）
│   │   ├── analysis/    // 比赛数据分析（KDA、预组队计算）
│   │   ├── storage/     // 数据库操作（SQLite 封装）
│   │   ├── updater/     // 自动更新逻辑（替代 akari-updater）
│   │   └── ipc/         // 进程间通信接口
│   ├── shared/          // 共享类型与工具（对应原 src/shared）
│   │   ├── types.rs     // 通用数据结构（游戏信息、配置等）
│   │   └── utils.rs     // 工具函数（时间处理、加密等）
│   └── frontend/        // 前端资源（若复用原 Web 代码，放置 HTML/CSS/JS）
├── Cargo.toml           // 依赖配置
└── tauri.conf.json      // Tauri 配置（窗口、权限等）
```

### **二、核心模块重构阶段**

按功能模块逐步迁移，优先实现底层逻辑，再衔接上层交互。

#### 1. LCU API 交互模块（对应 `src/main/shards/league-client`）

- **目标**：替换原 TypeScript 的 LCU 通信逻辑，实现状态监听和数据请求。
- **步骤**：

  - 解析 LCU 连接信息（端口、密码）：通过读取 `lockfile`（英雄联盟客户端生成），用 `std::fs` 读取文件，`regex` 解析内容。
  - 封装 HTTP 请求：用 `reqwest` 实现带 Basic Auth 的请求，处理 LCU API 端点（如 `/lol-matchmaking/v1/ready-check`）。
  - 实时事件监听：通过 WebSocket 连接 LCU 事件总线，用 `tokio-tungstenite` 订阅事件（如匹配状态变更），并通过 IPC 同步到前端。
  - 示例代码片段：

    ```rust
    // 读取 LCU lockfile
    fn read_lockfile() -> Result<Lockfile, Box<dyn Error>> {
        let path = Path::new("C:/Riot Games/League of Legends/lockfile");
        let content = std::fs::read_to_string(path)?;
        let parts: Vec<&str> = content.split(':').collect();
        Ok(Lockfile {
            port: parts[2].parse()?,
            password: parts[3].to_string(),
        })
    }

    // 发送 LCU 请求
    async fn get_matchmaking_status(lockfile: &Lockfile) -> Result<serde_json::Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let url = format!("https://127.0.0.1:{}/lol-matchmaking/v1/search", lockfile.port);
        let response = client
            .get(&url)
            .basic_auth("riot", Some(&lockfile.password))
            .danger_accept_invalid_certs(true) // LCU 使用自签名证书
            .send()
            .await?;
        Ok(response.json().await?)
    }
    ```

#### 2. 数据处理与分析模块（对应 `src/shared/utils/analysis.ts`）

- **目标**：重构战绩分析、预组队计算等核心逻辑，利用 Rust 性能优势。
- **步骤**：

  - 数据结构定义：用 `serde` 定义游戏数据模型（如 `Game`、`PlayerStats`），对应原 TypeScript 接口。
  - KDA 与胜率计算：实现统计函数，处理比赛历史数据，生成分析结果。
  - 预组队分析：基于 `petgraph` 构建玩家关系图，用并查集算法（`disjoint-sets` 库）计算共同游戏次数，替代原 JavaScript 逻辑。
  - 示例代码片段：

    ```rust
    // 定义比赛数据结构
    #[derive(Debug, serde::Deserialize)]
    struct MatchHistory {
        games: Vec<Game>,
    }

    #[derive(Debug, serde::Deserialize)]
    struct Game {
        participants: Vec<Participant>,
    }

    // 计算 KDA
    fn calculate_kda(participant: &Participant) -> f64 {
        let (k, d, a) = (participant.kills as f64, participant.deaths as f64, participant.assists as f64);
        if d == 0.0 { (k + a) / 1.0 } else { (k + a) / d }
    }
    ```

#### 3. 数据库模块（对应原 SQLite3 逻辑）

- **目标**：迁移数据持久化逻辑，兼容原 `LeagueAkari.db` 结构。
- **步骤**：
  - 用 `rusqlite` 连接数据库，创建表结构（如 `saved_players`、`match_history`），对应原 Local Storage 数据。
  - 实现 CRUD 操作：封装查询、插入、更新方法，处理玩家信息、比赛记录等数据。
  - 数据迁移：确保从原项目的数据库文件中读取历史数据，避免用户数据丢失。

#### 4. 自动更新模块（替代 `akari-updater/main.cc`）

- **目标**：用 Rust 重写跨平台更新逻辑，替代原 C++ updater。
- **步骤**：
  - 检查更新：通过 GitHub/Gitee API 获取最新版本（用 `reqwest` 发送请求），对比本地版本。
  - 下载安装包：用 `reqwest` 断点续传，结合 `tokio::fs` 写入文件。
  - 解压与替换：用 `zip` 或 `7z` 库解压安装包，替换旧文件（参考原 C++ 的备份-复制-删除逻辑）。
  - 重启应用：用 `std::process::Command` 启动新进程，关闭旧进程。

#### 5. 进程间通信（对应原 Electron IPC）

- **目标**：实现主进程（Rust）与渲染进程（前端）的数据同步。
- **步骤**：

  - 若复用原前端：用 Tauri 的 IPC 机制，通过 `#[tauri::command]` 暴露 Rust 方法，前端通过 `window.__TAURI__.invoke` 调用。
  - 若用 Rust 前端：用 `yew` 的 `use_effect_with` 监听 IPC 事件，同步状态。
  - 示例（Tauri IPC）：

    ```rust
    // Rust 后端暴露接口
    #[tauri::command]
    async fn get_ongoing_game_info() -> Result<OngoingGameInfo, String> {
        let info = ongoing_game_service.get_info().await.map_err(|e| e.to_string())?;
        Ok(info)
    }

    // 前端调用
    const gameInfo = await window.__TAURI__.invoke('get_ongoing_game_info');
    ```

### **三、渲染层与集成阶段**

#### 1. 前端适配（复用或重构）

- **复用原前端**：将原 `src/renderer` 目录的 Vue/TypeScript 代码迁移到 Tauri 的 `frontend` 目录，修改 IPC 调用逻辑（从 Electron IPC 改为 Tauri IPC）。
- **完全重构**：用 `yew` 或 `dioxus` 重写 UI，实现响应式布局，调用 Rust 后端接口。

#### 2. 窗口管理与系统交互

- 用 Tauri 的 `window` API 管理窗口（主窗口、辅助窗口），替代原 Electron 窗口逻辑。
- 实现 WMI 修复功能：通过 `std::process::Command` 执行 `wmic` 命令，替代原 `_rebuildWmi` 方法。

### **四、测试与优化阶段**

#### 1. 单元测试

- 为核心模块编写测试（如 `cargo test`），验证 LCU API 交互、数据分析逻辑的正确性。
- 测试数据库操作，确保数据读写与原项目一致。

#### 2. 集成测试

- 端到端测试：启动应用，模拟英雄联盟客户端环境，验证功能流程（如匹配状态同步、战绩加载）。
- 兼容性测试：在不同 Windows 版本（原项目主要支持 Windows）上测试，确保 LCU 通信、更新机制正常。

#### 3. 性能优化

- 利用 Rust 的内存安全特性，减少原 JavaScript 可能的内存泄漏。
- 优化异步逻辑：用 `tokio` 调度任务，避免阻塞（如批量加载战绩时的并发控制）。
- 减小体积：通过 `tauri-bundler` 打包，移除 Electron 冗余依赖，缩小安装包体积。

### **五、发布与迁移阶段**

1. **版本迁移**：提供从原项目到 Rust 版本的平滑迁移（如数据库文件兼容、配置导入）。
2. **打包发布**：用 `cargo tauri build` 生成 Windows 安装包，替代原 `yarn build:win`。
3. **文档更新**：修改 `README.md`，更新构建步骤（如 `cargo install`、`tauri dev`）和依赖说明。

### **关键挑战与解决方案**

- **LCU API 兼容性**：LCU 接口可能随游戏版本变更，需封装灵活的请求层，便于适配变更。
- **异步逻辑转换**：原项目的 Promise 异步逻辑需用 Rust 的 `async/await` 重构，合理使用 `tokio` 任务调度。
- **前端交互适配**：若复用原前端，需批量替换 IPC 调用，确保与 Rust 后端通信无误。

通过以上流程，可逐步将 LeagueAkari 从 Electron/TypeScript 迁移到 Rust 技术栈，保留核心功能的同时，提升性能和安全性。
