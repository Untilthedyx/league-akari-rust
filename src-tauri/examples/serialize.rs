// use serde::Serialize;
// use tauri_app_demo_lib::{
//     shared::http_api::league_client::httpclient::HttpClient,
//     shared::types::player_notifications::*, utils::error::http_error::HttpError,
// };

fn main() {
    // let notification = PlayerNotifications {
    //     critical: Some(true),
    //     data: Some(NotificationData {
    //         details: ("pre_translated_details".to_string()),
    //         title: ("asd".to_string()),
    //     }),
    //     detail_key: Some("pre_translated_details".to_string()),
    //     dismissible: Some(true),
    //     state: Some("toast".to_string()),
    //     title_key: Some("pre_translated_title".to_string()),
    //     r#type: Some("default".to_string()), // Rust 中 type 是关键字，需用 r#type 转义
    //     ..Default::default()
    // };

    // println!("{:?}", serde_json::to_string(&notification));
    let a = vec![1, 2, 3];
    println!("{:?}", serde_json::to_string(&a));
}
