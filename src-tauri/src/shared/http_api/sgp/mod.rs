pub mod http;

use crate::shared::http_api::sgp::http::HttpClient;
use crate::shared::init::lcu::get_lcu_client;
use crate::shared::types::sgp::game_detail::SgpGameDetailsLol;
use crate::shared::types::sgp::game_summary::SgpGameSummaryLol;
use crate::shared::types::sgp::history::SgpMatchHistoryLol;
use crate::shared::types::sgp::rank_stats::SgpRankedStats;
use crate::shared::types::sgp::spectator_data::SgpSpectatorData;
use crate::shared::types::sgp::summoner::SgpSummoner;
use crate::utils::error::http_error::HttpError;
use tungstenite::Bytes;

#[derive(Debug, Clone)]
pub struct SgpApi {
    pub rso_platform_id: String,
    pub client: HttpClient,
}

impl SgpApi {
    pub fn new(rso_platform_id: &str, region: &str) -> Self {
        let client = HttpClient::new(rso_platform_id, region).unwrap();
        let rso_platform_id = rso_platform_id.to_uppercase();
        Self {
            rso_platform_id,
            client,
        }
    }

    pub async fn get_match_history(
        &self,
        player_puuid: &str,
        start_index: i32,
        count: i32,
    ) -> Result<SgpMatchHistoryLol, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let uri = format!(
            "/match-history-query/v1/products/lol/player/{}/SUMMARY?startIndex={}&count={}",
            player_puuid, start_index, count
        );
        let url = self.client.build_url(uri.as_str(), "match_history");
        self.client.get(url.as_str(), Some(&token)).await
    }

    pub async fn get_game_summary(&self, game_id: i64) -> Result<SgpGameSummaryLol, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let uri = format!(
            "/match-history-query/v1/products/lol/{}_{}/SUMMARY",
            self.rso_platform_id, game_id
        );
        let url = self.client.build_url(uri.as_str(), "match_history");
        self.client.get(url.as_str(), Some(&token)).await
    }

    pub async fn get_game_detail(&self, game_id: i64) -> Result<SgpGameDetailsLol, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let uri = format!(
            "/match-history-query/v1/products/lol/{}_{}/DETAILS",
            self.rso_platform_id, game_id
        );
        let url = self.client.build_url(uri.as_str(), "match_history");
        self.client.get(url.as_str(), Some(&token)).await
    }

    pub async fn get_ranked_stats(&self, puuid: &str) -> Result<SgpRankedStats, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let uri = format!("/leagues-ledge/v2/rankedStats/puuid/{}", puuid);
        let url = self.client.build_url(uri.as_str(), "common");
        self.client.get(url.as_str(), Some(&token)).await
    }

    pub async fn get_summoner_by_puuid(&self, puuid: &str) -> Result<Vec<SgpSummoner>, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let uri = format!(
            "/summoner-ledge/v1/regions/{}/summoners/puuids",
            self.rso_platform_id
        );
        let url = self.client.build_url(uri.as_str(), "common");
        self.client
            .post::<Vec<String>, Vec<SgpSummoner>>(
                url.as_str(),
                Some(&vec![puuid.to_string()]),
                Some(&token),
            )
            .await
    }

    /// 在这里如果没有在游戏中就代表 Player not found
    /// 获取游戏状态
    pub async fn get_spectator_gameflow_by_puuid(
        &self,
        puuid: &str,
    ) -> Result<SgpSpectatorData, HttpError> {
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let uri = format!(
            "/gsm/v1/ledge/spectator/region/{}/puuid/{}",
            self.rso_platform_id, puuid
        );
        let url = self.client.build_url(uri.as_str(), "common");
        self.client.get(url.as_str(), Some(&token)).await
    }

    // /// Readable 需要处理
    pub async fn get_match_history_replay_stream(&self, game_id: i64) -> Result<Bytes, HttpError> {
        // TODO: responseType: 'stream'
        let lcu_client = get_lcu_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let uri = format!(
            "/match-history-query/v3/product/lol/matchId/{}_{}/infoType/replay",
            self.rso_platform_id, game_id
        );
        let url = self.client.build_url(uri.as_str(), "match_history");
        self.client.get_stream(url.as_str(), Some(&token)).await
    }
}
