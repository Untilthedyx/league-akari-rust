use crate::{
    shared::http_api::league_client::httpclient::HttpClient,
    shared::types::league_client::match_history::{Game, GameTimeline, MatchHistory},
    utils::error::http_error::HttpError,
};

pub struct MatchHistoryHttpApi {
    client: HttpClient,
}

impl MatchHistoryHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取当前召唤师的比赛历史
    pub async fn get_current_summoner_match_history(&self) -> Result<MatchHistory, HttpError> {
        let url = "/lol-match-history/v1/products/lol/current-summoner/matches";
        self.client.get(url).await
    }

    /// 获取指定 PUUID 的比赛历史（参数直接嵌入 URL）
    /// - `puuid`: 召唤师的 PUUID
    /// - `beg_index`: 起始索引（默认 0）
    /// - `end_index`: 结束索引（默认 19）
    pub async fn get_match_history(
        &self,
        puuid: &str,
        beg_index: Option<i32>,
        end_index: Option<i32>,
    ) -> Result<MatchHistory, HttpError> {
        // 处理默认参数
        let beg_index = beg_index.unwrap_or(0);
        let end_index = end_index.unwrap_or(19);

        // 直接将参数拼接在 URL 中
        let url = format!(
            "/lol-match-history/v1/products/lol/{}/matches?begIndex={}&endIndex={}",
            puuid, beg_index, end_index
        );

        self.client.get(&url).await
    }

    /// 获取指定比赛的详细信息
    pub async fn get_game(&self, game_id: i64) -> Result<Game, HttpError> {
        let url = format!("/lol-match-history/v1/games/{}", game_id);
        self.client.get(&url).await
    }

    /// 获取指定比赛的时间线数据
    pub async fn get_timeline(&self, game_id: i64) -> Result<GameTimeline, HttpError> {
        let url = format!("/lol-match-history/v1/game-timelines/{}", game_id);
        self.client.get(&url).await
    }
}
