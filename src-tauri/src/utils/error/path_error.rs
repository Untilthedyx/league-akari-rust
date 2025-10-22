use std::path::PathBuf;
use thiserror::Error;

/// 英雄联盟路径获取相关的错误类型
#[derive(Debug, Error)]
pub enum LolPathError {
    /// 未找到LOL安装路径的注册表项
    #[error("未找到LOL安装路径注册表项")]
    InstallPathNotFound,

    /// 路径包含非UTF-8字符
    #[error("路径包含非UTF-8字符")]
    InvalidUtf8Path,

    /// LOL客户端可执行文件不存在
    #[error("LOL客户端可执行文件不存在: {:?}", .0)]
    ExecutableNotFound(PathBuf),
}
