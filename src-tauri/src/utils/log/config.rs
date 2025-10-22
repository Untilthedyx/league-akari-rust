/// 日志配置（级别，输出格式，目标等）
use serde::Deserialize;
use tracing_subscriber::filter::LevelFilter;

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    pub level: String,             // 日志级别 trace debug info warn error
    pub output: String,            // 输出目标 console file both
    pub file_path: Option<String>, // 若输出目标为 file 指定文件路径
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
