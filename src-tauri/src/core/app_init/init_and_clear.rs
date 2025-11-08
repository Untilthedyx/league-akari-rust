use crate::shared::init::game_data::{
    clear_champion_info_cache, clear_item_info_cache, clear_perk_info_cache, clear_spell_info_cache,
};
use crate::shared::init::game_data::{
    init_champion_info_cache, init_item_info_cache, init_perk_info_cache, init_spell_info_cache,
};

use crate::shared::init::lcu::{clear_lcu_client, init_lcu_client};
use crate::shared::init::process::{clear_process_info, init_process_info};
use crate::shared::init::sgp::{clear_sgp_client, init_sgp_client};
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
                println!("{} 初始化成功", operation_name);
                info!("{} 初始化成功", operation_name);
                break;
            }
            Err(e) => {
                println!("{} 初始化失败: {}，1秒后重试...", operation_name, e);
                warn!("{} 初始化失败: {}，1秒后重试...", operation_name, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

pub async fn init_state() {
    init_process_info().await.unwrap();
    init_lcu_client().await.unwrap();
    init_sgp_client().await.unwrap();
    // 等待 3 秒，让 lol 客户端 初始化完成
    tokio::time::sleep(Duration::from_secs(3)).await;
    // 使用重试逻辑初始化缓存
    retry_with_delay(|| init_spell_info_cache(), "召唤师技能图标缓存").await;
    retry_with_delay(|| init_perk_info_cache(), "符文图标缓存").await;
    retry_with_delay(|| init_champion_info_cache(), "英雄图标缓存").await;
    retry_with_delay(|| init_item_info_cache(), "物品图标缓存").await;
}

pub async fn clear_state() {
    clear_process_info().await;
    clear_lcu_client().await;
    clear_sgp_client().await;
    clear_item_info_cache().await;
    clear_spell_info_cache().await;
    clear_perk_info_cache().await;
    clear_champion_info_cache().await;
}
