use crate::{
    shared::http_api::lcu::http::HttpClient,
    shared::types::league_client::gameflow::{GameflowPhase, GameflowSession},
    utils::error::http_error::HttpError,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct DodgeRequest {
    dodge_ids: Vec<u64>,
    phase: String,
}

#[derive(Clone)]
pub struct GameflowHttpApi {
    client: HttpClient,
}

impl GameflowHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_gameflow_phase(&self) -> Result<GameflowPhase, HttpError> {
        let url = "/lol-gameflow/v1/gameflow-phase";
        self.client.get(url).await
    }

    pub async fn get_gameflow_session(&self) -> Result<GameflowSession, HttpError> {
        let url = "/lol-gameflow/v1/session";
        self.client.get(url).await
    }

    pub async fn early_exit(&self) -> Result<(), HttpError> {
        let url = "/lol-gameflow/v1/early-exit";
        self.client.post(url, None::<&()>).await
    }

    pub async fn dodge(&self) -> Result<(), HttpError> {
        let url = "/lol-gameflow/v1/session/dodge";
        let request_body = DodgeRequest {
            dodge_ids: vec![1145141919810],
            phase: "ChampSelect".to_string(),
        };
        self.client.post(url, Some(&request_body)).await
    }

    pub async fn reconnect(&self) -> Result<(), HttpError> {
        let url = "/lol-gameflow/v1/reconnect";
        self.client.post(url, None::<&()>).await
    }

    pub async fn ack_failed_to_launch(&self) -> Result<(), HttpError> {
        let url = "/lol-gameflow/v1/ack-failed-to-launch";
        self.client.post(url, None::<&()>).await
    }
}
