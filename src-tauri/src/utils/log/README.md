# 日志

## 自定义日志

同时这里好像有 bug，file 老是写入 ansi 字符进去造成乱码

文件如下，首先是 log_error.rs 文件：

```rust
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
```


下面是 config.rs 文件

```rust
/// 日志配置（级别，输出格式，目标等）
use serde::Deserialize;
use std::default::Default;
use tracing_subscriber::filter::LevelFilter;

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,             // 日志级别 trace debug info warn error
    pub output: String,            // 输出目标 console file both
    pub file_path: Option<String>, // 若输出目标为 file 指定文件路径
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            output: "console".to_string(),
            file_path: None,
        }
    }
}

impl LogConfig {
    /// 将字符串级别转化为 tracing 的 LevelFilter
    pub fn to_level_filter(&self) -> LevelFilter {
        match self.level.as_str() {
            "trace" => LevelFilter::TRACE,
            "debug" => LevelFilter::DEBUG,
            "info" => LevelFilter::INFO,
            "warn" => LevelFilter::WARN,
            "error" => LevelFilter::ERROR,
            _ => LevelFilter::INFO,
        }
    }
}
```

这里是 mod.rs 文件

```rust
pub mod config;
use crate::utils::error::log_error::LogError;
use crate::utils::log::config::LogConfig;
use std::fs::OpenOptions;
use std::path::Path;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt, prelude::*, registry};

pub fn init_logger(config: Option<LogConfig>) -> Result<(), LogError> {
    let config = config.unwrap_or_default();
    let filter = config.to_level_filter();

    match config.output.as_str() {
        "console" => {
            let subscriber = registry().with(filter).with(
                fmt::layer()
                    .with_target(true)
                    .with_timer(fmt::time::LocalTime::rfc_3339())
                    .with_writer(std::io::stdout)
                    .with_ansi(true),
            );
            set_global_default(subscriber).map_err(|e| LogError::LogInitError(e.to_string()))?;
        }
        "file" => {
            let file_path = config
                .file_path
                .as_ref()
                .ok_or(LogError::InvalidLogConfig("need file path".to_string()))?;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(Path::new(file_path))
                .map_err(|e| LogError::LogFileError(e.to_string()))?;

            let subscriber = registry().with(filter).with(
                fmt::layer()
                    .with_writer(file)
                    .with_target(true)
                    .with_timer(fmt::time::LocalTime::rfc_3339())
                    .with_ansi(false),
            );
            set_global_default(subscriber).map_err(|e| LogError::LogInitError(e.to_string()))?;
        }
        "both" => {
            let file_path = config
                .file_path
                .as_ref()
                .ok_or(LogError::InvalidLogConfig("need file path".to_string()))?;
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(Path::new(file_path))
                .map_err(|e| LogError::LogFileError(e.to_string()))?;

            let subscriber = registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_target(true)
                        .with_timer(fmt::time::LocalTime::rfc_3339())
                        .with_writer(std::io::stdout)
                        .with_ansi(true), // 控制台有颜色
                )
                .with(
                    fmt::layer()
                        .with_writer(file)
                        .with_target(true)
                        .with_timer(fmt::time::LocalTime::rfc_3339())
                        // .with_ansi(false), // 控制台和文件都有颜色，无法去除
                        .json(), // 文件没有颜色
                );
            set_global_default(subscriber).map_err(|e| LogError::LogInitError(e.to_string()))?;
        }
        _ => {
            return Err(LogError::InvalidLogConfig(
                "must one of them: console, file, both".to_string(),
            ))
        }
    };

    Ok(())
}
```

## tauri_plugin_log

不推荐，不支持 tracing