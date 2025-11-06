use crate::{
    shared::http_api::lcu::http::HttpClient, utils::error::http_error::HttpError,
};

#[derive(Clone)]
pub struct LeagueSessionHttpApi {
    client: HttpClient,
}

impl LeagueSessionHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_league_session_token(&self) -> Result<String, HttpError> {
        let uri = "/lol-league-session/v1/league-session-token";
        self.client.get(&uri).await
    }
}
