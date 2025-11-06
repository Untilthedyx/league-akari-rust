//! SGP (Riot Games API) HTTP 客户端
//!
//! 此模块提供了与 Riot Games SGP API 交互的 HTTP 客户端实现。
//! 客户端使用 Bearer token 进行身份验证。

use crate::shared::constants::sgp;
use crate::utils::error::http_error::HttpError;
use reqwest::RequestBuilder;
use reqwest::{Client, Method, Response};
use serde::{de::DeserializeOwned, de::Error as SerdeError, Serialize};
use std::any::TypeId;
use tracing::{error, instrument, warn, Span};

/// SGP HTTP 客户端
///
/// 封装了与 Riot Games SGP API 通信的所有 HTTP 操作。
/// 客户端会自动处理：
/// - Bearer token 认证
/// - URL 构建（支持相对 URI）
///
/// # 字段
/// - `url`: 基础 URL（SGP API 服务器地址）
/// - `client`: 配置好的 reqwest Client 实例
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// 基础 URL（包含协议和主机）
    url: String,
    /// 配置好的 HTTP 客户端实例
    client: Client,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端实例
    ///
    /// # 参数
    /// - `rso_platform_id`: RSO 平台 ID（例如：`TENCENT_HN1`）
    /// - `region`: 地区代码（例如：`CN`）
    /// - `token`: 认证 token（用于 Bearer 认证）
    ///
    /// # 返回
    /// - `Ok(Self)`: 成功创建客户端实例
    /// - `Err(HttpError)`: 服务器配置不存在或客户端构建失败
    ///
    /// # 示例
    /// ```no_run
    /// let client = HttpClient::new(
    ///     "HN1",
    ///     "TENCENT",
    /// )?;
    /// ```
    pub fn new(rso_platform_id: &str, region: &str) -> Result<Self, HttpError> {
        let sgp_server = format!("{}_{}", region, rso_platform_id);

        let url = sgp::SGP_SERVERS
            .get(sgp_server.as_str())
            .ok_or_else(|| {
                error!("SGP server not found: {}", sgp_server);
                HttpError::NotFound(format!("SGP server '{}' not found", sgp_server))
            })?
            .match_history
            .to_string();

        let client = reqwest::Client::builder().build().map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            HttpError::HttpClientBuild(e)
        })?;

        Ok(Self { url, client })
    }

    fn build_url(&self, uri: &str) -> String {
        let uri = uri.trim_start_matches('/');
        format!("{}/{}", self.url, uri)
    }

    fn build_request(&self, method: Method, url: &str, token: Option<&str>) -> RequestBuilder {
        match token {
            Some(token) => {
                let mut builder = self.client.request(method, url);
                builder = builder.header("Authorization", format!("Bearer {}", token));
                builder
            }
            None => self.client.request(method, url),
        }
    }

    async fn check_response(
        &self,
        response: Result<Response, reqwest::Error>,
    ) -> Result<Response, HttpError> {
        match response {
            Ok(response) => {
                let status = response.status();
                Span::current().record("status", &status.as_u16());

                if status.is_success() {
                    Ok(response)
                } else {
                    if let Ok(text) = response.text().await {
                        if status.is_server_error() {
                            error!("Server error {}: {}", status, text);
                        } else {
                            warn!("Client error {}: {}", status, text);
                        }
                    }
                    Err(HttpError::HttpResponse(status.to_string()))
                }
            }
            Err(e) => {
                Span::current().record("status", format!("error: {}", e));
                Err(HttpError::HttpRequest(e))
            }
        }
    }

    fn process_json(&self, data: &impl Serialize) -> String {
        match serde_json::to_string(data) {
            Ok(s) if s.len() > 100 => format!("{}...", &s[..100]),
            Ok(s) => s,
            Err(e) => format!("Serialization failed: {}", e),
        }
    }

    async fn request_json<T, R>(
        &self,
        method: Method,
        url: &str,
        json: Option<&T>,
        token: Option<&str>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        let builder = self.build_request(method, url, token);

        // 处理请求体
        let response = match json {
            Some(data) => {
                let body_str = self.process_json(data);
                Span::current().record("request_body", body_str);
                builder.json(data).send().await
            }
            None => {
                Span::current().record("request_body", "none");
                builder.send().await
            }
        };

        // 处理响应状态
        let response = self.check_response(response).await?;

        // 处理响应体（根据 R 类型决定是否解析）
        if TypeId::of::<R>() == TypeId::of::<()>() {
            // 对于单元类型，不解析响应体
            Ok(R::default())
        } else {
            // 先读取响应体文本，以便提供更详细的错误信息
            let text = response.text().await.map_err(|e| {
                HttpError::JsonParse(SerdeError::custom(format!(
                    "Failed to read response body: {}",
                    e
                )))
            })?;

            // 尝试解析 JSON
            let data: R = serde_json::from_str(&text).map_err(|e| {
                // 如果解析失败，尝试提供更详细的错误信息
                let error_msg = if text.len() > 1000 {
                    format!(
                        "Failed to parse JSON response at line {} column {}: {}. Response preview: {}...",
                        e.line(),
                        e.column(),
                        e,
                        &text[..10000]
                    )
                } else {
                    format!(
                        "Failed to parse JSON response at line {} column {}: {}. Response: {}",
                        e.line(),
                        e.column(),
                        e,
                        text
                    )
                };
                HttpError::JsonParse(SerdeError::custom(error_msg))
            })?;

            let data_str = self.process_json(&data);
            Span::current().record("response_data", data_str);
            Ok(data)
        }
    }

    // -------------------------------------------------------------------------
    // 🧭 对外公开接口
    // -------------------------------------------------------------------------

    /// GET 请求
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "GET", status, response_data))]
    pub async fn get<R>(&self, uri: &str, token: Option<&str>) -> Result<R, HttpError>
    where
        R: HttpData,
    {
        self.request_json(Method::GET, uri, None::<&()>, token)
            .await
    }

    /// POST 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "POST", status, response_data))]
    pub async fn post<T, R>(
        &self,
        uri: &str,
        json: Option<&T>,
        token: Option<&str>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self.request_json(Method::POST, uri, json, token).await
    }

    /// PATCH 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "PATCH", status, response_data))]
    pub async fn patch<T, R>(
        &self,
        uri: &str,
        json: Option<&T>,
        token: Option<&str>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self.request_json(Method::PATCH, uri, json, token).await
    }

    /// PUT 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "PUT", status, response_data))]
    pub async fn put<T, R>(
        &self,
        uri: &str,
        json: Option<&T>,
        token: Option<&str>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self.request_json(Method::PUT, uri, json, token).await
    }

    /// DELETE 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "DELETE", status, response_data))]
    pub async fn delete<T, R>(
        &self,
        uri: &str,
        json: Option<&T>,
        token: Option<&str>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self.request_json(Method::DELETE, uri, json, token).await
    }
}

pub trait HttpData: Serialize + DeserializeOwned + Default + 'static {}

impl<T> HttpData for T where T: Serialize + DeserializeOwned + Default + 'static {}
