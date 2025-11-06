# http_api league_client

这里需要做很多东西，主要测试代码为：

```rust
use tauri_app_demo_lib::shared::http_api::httpclient::HttpClient;
use tauri_app_demo_lib::shared::http_api::league_client::LcuApi;
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
    let league_client_api = LcuApi::new(client);
    let res = league_client_api
        .ranked
        // .get_ranked_stats(&"55cc79c4-3d20-535a-9bff-00b1867534d8".to_string())
        .get_current_ranked_stats()
        .await
        .unwrap();
    println!("{:?}", res);
}
```

首先是 types 感觉很多都没对上，其次是 api 的实现，或许我们可以使用 `serde_json::Value` 来替换 `Result<(), HttpError>`，接着是 log 的配置，感觉 trace 会加上很多其他的东西，最好使用 Debug 或者 Info 来测试吧

在 types 实现上，感觉 AI 很好用的，直接使用 reqable 从 log 中获取请求相关发送就好了，然后将得到的结果使用 AI 转化一下格式；

