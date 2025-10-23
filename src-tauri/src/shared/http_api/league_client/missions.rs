use crate::{
    shared::http_api::league_client::httpclient::HttpClient,
    shared::types::league_client::missions::{Mission, MissionData, MissionSeries},
    utils::error::http_error::HttpError,
};
use serde::Serialize;

pub struct MissionsHttpApi {
    client: HttpClient,
}

impl MissionsHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 更新玩家任务状态
    pub async fn put_player(
        &self,
        series_ids: Option<Vec<String>>,
        mission_ids: Option<Vec<String>>,
    ) -> Result<(), HttpError> {
        #[derive(Debug, Serialize, Default)]
        #[serde(rename_all = "camelCase")]
        struct PutPlayerRequest {
            series_ids: Option<Vec<String>>,
            mission_ids: Option<Vec<String>>,
        }

        let url = "/lol-missions/v1/player";
        let request = PutPlayerRequest {
            series_ids,
            mission_ids,
        };
        // 仅当有字段需要更新时才序列化，否则发送空对象
        self.client.put(url, Some(&request)).await
    }

    /// 更新任务奖励组
    pub async fn put_reward_groups(
        &self,
        id: &str,
        reward_groups: Vec<String>,
    ) -> Result<(), HttpError> {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct RewardGroupsRequest {
            reward_groups: Vec<String>,
        }
        let url = format!("/lol-missions/v1/player/{}/reward-groups", id);
        let request = RewardGroupsRequest { reward_groups };
        self.client.put(&url, Some(&request)).await
    }

    /// 更新玩家特定任务状态
    pub async fn put_player_mission(
        &self,
        mission_id: &str,
        reward_groups: Option<Vec<String>>,
    ) -> Result<(), HttpError> {
        #[derive(Debug, Serialize, Default)]
        #[serde(rename_all = "camelCase")]
        struct PutPlayerMissionRequest {
            reward_groups: Option<Vec<String>>,
        }

        let url = format!("/lol-missions/v1/player/{}", mission_id);
        let request = PutPlayerMissionRequest { reward_groups };
        self.client.put(&url, Some(&request)).await
    }

    /// 获取所有任务列表
    pub async fn get_missions(&self) -> Result<Vec<Mission>, HttpError> {
        let url = "/lol-missions/v1/missions";
        self.client.get(url).await
    }

    /// 获取任务系列列表
    pub async fn get_series(&self) -> Result<Vec<MissionSeries>, HttpError> {
        let url = "/lol-missions/v1/series";
        self.client.get(url).await
    }

    /// 获取任务数据
    pub async fn get_data(&self) -> Result<MissionData, HttpError> {
        let url = "/lol-missions/v1/data";
        self.client.get(url).await
    }
}
