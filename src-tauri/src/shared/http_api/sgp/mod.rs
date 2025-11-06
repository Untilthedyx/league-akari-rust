pub mod http;

use crate::shared::http_api::sgp::http::HttpClient;
use crate::shared::init::http::get_http_client;
use crate::shared::types::sgp::*;
use crate::utils::error::http_error::HttpError;

pub struct SgpApi {
    pub client: HttpClient,
}

impl SgpApi {
    // rso_platform_id 需要大写
    pub fn new(rso_platform_id: &str, region: &str) -> Self {
        let client = HttpClient::new(rso_platform_id, region).unwrap();
        Self { client }
    }

    pub async fn get_match_history(
        &self,
        player_puuid: &str,
    ) -> Result<SgpMatchHistoryLol, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let uri = format!(
            "/match-history-query/v1/products/lol/player/{}/SUMMARY",
            player_puuid
        );
        self.client
            .get::<SgpMatchHistoryLol>(uri.as_str(), Some(&token))
            .await
    }

    pub async fn get_game_summary(
        &self,
        game_id: i64,
        rso_platform_id: &str,
    ) -> Result<SgpGameSummaryLol, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let url = format!(
            "/match-history-query/v1/products/lol/{}_{}/SUMMARY",
            rso_platform_id, game_id
        );
        self.client
            .get::<SgpGameSummaryLol>(url.as_str(), Some(&token))
            .await
    }

    pub async fn get_game_detail(
        &self,
        game_id: i64,
        rso_platform_id: &str,
    ) -> Result<SgpGameDetailsLol, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .entitlements
            .get_entitlements_token()
            .await
            .unwrap()
            .access_token;

        let url = format!(
            "/match-history-query/v1/products/lol/{}_{}/DETAILS",
            rso_platform_id, game_id
        );
        self.client
            .get::<SgpGameDetailsLol>(url.as_str(), Some(&token))
            .await
    }

    pub async fn get_ranked_stats(&self, puuid: &str) -> Result<SgpRankedStats, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let url = format!("/leagues-ledge/v2/rankedStats/puuid/{}", puuid);
        self.client
            .get::<SgpRankedStats>(url.as_str(), Some(&token))
            .await
    }

    pub async fn get_summoner_by_puuid(
        &self,
        puuid: &str,
        rso_platform_id: &str,
    ) -> Result<Vec<SgpSummoner>, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let url = format!(
            "/summoner-ledge/v1/regions/{}/summoners/puuids",
            rso_platform_id
        );
        self.client
            .post::<String, Vec<SgpSummoner>>(url.as_str(), Some(&puuid.to_string()), Some(&token))
            .await
    }

    pub async fn get_spectator_gameflow_by_puuid(
        &self,
        puuid: &str,
        rso_platform_id: &str,
    ) -> Result<SpectatorData, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let url = format!(
            "/gsm/v1/ledge/spectator/region/{}/puuid/{}",
            rso_platform_id, puuid
        );
        self.client
            .get::<SpectatorData>(url.as_str(), Some(&token))
            .await
    }

    /// Readable 需要处理
    pub async fn get_match_history_replay_stream(
        &self,
        game_id: i64,
        rso_platform_id: &str,
    ) -> Result<Readable, HttpError> {
        // TODO: responseType: 'stream'
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let url = format!(
            "/match-history-query/v3/product/lol/matchId/{}_{}/infoType/replay",
            rso_platform_id, game_id
        );
        self.client.get::<Readable>(url.as_str(), Some(&token)).await
    }

    /// 这里 _________ 需要处理
    pub async fn get_end_of_game_stats(
        &self,
        game_id: i64,
        rso_platform_id: &str,
        puuid: &str,
    ) -> Result<_________, HttpError> {
        let lcu_client = get_http_client().await.unwrap();
        let token = lcu_client
            .league_session
            .get_league_session_token()
            .await
            .unwrap();

        let url = format!(
            "/stats/endOfGame/region/{}/gameId/{}/puuid/{}",
            rso_platform_id, game_id, puuid
        );
        self.client
            .get::<_________>(url.as_str(), Some(&token))
            .await
    }
}
