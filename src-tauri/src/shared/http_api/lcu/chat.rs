//! 聊天相关 HTTP API 封装模块
//! 提供与游戏内聊天系统交互的各种接口，包括好友管理、会话操作、消息发送等功能

use crate::shared::{
    http_api::lcu::http::HttpClient, // 通用 HTTP 客户端
    types::league_client::chat::*,   // 聊天相关数据结构（Friend、Conversation 等）
};
use crate::utils::error::http_error::HttpError; // HTTP 错误类型
use serde::Serialize; // 用于 JSON 序列化

/// 聊天系统 HTTP API 封装
/// 封装了与游戏聊天服务交互的所有接口，基于通用 HttpClient 实现
#[derive(Clone)]
pub struct ChatHttpApi {
    /// 内部使用的 HTTP 客户端实例
    client: HttpClient,
}

impl ChatHttpApi {
    /// 创建 ChatHttpApi 实例
    /// - 参数: `client` - 预配置的 HttpClient 实例（需包含认证信息等）
    /// - 返回: 新的 ChatHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取好友列表
    /// - 返回: 好友列表（Vec<Friend>）或 HTTP 错误
    pub async fn get_friends(&self) -> Result<Vec<Friend>, HttpError> {
        let url = "/lol-chat/v1/friends";
        self.client.get(url).await
    }

    /// 删除指定好友
    /// - 参数: `id` - 好友唯一标识符（通常为字符串形式的 ID）
    /// - 返回: 成功时返回空，失败时返回 HTTP 错误
    pub async fn delete_friend(&self, id: &String) -> Result<(), HttpError> {
        let url = format!("/lol-chat/v1/friends/{}", id);
        self.client.delete(&url, None::<&()>).await
    }

    /// 获取好友分组列表
    /// - 返回: 好友分组列表（Vec<FriendGroup>）或 HTTP 错误
    pub async fn get_friend_groups(&self) -> Result<Vec<FriendGroup>, HttpError> {
        let url = "/lol-chat/v1/friend-groups";
        self.client.get(url).await
    }

    /// 获取当前用户的聊天信息
    /// - 返回: 当前用户信息（ChatPerson）或 HTTP 错误
    pub async fn get_me(&self) -> Result<ChatPerson, HttpError> {
        let url = "/lol-chat/v1/me";
        self.client.get(url).await
    }

    /// 获取所有会话列表
    /// - 返回: 会话列表（Vec<Conversation>）或 HTTP 错误
    pub async fn get_conversations(&self) -> Result<Vec<Conversation>, HttpError> {
        let url = "/lol-chat/v1/conversations";
        self.client.get(url).await
    }

    /// 获取指定会话的参与者列表
    /// - 参数: `id` - 会话唯一标识符
    /// - 返回: 参与者列表（Vec<ChatPerson>）或 HTTP 错误
    pub async fn get_participants(&self, id: &String) -> Result<Vec<ChatPerson>, HttpError> {
        let url = format!("/lol-chat/v1/conversations/{}/participants", id);
        self.client.get(&url).await
    }

    /// 修改当前用户的在线状态
    /// - 参数: `availability` - 目标在线状态（如在线、离线、勿扰等）
    /// - 返回: 成功时返回空，失败时返回 HTTP 错误
    pub async fn change_availability(
        &self,
        availability: AvailabilityType,
    ) -> Result<(), HttpError> {
        /// 状态修改请求体
        #[derive(Serialize)]
        struct Body {
            availability: AvailabilityType, // 在线状态字段
        }

        let url = "/lol-chat/v1/me";
        self.client.put(url, Some(&Body { availability })).await
    }

