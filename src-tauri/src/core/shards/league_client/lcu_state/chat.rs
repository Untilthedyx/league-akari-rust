use crate::shared::types::league_client::chat::{ChatPerson, Conversation};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// 会话集合（按场景分类）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Conversations {
    /// 英雄选择阶段会话
    pub champion_select: Option<Conversation>,
    /// 赛后会话
    pub post_game: Option<Conversation>,
    /// 自定义游戏会话
    pub custom_game: Option<Conversation>,
}

/// 参与者集合（按场景分类，存储召唤师 ID 列表）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Participants {
    /// 英雄选择阶段参与者
    pub champion_select: Option<Vec<u64>>,
    /// 赛后参与者
    pub post_game: Option<Vec<u64>>,
    /// 自定义游戏参与者
    pub custom_game: Option<Vec<u64>>,
}

/// 聊天状态核心结构体（对应原 ChatState 类）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatState {
    /// 各场景会话
    pub conversations: Conversations,
    /// 各场景参与者
    pub participants: Participants,
    /// 当前登录用户信息
    pub me: Option<ChatPerson>,
}

#[derive(Debug, Default)]
pub struct ChatStateLock {
    pub state: RwLock<ChatState>,
}

impl ChatStateLock {
    /// 初始化聊天状态（对应原构造函数）
    pub fn new() -> Self {
        Self::default() // 利用 Default 实现，所有字段初始化为 None/默认值
    }

    // ========== me 字段的 getter/setter ==========

    /// 获取当前用户信息
    pub async fn get_me(&self) -> Option<ChatPerson> {
        let state = self.state.read().await;
        state.me.clone()
    }

    /// 设置当前用户信息
    pub async fn set_me(&self, me: Option<ChatPerson>) {
        let mut state = self.state.write().await;
        state.me = me;
    }

    /// 更新当前用户的部分信息
    pub async fn update_me<F>(&self, updater: F) -> Result<(), String>
    where
        F: FnOnce(&mut ChatPerson),
    {
        let mut state = self.state.write().await;
        if let Some(ref mut me) = state.me {
            updater(me);
            Ok(())
        } else {
            Err("No user data found".to_string())
        }
    }

    // ========== conversations 字段的 getter/setter ==========

    /// 获取所有会话
    pub async fn get_conversations(&self) -> Conversations {
        let state = self.state.read().await;
        state.conversations.clone()
    }

    /// 获取英雄选择阶段会话
    pub async fn get_conversation_champ_select(&self) -> Option<Conversation> {
        let state = self.state.read().await;
        state.conversations.champion_select.clone()
    }

    /// 获取赛后会话
    pub async fn get_conversation_post_game(&self) -> Option<Conversation> {
        let state = self.state.read().await;
        state.conversations.post_game.clone()
    }

    /// 获取自定义游戏会话
    pub async fn get_conversation_custom_game(&self) -> Option<Conversation> {
        let state = self.state.read().await;
        state.conversations.custom_game.clone()
    }

    /// 设置英雄选择阶段会话
    pub async fn set_conversation_champ_select(&self, conversation: Option<Conversation>) {
        let mut state = self.state.write().await;
        state.conversations.champion_select = conversation;
    }

    /// 设置赛后会话
    pub async fn set_conversation_post_game(&self, conversation: Option<Conversation>) {
        let mut state = self.state.write().await;
        state.conversations.post_game = conversation;
    }

    /// 设置自定义游戏会话
    pub async fn set_conversation_custom_game(&self, conversation: Option<Conversation>) {
        let mut state = self.state.write().await;
        state.conversations.custom_game = conversation;
    }

    // ========== participants 字段的 getter/setter ==========

    /// 获取所有参与者
    pub async fn get_participants(&self) -> Participants {
        let state = self.state.read().await;
        state.participants.clone()
    }

    /// 获取英雄选择阶段参与者
    pub async fn get_participants_champ_select(&self) -> Option<Vec<u64>> {
        let state = self.state.read().await;
        state.participants.champion_select.clone()
    }

