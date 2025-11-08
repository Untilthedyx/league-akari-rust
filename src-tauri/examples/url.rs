use tauri_app_demo_lib::shared::process::get_client_info;

use tokio;

#[tokio::main]
async fn main() {
    let info = get_client_info().unwrap();
    println!(
        "基础网站：{}",
        format!("https://riot:{}@127.0.0.1:{}/", info.auth_token, info.port)
    );
}
