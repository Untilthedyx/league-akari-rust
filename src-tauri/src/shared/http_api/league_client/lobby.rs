use crate::{
    shared::http_api::league_client::httpclient::HttpClient,
    shared::types::league_client::lobby::{
        AvailableBot, EogStatus, Lobby, LobbyMember, QueueEligibility, ReceivedInvitation,
    },
    utils::error::http_error::HttpError,
};
use serde::Serialize;

/// 自定义游戏大厅配置
#[derive(Debug, Serialize)]
struct CustomGameLobbyConfig {
    game_mode: String,
    game_mutator: String,
    game_server_region: String,
    map_id: i32,
    mutators: Mutators,
    spectator_policy: String,
    team_size: i32,
}

/// 自定义游戏大厅
#[derive(Debug, Serialize)]
struct CustomGameLobby {
    configuration: CustomGameLobbyConfig,
    lobby_name: String,
    lobby_password: Option<String>,
}

/// 游戏突变器配置
#[derive(Debug, Serialize)]
struct Mutators {
    id: i32, // 1 自选 2 征召 3 禁用 4 全随机
}

/// 创建大厅请求体
#[derive(Debug, Serialize)]
struct CreateLobbyRequest {
    #[serde(rename = "customGameLobby", skip_serializing_if = "Option::is_none")]
    custom_game_lobby: Option<CustomGameLobby>,
    #[serde(rename = "isCustom")]
    is_custom: Option<bool>,
    #[serde(rename = "queueId", skip_serializing_if = "Option::is_none")]
    queue_id: Option<i32>,
}

/// 添加 bot 请求体
#[derive(Debug, Serialize)]
struct AddBotRequest {
    bot_difficulty: String,
    champion_id: i32,
    team_id: String, // 只能是 "100" 或 "200"
}

/// 玩家插槽配置（草莓模式 1）
#[derive(Debug, Serialize)]
struct PlayerSlotConfig {
    champion_id: i32,
    position_preference: String,
    spell1: i32,
    spell2: i32,
}

/// 草莓地图 ID 配置
#[derive(Debug, Serialize)]
struct StrawberryMapIdData {
    content_id: String,
    item_id: i32,
}

pub struct LobbyHttpApi {
    client: HttpClient,
}

