use anyhow::{anyhow, Result};
use base64::engine::{general_purpose, Engine as _};
use futures_util::{SinkExt, StreamExt};
use parking_lot::RwLock;
use p_queue::PQueue;
use radix_trie::Trie;
use reqwest::{Client, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::protocol::Message;
use url::Url;

// 连接状态枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

// LCU 认证信息（对应 TypeScript 的 UxCommandLine）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcuAuth {
    pub port: u16,
    pub auth_token: String,
    pub pid: u32,
    pub region: String,
    // 其他字段（如证书）按需添加
}

// LCU 事件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LcuEvent {
    pub uri: String,
    #[serde(rename = "eventType")]
    pub event_type: String,
    pub data: serde_json::Value,
}

// 事件订阅回调类型
type EventCallback = Arc<dyn Fn(LcuEvent) + Send + Sync + 'static>;

/// League Client 核心交互类
#[derive(Clone)]
pub struct LeagueClientMain {
    // 状态管理
    state: Arc<RwLock<ConnectionState>>,
    current_auth: Arc<RwLock<Option<LcuAuth>>>,
    
    // HTTP 客户端
    http_client: Arc<Mutex<Option<Client>>>,
    
    // WebSocket 相关
    ws_stream: Arc<Mutex<Option<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>>>,
    event_trie: Arc<Mutex<Trie<String, Vec<EventCallback>>>>,  // 高效路由事件
    
    // IPC 通信通道（假设使用 mpsc 模拟）
    ipc_sender: mpsc::Sender<IpcMessage>,
    
    // 请求限流
    asset_limiter: Arc<Mutex<PQueue>>,
}

// IPC 消息类型（前端与后端通信）
#[derive(Debug)]
pub enum IpcMessage {
    LcuEvent { sub_id: String, event: LcuEvent },
    ConnectionStateChanged(ConnectionState),
    Error(String),
}

impl LeagueClientMain {
    /// 创建新实例
    pub fn new(ipc_sender: mpsc::Sender<IpcMessage>) -> Self {
        Self {
            state: Arc::new(RwLock::new(ConnectionState::Disconnected)),
            current_auth: Arc::new(RwLock::new(None)),
            http_client: Arc::new(Mutex::new(None)),
            ws_stream: Arc::new(Mutex::new(None)),
            event_trie: Arc::new(Mutex::new(Trie::new())),
            ipc_sender,
            asset_limiter: Arc::new(Mutex::new(PQueue::new(8))),  // 并发限制为 8
        }
    }

    /// 获取当前连接状态
    pub fn get_state(&self) -> ConnectionState {
        self.state.read().clone()
    }

    /// 连接到 LCU
    pub async fn connect(&self, auth: LcuAuth) -> Result<()> {
        let current_state = self.get_state();
        if current_state == ConnectionState::Connected || current_state == ConnectionState::Connecting {
            return Err(anyhow!("Already connecting or connected"));
        }

        // 更新状态为连接中
        *self.state.write() = ConnectionState::Connecting;
        self.ipc_sender.send(IpcMessage::ConnectionStateChanged(ConnectionState::Connecting)).await?;

        // 1. 初始化 HTTP 客户端
        let http_client = self.create_http_client(&auth).await?;
        *self.http_client.lock().await = Some(http_client);

        // 2. 初始化 WebSocket 连接并订阅事件
        let ws_stream = self.connect_websocket(&auth).await?;
        *self.ws_stream.lock().await = Some(ws_stream);
        self.subscribe_default_endpoints().await?;

        // 3. 更新状态为已连接
        *self.current_auth.write() = Some(auth.clone());
        *self.state.write() = ConnectionState::Connected;
        self.ipc_sender.send(IpcMessage::ConnectionStateChanged(ConnectionState::Connected)).await?;

        // 4. 启动 WebSocket 消息监听任务
        self.spawn_ws_listener().await;

        Ok(())
    }

    /// 断开连接
    pub async fn disconnect(&self) -> Result<()> {
        // 关闭 WebSocket
        if let Some(mut ws) = self.ws_stream.lock().await.take() {
            ws.close(None).await?;
        }

        // 清空客户端
        *self.http_client.lock().await = None;
        *self.current_auth.write() = None;

        // 更新状态
        *self.state.write() = ConnectionState::Disconnected;
        self.ipc_sender.send(IpcMessage::ConnectionStateChanged(ConnectionState::Disconnected)).await?;

        Ok(())
    }

