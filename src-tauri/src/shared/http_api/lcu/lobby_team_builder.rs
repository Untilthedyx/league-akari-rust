use crate::{
    shared::http_api::http::HttpClient, utils::error::http_error::HttpError,
};

#[derive(Clone)]
pub struct LobbyTeamBuilderHttpApi {
    client: HttpClient,
}

impl LobbyTeamBuilderHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取英雄选择阶段的子集英雄列表（返回英雄ID数组）
    pub async fn get_champ_select_subset_champion_list(&self) -> Result<Vec<i32>, HttpError> {
        let url = "/lol-lobby-team-builder/champ-select/v1/subset-champion-list";
        self.client.get(url).await
    }
}
