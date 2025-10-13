use base64::engine::general_purpose;
use base64::Engine;
use reqwest::{Client, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::{Duration, Instant};
use std::{sync::OnceLock};
use tauri::async_runtime::Mutex;

use crate::client::{commandinfo::CommandInfo, detect::get_client_info};

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();
static AUTH: OnceLock<Mutex<CommandInfo>> = OnceLock::new();
static LAST_REFRESH_TIME: OnceLock<Mutex<Instant>> = OnceLock::new();

fn get_http_client() -> &'static Client {
    HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(10))
            .build()
            .expect("创建 reqwest 客户端失败")
    })
}

async fn get_auth() -> CommandInfo {
    let auth = AUTH.get_or_init(|| Mutex::new(get_client_info().unwrap()));
    let guard = auth.lock().await.clone();
    guard
}

async fn refresh_auth() {
    let last_refresh = LAST_REFRESH_TIME.get_or_init(|| Mutex::new(Instant::now()));
    let mut last_refresh_guard = last_refresh.lock().await;

    let now = Instant::now();
    if now.duration_since(*last_refresh_guard) < Duration::from_secs(1) {
        return;
    }
    *last_refresh_guard = now;
    AUTH.set(Mutex::new(get_client_info().unwrap())).unwrap();
}

async fn build_url(uri: &str) -> String {
    let uri = uri.trim_start_matches('/');
    let auth = get_auth().await;
    format!(
        "https://riot:{}@127.0.0.1:{}/{}",
        auth.auth_token, auth.port, uri
    )
}

pub async fn lcu_get<T: DeserializeOwned + 'static>(uri: &str) -> Result<T, String> {
    for _ in 0..2 {
        let url = build_url(uri).await;
        let resp = get_http_client().get(&url).send().await;
        if let Some(resp) = handle_response(resp).await {
            return resp;
        }
    }
    Err("Get请求失败".to_string())
}

pub async fn lcu_post<T: DeserializeOwned + 'static, D: Serialize>(
    uri: &str,
    data: &D,
) -> Result<T, String> {
    for _ in 0..2 {
        let url = build_url(uri).await;
        let resp = get_http_client().post(&url).json(data).send().await;
        if let Some(resp) = handle_response(resp).await {
            return resp;
        }
    }
    Err("Post请求失败".to_string())
}

pub async fn lcu_patch<T: DeserializeOwned + 'static, D: Serialize>(
    uri: &str,
    data: &D,
) -> Result<T, String> {
    for _ in 0..2 {
        let url = build_url(uri).await;
        let resp = get_http_client().patch(&url).json(data).send().await;
        if let Some(resp) = handle_response(resp).await {
            return resp;
        }
    }
    Err("Patch请求失败".to_string())
}

/// 如果你的图片是 小体积资源（如 LCU 中的英雄头像、技能图标，通常 < 50KB），优先用 Base64，理由是：
/// 前端零额外处理：拿到 Base64 字符串后，直接赋值给 <img src="data:xxx;base64,xxx"> 即可展示，无需转换 Blob 或创建临时 URL，开发效率极高。
/// Tauri 通信简单：Base64 是字符串类型，Tauri 的 invoke 命令天然支持字符串传递，无需处理二进制数据的序列化 / 反序列化（Rust 端 general_purpose::STANDARD.encode 直接生成字符串，前端直接接收）
pub async fn lcu_get_img_as_base64(uri: &str) -> Result<String, String> {
    for _ in 0..2 {
        let url = build_url(uri).await;
        let resp = get_http_client().get(&url).send().await;

        match resp {
            Ok(r) if r.status().is_success() => {
                let content_type = r
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("image/png")
                    .to_string();
                let bytes = r.bytes().await.map_err(|e| format!("读取图片失败:{}", e))?;
                let base64_str = general_purpose::STANDARD.encode(bytes);
                return Ok(format!("data:{};base64,{}", content_type, base64_str));
            }
            _ => {
                refresh_auth().await;
            }
        }
    }
    Err("Get图片请求失败".to_string())
}

pub async fn lcu_get_img_as_binary(uri: &str) -> Result<(Vec<u8>, String), String> {
    for _ in 0..2 {
        let url = build_url(uri).await;
        let resp = get_http_client().get(&url).send().await;
        match resp {
            Ok(r) if r.status().is_success() => {
                let content_type = r
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("image/png")
                    .to_string();
                let bytes = r
                    .bytes()
                    .await
                    .map_err(|e| format!("读取图片失败:{}", e))?
                    .to_vec();
                return Ok((bytes, content_type));
            }
            _ => {
                refresh_auth().await;
            }
        }
    }
    Err("图片二进制请求失败".to_string())
}

async fn handle_response<T: DeserializeOwned + 'static>(
    resp: Result<Response, reqwest::Error>,
) -> Option<Result<T, String>> {
    match resp {
        Ok(r) if r.status().is_success() => {
            // 解析成功响应
            let data = r
                .json::<T>()
                .await
                .map_err(|e| format!("反序列化失败: {}", e))
                .ok()?;
            Some(Ok(data))
        }
        Ok(r) => {
            // 非成功状态码（如404、500等）
            refresh_auth().await;
            Some(Err(format!("请求失败，状态码: {}", r.status())))
        }
        Err(e) => {
            // 请求发送失败（如网络错误）
            refresh_auth().await;
            Some(Err(format!("请求发送失败: {}", e)))
        }
    }
}
