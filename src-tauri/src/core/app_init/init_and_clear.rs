use crate::shared::init::game_data::{
    clear_item_icons_cache, clear_perk_icons_cache, clear_spell_icons_cache,
};
use crate::shared::init::game_data::{
    init_item_icons_cache, init_perk_icons_cache, init_spell_icons_cache,
};
use crate::shared::init::http::{clear_http_client, init_http_client};
use crate::shared::init::process::{clear_process_info, init_process_info};
use crate::utils::error::init_error::InitError;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

/// 重试辅助函数：每秒重试一次，直到成功
async fn retry_with_delay<F, Fut>(operation: F, operation_name: &str)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<(), InitError>>,
{
    loop {
        match operation().await {
            Ok(()) => {
                info!("{} 初始化成功", operation_name);
                break;
            }
            Err(e) => {
                warn!("{} 初始化失败: {}，1秒后重试...", operation_name, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

pub async fn init_state() {
    init_process_info().await.unwrap();
    init_http_client().await.unwrap();
    
    // 使用重试逻辑初始化缓存
    retry_with_delay(|| init_item_icons_cache(), "物品图标缓存").await;
    retry_with_delay(|| init_spell_icons_cache(), "召唤师技能图标缓存").await;
    retry_with_delay(|| init_perk_icons_cache(), "符文图标缓存").await;
}

pub async fn clear_state() {
    clear_process_info().await;
    clear_http_client().await;
    clear_item_icons_cache().await;
    clear_spell_icons_cache().await;
    clear_perk_icons_cache().await;
}
