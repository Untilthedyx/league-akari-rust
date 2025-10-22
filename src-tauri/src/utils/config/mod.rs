use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub theme: String,
    pub window_size: (u32, u32),
    pub auto_start: bool,
}

pub fn read_user_config(path: &PathBuf) -> UserConfig {
    if path.exists() {
        let content = fs::read_to_string(path).expect("读取配置文件失败");
        serde_json::from_str(&content).expect("解析配置文件失败")
    } else {
        let default = UserConfig::default();
        write_user_config(path, &default).expect("创建默认配置文件失败");
        default
    }
}

pub fn write_user_config(path: &PathBuf, config: &UserConfig) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}
