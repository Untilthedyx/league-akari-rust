// // src/client/detector.rs
// use crate::client::registry;
// use crate::error::ClientError;
// use crate::utils::process::is_running;
// use std::path::{Path, PathBuf}; // 引用工具函数

// /// WeGame 状态检测
// pub fn detect_wegame() -> Result<(bool, PathBuf, bool), ClientError> {
//     let path = registry::get_wegame_install_path()?;
//     let running = is_running("WeGame.exe");
//     Ok((true, path, running))
// }

// /// 英雄联盟客户端状态检测
// pub fn detect_league(wegame_path: &Path) -> Option<PathBuf> {
//     let league_path = wegame_path
//         .parent()?
//         .join("Game")
//         .join("League of Legends")
//         .join("LeagueClient.exe");

//     if league_path.exists() {
//         Some(league_path)
//     } else {
//         None
//     }
// }
