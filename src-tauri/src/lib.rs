// use serde::Serialize;
// use std::path::PathBuf;  // 添加 PathBuf 导入

// pub mod client;
// pub mod error;
pub mod utils;

// pub use client::detector::{detect_league, detect_wegame};
// pub use error::ClientError;

// // 客户端状态结构体（返回给前端）- 添加 pub 使其可见
// #[derive(Debug, Serialize)]
// pub struct ClientStatus {
//     installed: bool,
//     path: Option<PathBuf>,
//     running: bool,
// }

// // Tauri 命令：获取 WeGame 状态
// #[tauri::command]
// fn get_wegame_status() -> Result<ClientStatus, String> {
//     // 直接使用导入的函数，而不是通过 detector 模块
//     match detect_wegame() {
//         Ok((installed, path, running)) => Ok(ClientStatus {
//             installed,
//             path: Some(path),
//             running,
//         }),
//         Err(e) => {
//             // 可以考虑将错误信息打印出来用于调试
//             eprintln!("Error detecting WeGame: {}", e);
//             Ok(ClientStatus {
//                 installed: false,
//                 path: None,
//                 running: false,
//             })
//         }
//     }
// }

// #[tauri::command]
// fn get_league_status() -> Result<ClientStatus, String> {
//     // 同样修正为直接使用导入的函数
//     match detect_league() {
//         Ok((installed, path, running)) => Ok(ClientStatus {
//             installed,
//             path: Some(path),
//             running,
//         }),
//         Err(e) => {
//             eprintln!("Error detecting League: {}", e);
//             Ok(ClientStatus {
//                 installed: false,
//                 path: None,
//                 running: false,
//             })
//         }
//     }
// }

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}