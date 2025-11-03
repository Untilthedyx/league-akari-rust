# HTTP API 目录

## 概述

`http_api` 目录包含所有与 HTTP/WebSocket 通信相关的模块，主要用于与 Riot Games 的本地客户端 API 进行交互。该目录提供了统一的 HTTP 客户端封装和多个子系统的 API 接口实现。

## 目录结构

```
http_api/
├── mod.rs                          # 模块导出文件
├── http.rs                         # HTTP 客户端封装
├── websocket.rs                    # WebSocket 客户端封装
├── readme.md                       # API 使用说明
├── league_client/                  # League Client API 实现
├── league-client_yuanlai/          # 原始 League Client API（旧版实现）
├── game-client/                    # Game Client API（预留）
└── riot-client/                    # Riot Client API（预留）
```

## 核心模块

### http.rs - HTTP 客户端封装

提供通用的 HTTP 客户端实现，用于与 Riot 本地 API 通信。

**主要特性：**
- 支持 GET、POST、PUT、PATCH、DELETE 方法
- 自动处理自签名证书（`accept_invalid_certs`）
- 统一的错误处理（`HttpError`）
- 自动日志记录请求和响应
- 支持可选请求体（通过 `Option<T>`）
- 支持可选响应体（返回类型为 `()` 时跳过解析）

**使用示例：**
```rust
use crate::shared::http_api::http::HttpClient;

// 创建客户端（端口、令牌、接受无效证书）
let client = HttpClient::new(port, token, true)?;

// GET 请求
let data: ResponseType = client.get("/api/endpoint").await?;

// POST 请求（带请求体）
let result = client.post("/api/endpoint", Some(&request_data)).await?;

// POST 请求（无响应体）
let _: () = client.post("/api/endpoint", Some(&data)).await?;
```

### websocket.rs - WebSocket 客户端封装

提供 WebSocket 客户端实现，用于实时事件监听。

**主要特性：**
- 自动处理 TLS 连接（接受无效证书和主机名）
- 使用 Basic 认证（riot:token）
- 支持连接状态管理（Disconnected、Connected、Closing）
- 事件回调机制（on_connect、on_message、on_close）
- 异步消息发送和接收

**使用示例：**
```rust
use crate::shared::http_api::websocket::WebsocketClient;

let mut ws = WebsocketClient::new(port, token);

// 设置回调
ws.on_connect(|| println!("Connected"));
ws.on_message(|msg| println!("Message: {:?}", msg));
ws.on_close(|| println!("Closed"));

// 连接
ws.connect().await?;

// 发送消息
ws.send(r#"{"event": "Subscribe", "data": []}"#);

// 关闭连接
ws.close();
```

## 子目录说明

### league_client/

完整的 League Client API 实现，包含所有游戏功能模块的 HTTP API 接口。

详见：[league_client/README.md](./league_client/README.md)

### league-client_yuanlai/

原始版本的 League Client API 实现（旧版代码，可能用于兼容或参考）。

详见：[league-client_yuanlai/README.md](./league-client_yuanlai/README.md)

### game-client/

预留目录，用于实现 Game Client API 相关功能（当前为空）。

### riot-client/

预留目录，用于实现 Riot Client API 相关功能（当前为空）。

## 设计模式

1. **统一客户端封装**: 所有 API 模块共享同一个 `HttpClient` 实例（通过 clone，性能开销可忽略）
2. **类型安全**: 每个 API 模块都有对应的类型定义（位于 `types/` 目录）
3. **错误处理**: 统一的错误类型（`HttpError`）贯穿所有 API 调用
4. **日志记录**: 使用 `tracing` 进行结构化日志记录

## 依赖关系

- `reqwest`: HTTP 客户端库
- `tokio-tungstenite`: WebSocket 客户端库
- `serde`: 序列化/反序列化
- `tracing`: 日志记录
