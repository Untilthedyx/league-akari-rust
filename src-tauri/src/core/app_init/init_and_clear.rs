use crate::shared::init::game_data::{
    clear_item_icons_cache, clear_perk_icons_cache, clear_spell_icons_cache,
};
use crate::shared::init::game_data::{
    init_item_icons_cache, init_perk_icons_cache, init_spell_icons_cache,
};
use crate::shared::init::http::{clear_http_client, init_http_client};
use crate::shared::init::process::{clear_process_info, init_process_info};

pub async fn init_state() {
    init_process_info().await.unwrap();
    init_http_client().await.unwrap();
    init_item_icons_cache().await.unwrap();
    init_spell_icons_cache().await.unwrap();
    init_perk_icons_cache().await.unwrap();
}

pub async fn clear_state() {
    clear_process_info().await;
    clear_http_client().await;
    clear_item_icons_cache().await;
    clear_spell_icons_cache().await;
    clear_perk_icons_cache().await;
}