    /// 发送聊天消息到指定会话
    /// - 参数:
    ///   - `target_id` - 目标会话 ID（支持多种可转换为字符串的类型）
    ///   - `message` - 消息内容
    ///   - `r#type` - 消息类型（默认为 "chat"）
    ///   - `is_historical` - 是否为历史消息标记
    ///   - `summoner_id` - 发送者的召唤师 ID（可选）
    /// - 返回: 发送成功的消息详情（ChatMessage）或 HTTP 错误
    pub async fn chat_send(
        &self,
        target_id: impl ToString,
        message: &str,
        r#type: &str,
        is_historical: bool,
        summoner_id: Option<u64>,
    ) -> Result<ChatMessage, HttpError> {
        /// 消息发送请求体
        #[derive(Serialize)]
        struct Payload {
            body: String,          // 消息内容
            from_id: Option<u64>,  // 发送者 ID（可选）
            from_pid: String,      // 发送者平台 ID（暂为空）
            from_summoner_id: u64, // 发送者召唤师 ID（默认 0）
            id: String,            // 目标会话 ID
            is_historical: bool,   // 是否为历史消息
            timestamp: String,     // 时间戳（暂为空，由服务端填充）
            r#type: String,        // 消息类型
        }

        // 将目标 ID 转换为字符串（支持数字或字符串类型的 ID）
        let target_id_str = target_id.to_string();
        let payload = Payload {
            body: message.to_string(),
            from_id: summoner_id,
            from_pid: String::new(),
            from_summoner_id: summoner_id.unwrap_or(0), // 无 ID 时使用 0
            id: target_id_str.clone(),
            is_historical,
            timestamp: String::new(),
            r#type: r#type.to_string(),
        };

        let url = format!("/lol-chat/v1/conversations/{}/messages", target_id_str);

        self.client.post(&url, Some(&payload)).await
    }

    /// 获取指定聊天室的参与者列表
    /// - 参数: `chat_room_id` - 聊天室唯一标识符
    /// - 返回: 参与者列表（Vec<ChatPerson>）或 HTTP 错误
    pub async fn get_chat_participants(
        &self,
        chat_room_id: &str,
    ) -> Result<Vec<ChatPerson>, HttpError> {
        let url = format!("/lol-chat/v1/conversations/{}/participants", chat_room_id);
        self.client.get(&url).await
    }

    /// 修改当前用户的排位信息（段位展示）
    /// - 参数:
    ///   - `ranked_league_queue` - 排位队列类型（如 "RANKED_SOLO_5x5"）
    ///   - `ranked_league_tier` - 段位等级（如 "GOLD"、"PLATINUM"）
    ///   - `ranked_league_division` - 段位细分（如 "IV"、"III"，可选）
    /// - 返回: 更新后的用户信息（ChatPerson）或 HTTP 错误
    pub async fn change_ranked(
        &self,
        ranked_league_queue: &str,
        ranked_league_tier: &str,
        ranked_league_division: Option<&str>,
    ) -> Result<ChatPerson, HttpError> {
        /// 排位信息子结构
        #[derive(Serialize)]
        struct LolData {
            ranked_league_queue: String,            // 排位队列
            ranked_league_tier: String,             // 段位等级
            ranked_league_division: Option<String>, // 段位细分（可选）
        }

        /// 排位信息请求体
        #[derive(Serialize)]
        struct Payload {
            lol: LolData, // 嵌套的排位信息
        }

        let payload = Payload {
            lol: LolData {
                ranked_league_queue: ranked_league_queue.to_string(),
                ranked_league_tier: ranked_league_tier.to_string(),
                ranked_league_division: ranked_league_division.map(|s| s.to_string()),
            },
        };
        let url = "/lol-chat/v1/me";
        self.client.put(url, Some(&payload)).await
    }

    /// 发送好友请求
    /// - 参数:
    ///   - `game_name` - 目标用户的游戏名称
    ///   - `tag_line` - 目标用户的标签（如游戏内的 #XXXX 后缀）
    /// - 返回: 成功时返回空，失败时返回 HTTP 错误
    pub async fn friend_requests(&self, game_name: &str, tag_line: &str) -> Result<(), HttpError> {
        /// 好友请求体
        #[derive(Serialize)]
        struct Payload {
            game_name: String, // 游戏名称
            tag_line: String,  // 标签
            game_tag: String,  // 游戏标签（与 tag_line 一致）
        }

        let url = "/lol-chat/v2/friend-requests";
        self.client
            .post(
                url,
                Some(&Payload {
                    game_name: game_name.to_string(),
                    tag_line: tag_line.to_string(),
                    game_tag: tag_line.to_string(),
                }),
            )
            .await
    }

    /// 设置当前用户的聊天状态消息
    /// - 参数: `message` - 状态消息内容（如 "正在游戏中"）
    /// - 返回: 更新后的用户信息（ChatPerson）或 HTTP 错误
    pub async fn set_chat_status_message(&self, message: &str) -> Result<ChatPerson, HttpError> {
        /// 状态消息请求体
        #[derive(Serialize)]
        struct Payload {
            status_message: String, // 状态消息内容
        }

        let url = "/lol-chat/v1/me";
        self.client
            .put(
                url,
                Some(&Payload {
                    status_message: message.to_string(),
                }),
            )
            .await
    }
}