impl LobbyHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 创建自定义大厅
    pub async fn create_custom_lobby(
        &self,
        mode: &str,
        map_id: i32,
        spectator_policy: &str,
        lobby_name: &str,
        lobby_password: Option<&str>,
        is_custom: bool,
    ) -> Result<Lobby, HttpError> {
        let url = "/lol-lobby/v2/lobby";
        let request = CreateLobbyRequest {
            custom_game_lobby: Some(CustomGameLobby {
                configuration: CustomGameLobbyConfig {
                    game_mode: mode.to_string(),
                    game_mutator: String::new(),
                    game_server_region: String::new(),
                    map_id,
                    mutators: Mutators { id: 1 }, // 默认自选
                    spectator_policy: spectator_policy.to_string(),
                    team_size: 5,
                },
                lobby_name: lobby_name.to_string(),
                lobby_password: lobby_password.map(|p| p.to_string()),
            }),
            is_custom: Some(is_custom),
            queue_id: None,
        };
        self.client.post(url, Some(&request)).await
    }

    /// 创建队列大厅
    pub async fn create_queue_lobby(&self, queue_id: i32) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/lobby";
        let request = CreateLobbyRequest {
            custom_game_lobby: None,
            is_custom: None,
            queue_id: Some(queue_id),
        };
        self.client.post(url, Some(&request)).await
    }

    /// 创建 5v5 训练模式大厅
    pub async fn create_practice_5x5(
        &self,
        name: Option<&str>,
        password: Option<&str>,
    ) -> Result<Lobby, HttpError> {
        self.create_custom_lobby(
            "PRACTICETOOL",
            11,
            "AllAllowed",
            name.unwrap_or("League Stalker Room"),
            password,
            true,
        )
        .await
    }

    /// 提升为房主
    pub async fn promote(&self, summoner_id: impl ToString) -> Result<i32, HttpError> {
        let url = format!(
            "/lol-lobby/v2/lobby/members/{}/promote",
            summoner_id.to_string()
        );
        self.client.post(&url, None::<&()>).await
    }

    /// 踢出成员
    pub async fn kick(&self, summoner_id: impl ToString) -> Result<i32, HttpError> {
        let url = format!(
            "/lol-lobby/v2/lobby/members/{}/kick",
            summoner_id.to_string()
        );
        self.client.post(&url, None::<&()>).await
    }

    /// 获取大厅成员列表
    pub async fn get_members(&self) -> Result<Vec<LobbyMember>, HttpError> {
        let url = "/lol-lobby/v2/lobby/members";
        self.client.get(url).await
    }

    /// 获取当前大厅信息
    pub async fn get_lobby(&self) -> Result<Lobby, HttpError> {
        let url = "/lol-lobby/v2/lobby";
        self.client.get(url).await
    }

    /// 解散大厅
    pub async fn delete_lobby(&self) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/lobby";
        self.client.delete(url, None::<&()>).await
    }

    /// 获取可用的人机列表
    pub async fn get_available_bots(&self) -> Result<Vec<AvailableBot>, HttpError> {
        let url = "/lol-lobby/v2/lobby/custom/available-bots";
        self.client.get(url).await
    }

    /// 检查是否可以添加人机
    pub async fn is_bot_enabled(&self) -> Result<bool, HttpError> {
        let url = "/lol-lobby/v2/lobby/custom/bots-enabled";
        self.client.get(url).await
    }

    /// 添加人机
    pub async fn add_bot(
        &self,
        bot_difficulty: &str,
        champ_id: i32,
        team_id: &str, // 必须是 "100" 或 "200"
    ) -> Result<(), HttpError> {
        // 实际使用时可添加 team_id 合法性校验
        let url = "/lol-lobby/v1/lobby/custom/bots";
        let request = AddBotRequest {
            bot_difficulty: bot_difficulty.to_string(),
            champion_id: champ_id,
            team_id: team_id.to_string(),
        };
        self.client.post(url, Some(&request)).await
    }

    /// 开始匹配
    pub async fn search_match(&self) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/lobby/matchmaking/search";
        self.client.post(url, None::<&()>).await
    }

    /// 取消匹配搜索
    pub async fn delete_search_match(&self) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/lobby/matchmaking/search";
        self.client.delete(url, None::<&()>).await
    }

    /// 再来一局
    pub async fn play_again(&self) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/play-again";
        self.client.post(url, None::<&()>).await
    }

    /// 获取赛后状态
    pub async fn get_eog_status(&self) -> Result<EogStatus, HttpError> {
        let url = "/lol-lobby/v2/party/eog-status";
        self.client.get(url).await
    }

    /// 接受邀请
    pub async fn accept_received_invitation(&self, invitation_id: &str) -> Result<(), HttpError> {
        let url = format!(
            "/lol-lobby/v2/received-invitations/{}/accept",
            invitation_id
        );
        self.client.post(&url, None::<&()>).await
    }

    /// 拒绝邀请
    pub async fn decline_received_invitation(&self, invitation_id: &str) -> Result<(), HttpError> {
        let url = format!(
            "/lol-lobby/v2/received-invitations/{}/decline",
            invitation_id
        );
        self.client.post(&url, None::<&()>).await
    }

    /// 获取收到的邀请列表
    pub async fn get_received_invitations(&self) -> Result<Vec<ReceivedInvitation>, HttpError> {
        let url = "/lol-lobby/v2/received-invitations";
        self.client.get(url).await
    }

    /// 获取队伍可参与的队列
    pub async fn get_eligible_party_queues(&self) -> Result<Vec<QueueEligibility>, HttpError> {
        let url = "/lol-lobby/v2/eligibility/party";
        self.client.post(url, None::<&()>).await
    }

    /// 获取自己可参与的队列
    pub async fn get_eligible_self_queues(&self) -> Result<Vec<QueueEligibility>, HttpError> {
        let url = "/lol-lobby/v2/eligibility/self";
        self.client.post(url, None::<&()>).await
    }

    /// 设置草莓模式 1 的玩家插槽
    pub async fn set_player_slots_strawberry1(
        &self,
        champion_id: i32,
        map_id: Option<i32>,
        difficulty_id: Option<i32>,
    ) -> Result<(), HttpError> {
        let url = "/lol-lobby/v1/lobby/members/localMember/player-slots";
        let config = PlayerSlotConfig {
            champion_id,
            position_preference: "UNSELECTED".to_string(),
            spell1: map_id.unwrap_or(1),
            spell2: difficulty_id.unwrap_or(1),
        };
        self.client.put(url, Some(&[config])).await
    }

    /// 设置草莓地图 ID
    pub async fn set_strawberry_map_id(
        &self,
        content_id: &str,
        item_id: i32,
    ) -> Result<(), HttpError> {
        let url = "/lol-lobby/v2/lobby/strawberryMapId";
        let data = StrawberryMapIdData {
            content_id: content_id.to_string(),
            item_id,
        };
        self.client.put(url, Some(&data)).await
    }
}
