use tauri_app_demo_lib::shared::http_api::websocket::WebsocketClient;
use tauri_app_demo_lib::shared::process::get_client_info;

#[tokio::main]
async fn main() {
    let info = get_client_info().unwrap();
    let on_message = |message: serde_json::Value| {
        println!("Received message: {}", message);
    };
    let mut ws = WebsocketClient::new(info.port, info.auth_token.to_string());
    ws.on_message(on_message);
    println!("Connected to WebsocketClient");
    ws.connect().await.unwrap();
    ws.send("[5,\"OnJsonApiEvent\"]");

    println!("WebsocketClient listening... Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await.unwrap();
    ws.close();
}
