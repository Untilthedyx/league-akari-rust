// use crate::shared::constants::common::RIOT_CA_CERTIFICATE;
use crate::utils::error::websocket_error::WebsocketError;
use base64::{engine::general_purpose::STANDARD, Engine};
use futures::{SinkExt, StreamExt};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::Message};
use tungstenite::client::IntoClientRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WsStatus {
    Disconnected,
    Connected,
    Closing,
}

pub struct WebsocketClient {
    port: u32,
    token: String,
    sender: Option<mpsc::UnboundedSender<Message>>,

    status: Arc<Mutex<WsStatus>>,
    on_connect: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
    on_message: Option<Arc<Mutex<dyn Fn(Value) + Send + Sync>>>,
    on_close: Option<Arc<Mutex<dyn Fn() + Send + Sync>>>,
}

impl WebsocketClient {
    pub fn new(port: u32, token: String) -> Self {
        Self {
            port,
            token,
            sender: None,

            status: Arc::new(Mutex::new(WsStatus::Disconnected)),
            on_connect: None,
            on_message: None,
            on_close: None,
        }
    }

    pub fn on_connect<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_connect = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn on_close<F>(&mut self, callback: F)
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_close = Some(Arc::new(Mutex::new(callback)));
    }

    pub fn on_message<F>(&mut self, callback: F)
    where
        F: Fn(Value) + Send + Sync + 'static,
    {
        self.on_message = Some(Arc::new(Mutex::new(callback)));
    }

    async fn set_status(&self, new: WsStatus) {
        let mut st = self.status.lock().await;
        *st = new;
    }

    pub async fn connect(&mut self) -> Result<(), WebsocketError> {
        let url = format!("wss://127.0.0.1:{}/", &self.port);

        // 在这里可以加入证书验证
        // let cert = native_tls::Certificate::from_pem(RIOT_CA_CERTIFICATE.as_bytes())
        //     .map_err(|e| WebsocketError::CertificateParse(e))?;
        // .add_root_certificate(cert)

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
                // connect
                self.set_status(WsStatus::Connected).await;
                if let Some(cb) = &self.on_connect {
                    (cb.lock().await)();
                }

                println!("///asd ");

                let (mut write, mut read) = ws_stream.split();
                let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

                self.sender = Some(tx.clone());
                let on_message = self.on_message.clone();
                let on_close = self.on_close.clone();
                let status = self.status.clone();

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
                    let mut st = status.lock().await;
                    *st = WsStatus::Closing;

                    if let Some(cb) = on_close {
                        (cb.lock().await)();
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
