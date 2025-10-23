use crate::{
    shared::http_api::league_client::httpclient::HttpClient, shared::types::league_client::login::LoginQueueState,
    utils::error::http_error::HttpError,
};
use serde::Serialize;

/// 登录相关 API 调用的请求体结构

pub struct LoginHttpApi {
    client: HttpClient,
}

impl LoginHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 执行闪避操作（英雄选择阶段退出）
    pub async fn dodge(&self) -> Result<(), HttpError> {
        #[derive(Debug, Serialize)]
        struct LoginInvokeRequest {
            data: Vec<String>,
        }

        // 直接在 URL 中嵌入查询参数
        let url = "/lol-login/v1/session/invoke?destination=lcdsServiceProxy&method=call&args=[\"\", \"teambuilder-draft\", \"quitV2\", \"\"]";

        // 构建请求体数据
        let request_body = LoginInvokeRequest {
            data: vec![
                String::new(),
                "teambuilder-draft".to_string(),
                "quitV2".to_string(),
                String::new(),
            ],
        };

        // 直接使用 post 方法（参数已嵌入 URL）
        self.client.post(url, Some(&request_body)).await
    }

    /// 获取登录队列状态
    pub async fn get_login_queue_state(&self) -> Result<LoginQueueState, HttpError> {
        let url = "/lol-login/v1/login-queue-state";
        self.client.get(url).await
    }
}
