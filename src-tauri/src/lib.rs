pub mod command;
pub mod core;
pub mod shared;
pub mod utils;

use crate::core::app_init::app_state::app_setup;
use crate::utils::log::init_logger;

/// Command handlers
use crate::command::asset::{
    get_champion_icon, get_item_icon, get_perk_icon, get_profile_icon, get_spell_icon,
};
use crate::command::history::get_rank_list;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logger();
    tauri::Builder::default()
        .setup(|app| app_setup(app))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_rank_list,
            get_profile_icon,
            get_champion_icon,
            get_item_icon,
            get_spell_icon,
            get_perk_icon,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
