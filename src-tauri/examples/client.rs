use tauri_app_demo_lib::client::detect::get_client_info;

///  uri 格式如下，即我们主要用到 auth_token 和 port 两个字段
/// "https://riot:{auth.auth_token}@127.0.0.1:{auth.port}/{uri}",
fn main() {
    let info = get_client_info();
    println!("{info:?}");
}
