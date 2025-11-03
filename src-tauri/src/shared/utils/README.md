# Utils 目录

## 概述

`utils` 目录包含应用程序中使用的各种工具模块，提供配置管理、状态管理、进程控制、监控等通用功能。

## 目录结构

```
utils/
├── mod.rs          # 模块导出文件
├── config.rs       # 配置管理模块
├── state.rs        # 应用状态管理
├── monitor.rs      # 游戏状态监控
├── process.rs      # 进程控制工具
├── tests.rs        # 测试工具（仅在测试模式下编译）
└── README.md       # 本文档
```

## 功能模块

### config.rs - 配置管理

提供统一的配置管理系统，支持配置的读取、写入和变更监听。

**主要特性：**
- 基于文件的配置存储（`config.json`）
- 使用 `moka` 缓存的异步缓存系统
- 支持配置变更回调注册
- 自动类型推断和默认值
- 支持多种值类型（String、Integer、Boolean、List、Map）

**核心类型：**
```rust
pub enum Value {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}
```

**主要功能：**
- `init_config()`: 初始化配置，从文件加载到缓存
- `get_config(key)`: 获取配置值，支持默认值推断
- `put_config(key, value)`: 设置配置值并持久化
- `register_on_change_callback()`: 注册配置变更回调
- `extract_bool()`: 从 Value 中提取布尔值

**使用示例：**
```rust
use crate::shared::utils::config;

// 初始化配置
config::init_config().await?;

// 获取配置
let value = config::get_config("someKey").await?;

// 设置配置
config::put_config("someKey".to_string(), Value::Boolean(true)).await?;

// 注册变更监听
config::register_on_change_callback(|key, value| {
    println!("Config {} changed to {:?}", key, value);
});
```

**设计说明：**
- 使用 `OnceCell` 确保缓存只初始化一次
- 配置变更时自动触发所有注册的回调
- 自动根据键名推断默认值类型（如以 "Switch" 结尾的默认为 false）

### state.rs - 应用状态管理

管理应用程序的共享状态，用于 Tauri 应用。

**主要类型：**
```rust
pub struct AppState {
    pub http_port: OnceLock<u16>,
}
```

**功能：**
- 使用 `OnceLock` 实现线程安全的单次初始化
- 存储 HTTP 服务端口等应用级状态
- 通过 `Default` trait 提供默认值

**使用场景：**
- 在 Tauri 命令之间共享状态
- 存储全局配置值
- 应用级别的状态管理

### monitor.rs - 游戏状态监控

监控游戏客户端的状态变化，并通过 Tauri 事件系统通知前端。

**主要功能：**
- 定期检查游戏连接状态
- 监控游戏阶段变化
- 跟踪召唤师信息变化
- 通过 Tauri 事件系统发送状态更新

**核心类型：**
```rust
pub struct GameStateEvent {
    pub connected: bool,
    pub phase: Option<String>,
    pub summoner: Option<Summoner>,
}
```

**主要函数：**
- `start_game_state_monitor(app_handle)`: 启动游戏状态监控

**工作机制：**
1. 每 2 秒检查一次游戏状态
2. 对比当前状态与上次状态
3. 如果状态变化或超过 5 秒，发送 Tauri 事件
4. 前端通过监听 `game-state-changed` 事件获取更新

**使用示例：**
```rust
use crate::shared::utils::monitor;

// 启动监控
monitor::start_game_state_monitor(app_handle).await;

// 前端监听事件
app.listen("game-state-changed", (event) => {
    console.log("Game state:", event.payload);
});
```

### process.rs - 进程控制工具

提供进程相关的工具函数（当前可能为空或包含基础功能）。

**预期功能：**
- 进程查找和监控
- 进程信息获取
- 进程控制操作

### tests.rs - 测试工具

包含测试辅助函数和测试用例（仅在 `#[cfg(test)]` 模式下编译）。

## 设计模式

### 1. 单例模式

多个模块使用 `OnceCell` 或 `LazyLock` 实现单例模式：
- `config.rs`: 使用 `OnceCell<Cache>` 管理配置缓存
- `state.rs`: 使用 `OnceLock` 管理应用状态
- `monitor.rs`: 使用 `OnceCell` 管理监控器实例

### 2. 回调机制

`config.rs` 实现了配置变更的回调机制，允许注册监听器在配置变化时执行操作。

### 3. 异步编程

所有 I/O 操作都是异步的，使用 `async/await` 语法，与 Tokio 运行时集成。

## 依赖关系

- **moka**: 高性能缓存库
- **serde_yaml**: YAML 序列化（用于配置文件）
- **tokio**: 异步运行时
- **tauri**: 桌面应用框架（用于事件系统）

## 使用建议

1. **配置管理**: 优先使用 `config.rs` 进行配置存储，而不是硬编码
2. **状态共享**: 使用 `state.rs` 在 Tauri 命令间共享状态
3. **状态监控**: 使用 `monitor.rs` 监听游戏状态变化，避免频繁轮询
4. **错误处理**: 所有异步函数都返回 `Result`，需要适当处理错误

## 相关文档

- [Tauri 官方文档](https://tauri.app/)
- [Moka 缓存文档](https://github.com/moka-rs/moka)

