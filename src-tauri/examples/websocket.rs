use tauri_app_demo_lib::shared::http_api::websocket::Websocket;
use tauri_app_demo_lib::utils::process::get_client_info;

#[tokio::main]
async fn main() {
    let info = get_client_info().unwrap();
    let on_message = |message: serde_json::Value| {
        println!("Received message: {}", message);
    };
    let mut ws = Websocket::new(info.port, info.auth_token.to_string(), on_message);

    println!("Connected to websocket");
    ws.connect().await.unwrap();
    ws.send("[5,\"OnJsonApiEvent\"]");

    println!("WebSocket listening... Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await.unwrap();
    ws.close();
}
