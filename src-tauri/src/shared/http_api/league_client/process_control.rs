use crate::{
    shared::http_api::league_client::httpclient::HttpClient, utils::error::http_error::HttpError,
};

pub struct ProcessControlHttpApi {
    client: HttpClient,
}

impl ProcessControlHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn quit(&self) -> Result<(), HttpError> {
        let url = "/process-control/v1/process/quit";
        self.client.post(url, None::<&()>).await
    }
}
