/// 日志配置错误
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {
    #[error("初始化对象错误: {0}")]
    Init(String),
    #[error("获取对象错误: {0}")]
    Get(String),
    #[error("清除对象错误: {0}")]
    Clear(String),
    #[error("其他错误: {0}, {1}")]
    Other(String, String),
}
