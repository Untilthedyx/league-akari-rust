use crate::shared::types::league_client::entitlements::EntitlementsToken;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct EntitlementsState {
    pub token: Option<EntitlementsToken>,
}

#[derive(Debug, Default)]
pub struct EntitlementsStateLock {
    pub state: RwLock<EntitlementsState>,
}

impl EntitlementsStateLock {
    /// 初始化授权状态
    pub fn new() -> Self {
        Self::default()
    }

    // ========== token 字段的 getter/setter ==========

    /// 获取当前令牌
    pub async fn get_token(&self) -> Option<EntitlementsToken> {
        let state = self.state.read().await;
        state.token.clone()
    }

    /// 设置令牌
    pub async fn set_token(&self, token: &EntitlementsToken) {
        let mut state = self.state.write().await;
        state.token = Some(token.clone());
    }

    /// 设置令牌（直接使用 Option）
    pub async fn set_token_option(&self, token: Option<EntitlementsToken>) {
        let mut state = self.state.write().await;
        state.token = token;
    }

    /// 更新令牌的部分信息
    pub async fn update_token<F>(&self, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut EntitlementsToken),
    {
        let mut state = self.state.write().await;
        if let Some(ref mut token) = state.token {
            updater(token);
            Ok(())
        } else {
            Err("No token found".to_string())
        }
    }

    // ========== 状态检查方法 ==========

    /// 检查是否有有效的令牌
    pub async fn has_token(&self) -> bool {
        let state = self.state.read().await;
        state.token.is_some()
    }

    /// 检查令牌是否有效（根据过期时间等）
    pub async fn is_token_valid(&self) -> bool {
        let state = self.state.read().await;
        // 这里可以根据 EntitlementsToken 的实际字段添加更复杂的验证逻辑
        // 例如检查过期时间等
        state.token.is_some()
    }

    /// 获取令牌字符串（如果存在）
    pub async fn get_token_string(&self) -> Option<String> {
        let state = self.state.read().await;
        // 假设 EntitlementsToken 有一个 token 字段
        // 根据你的实际结构调整
        state.token.as_ref().map(|t| t.access_token.clone()) // 调整字段名
    }

    // ========== 批量操作 ==========

    /// 获取完整状态
    pub async fn get_full_state(&self) -> EntitlementsState {
        let state = self.state.read().await;
        state.clone()
    }

    /// 设置完整状态
    pub async fn set_full_state(&self, new_state: EntitlementsState) {
        let mut state = self.state.write().await;
        *state = new_state;
    }

    /// 清空状态（用于登出或重置）
    pub async fn clear(&self) {
        let mut state = self.state.write().await;
        *state = EntitlementsState::default();
    }

    /// 重置令牌
    pub async fn reset_token(&self) {
        let mut state = self.state.write().await;
        state.token = None;
    }
}

// 为 Tauri 命令准备的便捷方法
impl EntitlementsStateLock {
    /// 用于 Tauri 前端的便捷方法：获取可序列化的状态
    pub async fn get_serializable_state(&self) -> EntitlementsState {
        self.get_full_state().await
    }

    /// 用于 Tauri 前端的便捷方法：设置令牌
    pub async fn set_token_from_value(&self, token: EntitlementsToken) -> Result<(), String> {
        self.set_token(&token).await;
        Ok(())
    }
}

// 保留原有的 EntitlementsState 实现，但移除需要 &mut self 的方法
impl EntitlementsState {
    pub fn new() -> Self {
        Self::default()
    }

    // 注意：移除了原来的 set_token 方法，因为需要 &mut self
    // 现在这些操作都在 EntitlementsStateLock 中实现
}