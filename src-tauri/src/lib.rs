pub mod client;
pub mod error;
pub mod utils;

use client::detect::is_running;

#[tauri::command]
fn greet(name: &str) -> String {
    let flag = is_running();
    format!("Hello, {}! You've been greeted from Rust!, {}", name, flag)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
