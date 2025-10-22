pub mod config;
use crate::utils::error::log_error::LogError;
use crate::utils::log::config::LogConfig;
use std::fs::OpenOptions;
use std::path::Path;
use tracing::subscriber::set_global_default;
use tracing_subscriber::{fmt, prelude::*, registry};

pub fn init_logger(config: &LogConfig) -> Result<(), LogError> {
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
                        // .with_ansi(false), 控制台和文件都有颜色，无法去除
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
