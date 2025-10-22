use crate::utils::error::http_error::HttpError;
use reqwest::{Client, Method, Response};
use serde::{
    de::{DeserializeOwned, Error as SerdeError},
    Serialize,
};
use std::any::TypeId;
use tracing::{error, instrument, warn, Span};



/// 通用 HTTP 客户端封装（适用于 Riot 本地 API 或自签证书服务）
///
/// ✅ 支持：GET / POST / PUT / PATCH / DELETE  
/// ✅ 显式处理空请求体（通过 Option）  
/// ✅ 自动日志记录请求与响应数据  
/// ✅ 错误类型统一封装为 `HttpError`
#[derive(Debug, Clone)]
pub struct HttpClient {
    port: u32,
    token: String,
    client: Client,
}

impl HttpClient {
    /// 创建新的 HTTP 客户端
    /// - `accept_invalid_certs`: 是否接受无效证书（本地服务通常需要）
    pub fn new(port: u32, token: String, accept_invalid_certs: bool) -> Result<Self, HttpError> {
        let mut builder = Client::builder();

        if accept_invalid_certs {
            builder = builder.danger_accept_invalid_certs(true);
        }

        let client = builder.build().map_err(|e| {
            error!("Failed to create HTTP client: {}", e);
            HttpError::HttpClientBuild(e)
        })?;

        Ok(Self {
            port,
            token,
            client,
        })
    }

    /// 构建完整 URL
    fn build_url(&self, uri: &str) -> String {
        let uri = uri.trim_start_matches('/');
        format!(
            "https://riot:{}@127.0.0.1:{}/{}",
            self.token, self.port, uri
        )
    }

    /// 处理响应：记录状态码并统一错误类型
    async fn process_response(
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

    /// 处理 JSON 数据用于日志输出
    fn process_json(&self, data: &impl Serialize) -> String {
        match serde_json::to_string(data) {
            Ok(s) if s.len() > 100 => format!("{}...", &s[..100]),
            Ok(s) => s,
            Err(e) => format!("Serialization failed: {}", e),
        }
    }

    /// 通用请求处理内部方法
    async fn _request_internal<T, R>(
        &self,
        method: Method,
        uri: &str,
        json: Option<&T>,
    ) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        let url = self.build_url(uri);
        let builder = self.client.request(method.clone(), &url);

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
        let response = self.process_response(response).await?;

        // 处理响应体（根据 R 类型决定是否解析）
        if TypeId::of::<R>() == TypeId::of::<()>() {
            // 对于单元类型，不解析响应体
            Ok(R::default()) // 需要 default + 'static 约束
        } else {
            let data = response.json::<R>().await.map_err(|e| {
                HttpError::JsonParse(SerdeError::custom(format!(
                    "Failed to parse JSON response: {}",
                    e
                )))
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
    pub async fn get<R>(&self, uri: &str) -> Result<R, HttpError>
    where
        R: HttpData,
    {
        self._request_internal(Method::GET, uri, None::<&()>).await
    }

    /// POST 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "POST", status, response_data))]
    pub async fn post<T, R>(&self, uri: &str, json: Option<&T>) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self._request_internal(Method::POST, uri, json).await
    }

    /// PATCH 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "PATCH", status, response_data))]
    pub async fn patch<T, R>(&self, uri: &str, json: Option<&T>) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self._request_internal(Method::PATCH, uri, json).await
    }

    /// PUT 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "PUT", status, response_data))]
    pub async fn put<T, R>(&self, uri: &str, json: Option<&T>) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self._request_internal(Method::PUT, uri, json).await
    }

    /// DELETE 请求
    /// - 无请求体：传递 `None::<&()>`
    /// - 有请求体：传递 `Some(&data)`
    /// - 无响应体：指定返回类型为 `()`
    /// - 有响应体：指定具体的返回类型
    #[instrument(skip_all, fields(uri = %self.build_url(uri), method = "DELETE", status, response_data))]
    pub async fn delete<T, R>(&self, uri: &str, json: Option<&T>) -> Result<R, HttpError>
    where
        T: Serialize,
        R: HttpData,
    {
        self._request_internal(Method::DELETE, uri, json).await
    }
}


pub trait HttpData: Serialize + DeserializeOwned + Default + 'static {}

impl<T> HttpData for T
where
    T: Serialize + DeserializeOwned + Default + 'static,
{}

