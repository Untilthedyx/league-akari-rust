pub mod auto;
pub mod command;
pub mod utils;
pub mod shared;

use crate::command::config::put_config;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![put_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