    /// 获取赛后参与者
    pub async fn get_participants_post_game(&self) -> Option<Vec<u64>> {
        let state = self.state.read().await;
        state.participants.post_game.clone()
    }

    /// 获取自定义游戏参与者
    pub async fn get_participants_custom_game(&self) -> Option<Vec<u64>> {
        let state = self.state.read().await;
        state.participants.custom_game.clone()
    }

    /// 设置英雄选择阶段参与者
    pub async fn set_participants_champ_select(&self, participants: Option<Vec<u64>>) {
        let mut state = self.state.write().await;
        state.participants.champion_select = participants;
    }

    /// 设置赛后参与者
    pub async fn set_participants_post_game(&self, participants: Option<Vec<u64>>) {
        let mut state = self.state.write().await;
        state.participants.post_game = participants;
    }

    /// 设置自定义游戏参与者
    pub async fn set_participants_custom_game(&self, participants: Option<Vec<u64>>) {
        let mut state = self.state.write().await;
        state.participants.custom_game = participants;
    }

    /// 添加参与者到英雄选择阶段
    pub async fn add_participant_champ_select(&self, participant_id: u64) -> Result<(), String> {
        let mut state = self.state.write().await;
        match &mut state.participants.champion_select {
            Some(participants) => {
                if !participants.contains(&participant_id) {
                    participants.push(participant_id);
                }
            }
            None => {
                state.participants.champion_select = Some(vec![participant_id]);
            }
        }
        Ok(())
    }

    /// 从英雄选择阶段移除参与者
    pub async fn remove_participant_champ_select(&self, participant_id: u64) -> Result<(), String> {
        let mut state = self.state.write().await;
        if let Some(participants) = &mut state.participants.champion_select {
            participants.retain(|&id| id != participant_id);
            // 如果参与者列表为空，设置为 None
            if participants.is_empty() {
                state.participants.champion_select = None;
            }
        }
        Ok(())
    }

    // ========== 批量操作 ==========

    /// 批量更新多个会话
    pub async fn update_conversations<F>(&self, updater: F)
    where
        F: FnOnce(&mut Conversations),
    {
        let mut state = self.state.write().await;
        updater(&mut state.conversations);
    }

    /// 批量更新多个参与者
    pub async fn update_participants<F>(&self, updater: F)
    where
        F: FnOnce(&mut Participants),
    {
        let mut state = self.state.write().await;
        updater(&mut state.participants);
    }

    /// 获取完整状态（用于调试或序列化）
    pub async fn get_full_state(&self) -> ChatState {
        let state = self.state.read().await;
        state.clone()
    }

    /// 辅助方法：清空所有状态（可选，用于登出或重置）
    pub async fn clear(&self) {
        let mut state = self.state.write().await;
        *state = ChatState::default();
    }

    /// 检查是否有活跃的会话
    pub async fn has_active_conversations(&self) -> bool {
        let state = self.state.read().await;
        state.conversations.champion_select.is_some()
            || state.conversations.post_game.is_some()
            || state.conversations.custom_game.is_some()
    }

    /// 获取所有参与者的总数
    pub async fn get_total_participants_count(&self) -> usize {
        let state = self.state.read().await;
        let mut count = 0;
        
        if let Some(participants) = &state.participants.champion_select {
            count += participants.len();
        }
        if let Some(participants) = &state.participants.post_game {
            count += participants.len();
        }
        if let Some(participants) = &state.participants.custom_game {
            count += participants.len();
        }
        
        count
    }
}

// 为 Tauri 命令准备的便捷方法
impl ChatStateLock {
    /// 用于 Tauri 前端的便捷方法：获取可序列化的状态
    pub async fn get_serializable_state(&self) -> ChatState {
        self.get_full_state().await
    }

    /// 用于 Tauri 前端的便捷方法：更新用户状态
    pub async fn update_me_state(&self, me: ChatPerson) -> Result<(), String> {
        self.set_me(Some(me)).await;
        Ok(())
    }
}