pub mod client;
pub mod error;
pub mod utils;
pub mod lcu;

use client::commandinfo::CommandInfo;
use client::detect::get_client_info;
use client::detect::is_running;

#[tauri::command]
fn _is_running() -> bool {
    is_running()
}

#[tauri::command]
fn _get_client_info() -> Result<CommandInfo, String> {
    get_client_info()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![_is_running, _get_client_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
