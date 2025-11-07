# Shared 目录

## 概述

`shared` 目录包含应用程序的共享代码模块，这些模块在整个应用中被多个组件使用。该目录提供了与 League of Legends 客户端交互的核心功能，包括 HTTP API 封装、类型定义、工具函数等。

## 目录结构

```
shared/
├── mod.rs                    # 模块导出文件
├── constants/                # 常量定义
│   ├── mod.rs
│   ├── common.rs
│   └── README.md
├── http_api/                 # HTTP/WebSocket API 封装
│   ├── mod.rs
│   ├── http.rs               # HTTP 客户端封装
│   ├── websocket.rs          # WebSocket 客户端封装
│   ├── league_client/        # League Client API 实现
│   ├── league-client_yuanlai/ # 原始版本 API（旧版）
│   ├── game-client/          # Game Client API（预留）
│   ├── riot-client/          # Riot Client API（预留）
│   └── README.md
├── types/                     # 类型定义
│   ├── mod.rs
│   ├── league_client/        # League Client 类型
│   ├── task_runner/          # 任务运行器类型
│   └── README.md
├── utils/                     # 工具模块
│   ├── mod.rs
│   ├── config.rs             # 配置管理
│   ├── state.rs              # 应用状态管理
│   ├── monitor.rs            # 游戏状态监控
│   ├── process.rs            # 进程控制
│   ├── tests.rs              # 测试工具
│   └── README.md
└── README.md                  # 本文档
```

## 模块说明

### constants/

包含应用程序中使用的全局常量，如 URL、标识符等。

**详细文档**: [constants/README.md](./constants/README.md)

### http_api/

提供与 Riot Games 客户端 API 交互的核心功能：
- HTTP 客户端封装（支持自签名证书）
- WebSocket 客户端封装（实时事件监听）
- 完整的 League Client API 实现
- 旧版 API 实现（兼容性）

**详细文档**: [http_api/README.md](./http_api/README.md)

### types/

定义所有 API 交互使用的数据类型：
- League Client API 的所有请求/响应类型
- 任务运行器相关类型
- 使用 serde 进行序列化/反序列化

**详细文档**: [types/README.md](./types/README.md)

### utils/

提供各种工具功能：
- 配置管理（基于文件的键值存储，支持变更监听）
- 应用状态管理（Tauri 共享状态）
- 游戏状态监控（定期检查并发送事件）
- 进程控制工具

**详细文档**: [utils/README.md](./utils/README.md)

## 架构设计

### 分层结构

```
┌─────────────────────────────────────┐
│        Application Layer            │  ← 应用层
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│         http_api/                   │  ← API 封装层
│  (HTTP/WebSocket 客户端)            │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│         types/                      │  ← 类型定义层
│  (数据结构定义)                      │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│    constants/ + utils/              │  ← 基础工具层
│  (常量、配置、状态管理)               │
└─────────────────────────────────────┘
```

### 设计原则

1. **关注点分离**: 每个目录专注于特定功能领域
2. **类型安全**: 强类型定义，避免使用通用类型
3. **统一错误处理**: 使用统一的错误类型（`HttpError`）
4. **异步优先**: 所有 I/O 操作都是异步的
5. **可扩展性**: 预留了扩展空间（如 game-client、riot-client）

## 依赖关系

### 内部依赖

- `http_api` → `types`: API 实现使用类型定义
- `http_api` → `constants`: 使用常量定义
- `utils` → `http_api`: 工具模块可能调用 API
- `utils` → `types`: 使用类型定义

### 外部依赖

- **reqwest**: HTTP 客户端
- **tokio-tungstenite**: WebSocket 客户端
- **serde**: 序列化/反序列化
- **moka**: 缓存库
- **tauri**: 桌面应用框架
- **tracing**: 日志记录

## 使用示例

### 基本 API 调用

```rust
use crate::shared::http_api::league_client::LcuApi;

// 初始化 API
let api = LcuApi::new(port, token, true);

// 获取召唤师信息
let summoner = api.summoner.get_current_summoner().await?;

// 获取英雄选择会话
let session = api.champ_select.get_session().await?;
```

### 配置管理

```rust
use crate::shared::utils::config;

// 初始化配置
config::init_config().await?;

// 获取配置
let value = config::get_config("someKey").await?;

// 设置配置
config::put_config("key".to_string(), Value::Boolean(true)).await?;
```

### 游戏状态监控

```rust
use crate::shared::utils::monitor;

// 启动监控
monitor::start_game_state_monitor(app_handle).await;
```

## 开发指南

### 添加新的 API 模块

1. 在 `http_api/league_client/` 创建新的 API 文件
2. 在 `types/league_client/` 创建对应的类型文件
3. 在 `http_api/league_client/mod.rs` 中注册新模块
4. 在 `LcuApi` 中添加字段

### 添加新的工具功能

1. 在 `utils/` 目录创建新文件
2. 在 `utils/mod.rs` 中导出模块
3. 遵循现有的异步编程模式

### 类型定义规范

- 使用 `serde` 进行序列化
- 使用 `#[serde(rename_all = "camelCase")]` 处理命名
- 实现 `Default` trait
- 使用 `Option<T>` 表示可选字段

## 文档索引

- [常量定义文档](./constants/README.md)
- [HTTP API 文档](./http_api/README.md)
  - [League Client API](./http_api/league_client/README.md)
  - [原始版本 API](./http_api/league-client_yuanlai/README.md)
- [类型定义文档](./types/README.md)
  - [League Client 类型](./types/league_client/README.md)
  - [任务运行器类型](./types/task_runner/README.md)
- [工具模块文档](./utils/README.md)

## 维护说明

- 所有 API 类型应与 Riot Games 官方 API 文档保持同步
- 添加新功能时，请同时更新相关文档
- 保持代码风格一致，遵循 Rust 最佳实践
- 所有异步函数都应返回 `Result` 类型以便错误处理

