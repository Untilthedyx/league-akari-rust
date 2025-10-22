use tauri_app_demo_lib::shared::http_api::league_client::champion_mastery::ChampionMasteryHttpApi;
use tauri_app_demo_lib::shared::http_api::league_client::httpclient::HttpClient;
use tauri_app_demo_lib::utils::process::get_client_info;
use tokio;

///  uri 格式如下，即我们主要用到 auth_token 和 port 两个字段
/// "https://riot:{auth.auth_token}@127.0.0.1:{auth.port}/{uri}",
#[tokio::main]
async fn main() {
    let info = get_client_info().unwrap();
    let client = HttpClient::new(info.port, info.auth_token).unwrap();
    let champ_mastery_api = ChampionMasteryHttpApi::new(client);
    let res = champ_mastery_api
        .get_player_champion_mastery("55cc79c4-3d20-535a-9bff-00b1867534d8")
        .await
        .unwrap();
    println!("Champion Mastery: {:?}", res);
}
