use tauri_app_demo_lib::shared::http_api::sgp::SgpApi;
use tauri_app_demo_lib::shared::init::http::{get_http_client, init_http_client};
use tauri_app_demo_lib::shared::init::process::{get_process_info, init_process_info};
use tauri_app_demo_lib::utils::log::{config::LogConfig, init_logger};
use tokio::runtime::Builder;

#[test]
fn test_sgp_api() {
    async fn spg_api() {
        let config = LogConfig {
            level: "trace".to_string(),
            output: "console".to_string(),
            file_path: None,
        };

        if let Err(e) = init_logger(&config) {
            panic!("init logger error: {}", e.to_string());
        }

        init_process_info().await.unwrap();

        let a = get_process_info().await.unwrap();
        println!("{:?}", a);

        init_http_client().await.unwrap();
        println!("init http client success");
        let client = get_http_client().await.unwrap();
        let ranked = client.entitlements.get_entitlements_token().await.unwrap();
        let access_token = ranked.access_token;
        println!("{:?}", access_token);
        // let league_token = client
        //     .league_session
        //     .get_league_session_token()
        //     .await
        //     .unwrap();
        // println!("{}", league_token);

        let sgp_client = SgpApi::new(a.rso_platform_id.as_str(), a.region.as_str());
        let match_history = sgp_client
            .get_match_history("55cc79c4-3d20-535a-9bff-00b1867534d8", 0, 20)
            .await
            .unwrap();
        // println!("{:?}", match_history);
        let game_summary = sgp_client.get_game_summary(10361188578).await.unwrap();
        // println!("{:?}", game_summary);
        let game_detail = sgp_client.get_game_detail(10361188578).await.unwrap();
        // println!("{:?}", game_detail);
        let rank_stats = sgp_client
            .get_ranked_stats("55cc79c4-3d20-535a-9bff-00b1867534d8")
            .await
            .unwrap();
        // println!("{:?}", rank_stats);
        let summoner_list = sgp_client
            .get_summoner_by_puuid("55cc79c4-3d20-535a-9bff-00b1867534d8")
            .await
            .unwrap();
        // println!("{:?}", summoner_list);

        // 在游戏中才不会报错
        // let spectator_data = sgp_client
        //     .get_spectator_gameflow_by_puuid("55cc79c4-3d20-535a-9bff-00b1867534d8")
        //     .await
        //     .unwrap();
        // println!("{:?}", spectator_data);
        let replay_stream = sgp_client
            .get_match_history_replay_stream(10361188578)
            .await
            .unwrap();
        // println!("{:?}", replay_stream);
    }

    let runner = Builder::new_current_thread().enable_all().build().unwrap();
    runner.block_on(spg_api());
}
