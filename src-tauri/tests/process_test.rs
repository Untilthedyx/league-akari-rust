use tauri_app_demo_lib::utils::process::is_running;

#[test]
fn test_is_running() {
    /// 检测 lol 客户端是否开启
    let process_name = "LeagueClient.exe";
    assert_eq!(is_running(process_name), false);
}