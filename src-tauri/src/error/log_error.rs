/// 日志配置错误
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Log init error: {0}")]
    LogInitError(String),

    #[error("Invalid log config: {0}")]
    InvalidLogConfig(String),

    #[error("Log file error: {0}")]
    LogFileError(String),
}