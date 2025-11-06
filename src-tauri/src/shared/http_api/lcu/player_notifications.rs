use crate::{
    shared::http_api::http::HttpClient,
    shared::types::league_client::player_notifications::*, utils::error::http_error::HttpError,
};

#[derive(Clone)]
pub struct PlayerNotificationsHttpApi {
    client: HttpClient,
}

impl PlayerNotificationsHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn post_notification(&self, data: &PlayerNotifications) -> Result<(), HttpError> {
        let url = "/player-notifications/v1/notifications";
        self.client.post(url, Some(&data)).await
    }

    pub async fn create_title_details_notification(
        &self,
        title: String,
        details: String,
    ) -> Result<(), HttpError> {
        let notification = PlayerNotifications {
            critical: Some(true),
            data: Some(NotificationData { details, title }),
            detail_key: Some("pre_translated_details".to_string()),
            dismissible: Some(true),
            state: Some("toast".to_string()),
            title_key: Some("pre_translated_title".to_string()),
            r#type: Some("default".to_string()), // Rust 中 type 是关键字，需用 r#type 转义
            ..Default::default()
        };

        // 调用 post_notification 并返回结果
        self.post_notification(&notification).await
    }
}
