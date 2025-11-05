/// 日志配置错误
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HttpError {
    #[error("构建client错误: {0}")]
    HttpClientBuild(reqwest::Error), // 构建 HTTP 客户端失败
    #[error("请求错误: {0}")]
    HttpRequest(reqwest::Error), // 网络错误、连接超时等
    #[error("响应错误: {0}")]
    HttpResponse(String), // 非 2xx 状态码（如 404、500）
    #[error("解析错误: {0}")]
    JsonParse(serde_json::Error), // 响应体 JSON 解析失败
    #[error("未找到资源: {0}")]
    NotFound(String), // 未找到资源
}