    /// 创建 HTTP 客户端（忽略 TLS 证书验证）
    async fn create_http_client(&self, auth: &LcuAuth) -> Result<Client> {
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("riot:{}", auth.auth_token))
        );

        let client = Client::builder()
            .danger_accept_invalid_certs(true)  // 忽略自签名证书
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    reqwest::header::HeaderValue::from_str(&auth_header)?,
                );
                headers
            })
            .timeout(std::time::Duration::from_millis(12500))
            .build()?;

        // 验证连接
        let ping_url = format!("https://127.0.0.1:{}/riotclient/auth-token", auth.port);
        let response = client.get(&ping_url).send().await?;
        if !response.status().is_success() {
            return Err(anyhow!("LCU ping failed: {}", response.status()));
        }

        Ok(client)
    }

    /// 连接 WebSocket
    async fn connect_websocket(&self, auth: &LcuAuth) -> Result<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>> {
        let ws_url = format!("wss://127.0.0.1:{}", auth.port);
        let url = Url::parse(&ws_url)?;

        // 构建认证头
        let auth_header = format!(
            "Basic {}",
            general_purpose::STANDARD.encode(format!("riot:{}", auth.auth_token))
        );

        // 连接 WebSocket（忽略证书验证）
        let (ws_stream, response) = tokio_tungstenite::connect_async(tokio_tungstenite::tungstenite::client::IntoClientRequest::into_client_request(
            url
        )?.header("Authorization", auth_header))
        .await?;

        if response.status() != StatusCode::SWITCHING_PROTOCOLS {
            return Err(anyhow!("WebSocket handshake failed: {}", response.status()));
        }

        Ok(ws_stream)
    }

    /// 订阅默认 LCU 端点（对应 SUBSCRIBED_LCU_ENDPOINTS）
    async fn subscribe_default_endpoints(&self) -> Result<()> {
        let default_endpoints = [
            "/lol-gameflow/v1/gameflow-phase",
            "/lol-champ-select/v1/session",
            "/lol-summoner/v1/current-summoner",
            // 其他需要订阅的端点...
        ];

        let mut ws = self.ws_stream.lock().await;
        let ws = ws.as_mut().ok_or(anyhow!("WebSocket not connected"))?;

        for endpoint in default_endpoints {
            // LCU 订阅格式：[5, "endpoint"]
            let subscribe_msg = serde_json::to_string(&[5, endpoint])?;
            ws.send(Message::Text(subscribe_msg)).await?;
        }

        Ok(())
    }

    /// 启动 WebSocket 消息监听任务
    async fn spawn_ws_listener(&self) {
        let self_clone = self.clone();
        tokio::spawn(async move {
            let mut ws = loop {
                if let Some(ws) = self_clone.ws_stream.lock().await.as_mut() {
                    break ws;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            };

            while let Some(msg) = ws.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = self_clone.handle_ws_message(&text).await {
                            let _ = self_clone.ipc_sender.send(IpcMessage::Error(format!("WS message error: {}", e))).await;
                        }
                    }
                    Ok(Message::Close(_)) => break,
                    Err(e) => {
                        let _ = self_clone.ipc_sender.send(IpcMessage::Error(format!("WS error: {}", e))).await;
                        break;
                    }
                    _ => {}
                }
            }

            // 连接断开时自动重连
            let _ = self_clone.reconnect().await;
        });
    }

    /// 处理 WebSocket 消息并分发事件
    async fn handle_ws_message(&self, text: &str) -> Result<()> {
        // LCU 消息格式：[0, "OnJsonApiEvent", { ... }]
        let msg: Vec<serde_json::Value> = serde_json::from_str(text)?;
        if msg.len() < 3 {
            return Ok(());
        }

        let event_data = &msg[2];
        let event: LcuEvent = serde_json::from_value(event_data.clone())?;

        // 从 Trie 中匹配订阅该 URI 的回调并执行
        let trie = self.event_trie.lock().await;
        for (_, callbacks) in trie.prefix_iter(&event.uri) {
            for callback in callbacks {
                callback(event.clone());
            }
        }

        Ok(())
    }

    /// 订阅 LCU 事件
    pub async fn subscribe_event<F>(&self, uri: &str, callback: F) -> String
    where
        F: Fn(LcuEvent) + Send + Sync + 'static,
    {
        let sub_id = format!("sub_{}", rand::random::<u64>());  // 生成唯一订阅 ID
        let callback = Arc::new(callback);

        let mut trie = self.event_trie.lock().await;
        trie.entry(uri.to_string())
            .or_insert_with(Vec::new)
            .push(callback);

        sub_id
    }

    /// 取消订阅 LCU 事件（简化版，实际需跟踪订阅 ID 与回调映射）
    pub async fn unsubscribe_event(&self, uri: &str) -> Result<()> {
        let mut trie = self.event_trie.lock().await;
        trie.remove(uri);
        Ok(())
    }

    /// 发送 HTTP 请求到 LCU
    pub async fn send_request(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        let auth = self.current_auth.read().clone().ok_or(anyhow!("Not connected to LCU"))?;
        let client = self.http_client.lock().await.as_ref().ok_or(anyhow!("HTTP client not initialized"))?;

        let url = format!("https://127.0.0.1:{}{}", auth.port, path);
        let mut request = client.request(method, &url);

        if let Some(body) = body {
            request = request.json(&body);
        }

        // 资产请求限流
        let response = if path.starts_with("/lol-game-data/assets") {
            let mut limiter = self.asset_limiter.lock().await;
            limiter.push(async { request.send().await }).await??
        } else {
            request.send().await?
        };

        Ok(response)
    }

    /// 自动重连逻辑
    async fn reconnect(&self) -> Result<()> {
        let auth = self.current_auth.read().clone().ok_or(anyhow!("No previous auth info"))?;
        self.disconnect().await?;

        // 重试连接（最多 5 次）
        for i in 0..5 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            match self.connect(auth.clone()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    let _ = self.ipc_sender.send(IpcMessage::Error(format!("Reconnect attempt {} failed: {}", i + 1, e))).await;
                }
            }
        }

        Err(anyhow!("Failed to reconnect after 5 attempts"))
    }
}