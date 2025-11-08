use tauri_app_demo_lib::shared::init::lcu::{clear_lcu_client, get_lcu_client, init_lcu_client};
use tauri_app_demo_lib::shared::init::process::{get_process_info, init_process_info};
use tauri_app_demo_lib::utils::log::init_logger;
use tokio;

///  uri 格式如下，即我们主要用到 auth_token 和 port 两个字段
/// "https://riot:{auth.auth_token}@127.0.0.1:{auth.port}/{uri}",
#[tokio::main]
async fn main() {
    init_logger();
    init_lcu_client().await.unwrap();
    init_lcu_client().await.unwrap();
    let client = get_lcu_client().await.unwrap();
    let history = client
        .match_history
        .get_current_summoner_match_history()
        .await
        .unwrap();
    println!("{:?}", serde_json::to_string(&history).unwrap());
    tokio::signal::ctrl_c().await.unwrap();
}
