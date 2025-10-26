use futures_channel::mpsc::SendError;
/// 日志配置错误
use thiserror::Error;
use tungstenite::error::Error as TungsteniteError;

#[derive(Debug, Error)]
pub enum WebsocketError {
    #[error("Failed to connect to websocket server: {0}")]
    Tungstenite(TungsteniteError),
    #[error("Failed to parse websocket request: {0}")]
    Tls(native_tls::Error),
    #[error("Failed to send message: {0}")]
    Io(std::io::Error),
    #[error("Failed to send message: {0}")]
    Send(SendError),
    #[error("Channel closed")]
    ChannelClosed,
}
