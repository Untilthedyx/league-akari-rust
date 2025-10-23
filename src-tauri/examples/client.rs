use tauri_app_demo_lib::shared::http_api::league_client::httpclient::HttpClient;
use tauri_app_demo_lib::shared::http_api::league_client::LeagueClientHttpApiAxiosHelper;
use tauri_app_demo_lib::utils::log::{config::LogConfig, init_logger};
use tauri_app_demo_lib::utils::process::get_client_info;
use tokio;
use tracing::warn;

///  uri 格式如下，即我们主要用到 auth_token 和 port 两个字段
/// "https://riot:{auth.auth_token}@127.0.0.1:{auth.port}/{uri}",
#[tokio::main]
async fn main() {
    let config = LogConfig {
        level: "trace".to_string(),
        output: "console".to_string(),
        file_path: None,
    };

    if let Err(e) = init_logger(&config) {
        panic!("init logger error: {}", e.to_string());
    }

    warn!("client start");

    let info = get_client_info().unwrap();
    let client = HttpClient::new(info.port, info.auth_token, true).unwrap();
    let league_client_api = LeagueClientHttpApiAxiosHelper::new(client);
    let res = league_client_api
        .ranked
        // .get_ranked_stats(&"55cc79c4-3d20-535a-9bff-00b1867534d8".to_string())
        .get_current_ranked_stats()
        .await
        .unwrap();
    println!("{:?}", res);
}