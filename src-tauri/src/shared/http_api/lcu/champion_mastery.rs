use crate::shared::{http_api::http::HttpClient, types::league_client::champion_mastery::*};
use crate::utils::error::http_error::HttpError;
use serde::Serialize;

#[derive(Clone)]
pub struct ChampionMasteryHttpApi {
    client: HttpClient,
}

impl ChampionMasteryHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_player_champion_mastery_top_n(
        &self,
        puuid: &str,
        count: u32,
    ) -> Result<PlayerChampionMastery, HttpError> {
        #[derive(Serialize)]
        struct Body {
            skip_cache: bool,
        }

        let url = format!(
            "/lol-champion-mastery/v1/{}/champion-mastery/top?count={}",
            puuid, count
        );

        self.client
            .post::<Body, PlayerChampionMastery>(&url, Some(&Body { skip_cache: true }))
            .await
    }

    pub async fn get_player_champion_mastery(
        &self,
        puuid: &str,
    ) -> Result<Vec<Mastery>, HttpError> {
        let url = format!("/lol-champion-mastery/v1/{}/champion-mastery", puuid);
        self.client.get(&url).await
    }

    pub async fn ack_notifications(&self) -> Result<(), HttpError> {
        self.client
            .post("/lol-champion-mastery/v1/notifications/ack", None::<&()>)
            .await
    }
}
