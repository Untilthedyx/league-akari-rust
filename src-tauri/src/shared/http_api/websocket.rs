use crate::utils::error::websocket_error::WebsocketError;
use base64::{engine::general_purpose::STANDARD, Engine};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::Message};
use tungstenite::client::IntoClientRequest;

pub struct Websocket {
    port: u32,
    token: String,
    sender: Option<mpsc::UnboundedSender<Message>>,
    on_messsage: Option<Arc<Mutex<dyn Fn(Value) + Send + Sync>>>,
}

impl Websocket {
    pub fn new<F>(port: u32, token: String, callback: F) -> Self
    where
        F: Fn(Value) + Send + Sync + 'static,
    {
        Self {
            port,
            token,
            sender: None,
            on_messsage: Some(Arc::new(Mutex::new(callback))),
        }
    }

    pub async fn connect(&mut self) -> Result<(), WebsocketError> {
        let url = format!("wss://127.0.0.1:{}/", &self.port);
        let connector = tokio_tungstenite::Connector::NativeTls(
            native_tls::TlsConnector::builder()
                .danger_accept_invalid_certs(true)
                .danger_accept_invalid_hostnames(true)
                .build()
                .unwrap(),
        );
        let mut request = url.into_client_request().unwrap();
        let encode = STANDARD.encode(format!("riot:{}", &self.token));
        request.headers_mut().insert(
            "Authorization",
            format!("Basic {}", encode).parse().unwrap(),
        );

        match connect_async_tls_with_config(request, None, false, Some(connector)).await {
            Ok((ws_stream, _)) => {
                let (mut write, mut read) = ws_stream.split();
                let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

                self.sender = Some(tx.clone());
                let on_message = self.on_messsage.clone();

                tokio::spawn(async move {
                    while let Some(msg) = rx.recv().await {
                        if write.send(msg).await.is_err() {
                            // 发送失败，连接可能关闭
                            break;
                        }
                    }
                });

                tokio::spawn(async move {
                    while let Some(msg) = read.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                    if let Some(cb) = &on_message {
                                        (cb.lock().await)(json);
                                    } else {
                                        // 收到文本消息
                                    }
                                }
                            }
                            Ok(Message::Ping(_)) => { // 收到 Ping
                            }
                            Ok(Message::Close(_)) => {
                                // 收到 Pong
                                // 连接关闭
                                break;
                            }
                            Err(e) => {
                                // 处理错误
                                break;
                            }
                            _ => {}
                        }
                    }
                });

                Ok(())
            }
            Err(e) => Err(WebsocketError::Tungstenite(e)),
        }
    }

    pub fn send(&self, txt: &str) {
        let _ = self
            .sender
            .as_ref()
            .unwrap()
            .send(Message::Text(txt.into()));
    }

    pub fn close(&self) {
        let _ = self.sender.as_ref().unwrap().send(Message::Close(None));
    }
}
