use crate::shared::init::game_data::{
    clear_champion_info_cache, clear_item_info_cache, clear_perk_info_cache, clear_spell_info_cache,
};
use crate::shared::init::game_data::{
    init_champion_info_cache, init_item_info_cache, init_perk_info_cache, init_spell_info_cache,
};

use crate::shared::init::lcu::get_lcu_client;
use crate::shared::init::lcu::{clear_lcu_client, init_lcu_client};
use crate::shared::init::process::{clear_process_info, init_process_info};
use crate::shared::init::sgp::{clear_sgp_client, init_sgp_client};
use crate::utils::error::init_error::InitError;
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize)]
pub struct InitStatus {
    pub initialized: bool,
    pub message: String,
}

/// 发送初始化状态事件
fn emit_init_status(app_handle: &Option<AppHandle>, initialized: bool, message: &str) {
    if let Some(handle) = app_handle {
        let _ = handle.emit(
            "init-status",
            InitStatus {
                initialized,
                message: message.to_string(),
            },
        );
    }
}

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
                warn!("{} 初始化失败: {}, 1秒后重试...", operation_name, e);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}

pub async fn wait_client_ready(app_handle: &Option<AppHandle>) {
    let mut i = 1;
    emit_init_status(&app_handle, false, "等待客户端就绪...");
    loop {
        let client = get_lcu_client().await.unwrap();
        if client.summoner.get_current_summoner().await.is_ok() {
            break;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
        info!("等待客户端就绪: 已经等待了 {} 秒", i);
        emit_init_status(
            &app_handle,
            false,
            &format!("等待客户端就绪: 已经等待了 {} 秒", i),
        );
        i += 1;
    }
    emit_init_status(&app_handle, true, "客户端就绪");
}

pub async fn init_state(app_handle: Option<AppHandle>) {
    info!("正在初始化进程信息...");
    emit_init_status(&app_handle, false, "正在初始化进程信息...");
    init_process_info().await.unwrap();
    info!("正在初始化 LCU 客户端...");
    emit_init_status(&app_handle, false, "正在初始化 LCU 客户端...");
    init_lcu_client().await.unwrap();
    info!("正在初始化 SGP 客户端...");
    emit_init_status(&app_handle, false, "正在初始化 SGP 客户端...");
    init_sgp_client().await.unwrap();

    // 等待 客户端 初始化完成
    info!("等待客户端就绪...");
    wait_client_ready(&app_handle).await;
    info!("客户端就绪");

    info!("正在初始化召唤师技能图标缓存...");
    emit_init_status(&app_handle, false, "正在初始化召唤师技能图标缓存...");
    retry_with_delay(|| init_spell_info_cache(), "召唤师技能图标缓存").await;

    info!("正在初始化符文图标缓存...");
    emit_init_status(&app_handle, false, "正在初始化符文图标缓存...");
    retry_with_delay(|| init_perk_info_cache(), "符文图标缓存").await;

    info!("正在初始化英雄图标缓存...");
    emit_init_status(&app_handle, false, "正在初始化英雄图标缓存...");
    retry_with_delay(|| init_champion_info_cache(), "英雄图标缓存").await;

    info!("正在初始化物品图标缓存...");
    emit_init_status(&app_handle, false, "正在初始化物品图标缓存...");
    retry_with_delay(|| init_item_info_cache(), "物品图标缓存").await;

    // 发送初始化完成事件
    info!("初始化完成");
    emit_init_status(&app_handle, true, "初始化完成");
}

pub async fn clear_state(app_handle: Option<AppHandle>) {
    info!("正在清除状态...");
    emit_init_status(&app_handle, false, "正在清除状态...");
    
    info!("正在清除进程信息...");
    clear_process_info().await;
    info!("正在清除 LCU 客户端...");
    clear_lcu_client().await;
    info!("正在清除 SGP 客户端...");
    clear_sgp_client().await;
    info!("正在清除物品图标缓存...");
    clear_item_info_cache().await;
    info!("正在清除召唤师技能图标缓存...");
    clear_spell_info_cache().await;
    info!("正在清除符文图标缓存...");
    clear_perk_info_cache().await;
    info!("正在清除英雄图标缓存...");
    clear_champion_info_cache().await;
    
    info!("清除完成");
    emit_init_status(&app_handle, false, "状态已清除，等待重新初始化...");
}
