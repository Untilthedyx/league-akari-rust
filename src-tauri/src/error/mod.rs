// use thiserror::Error;
// use std::io::Error as IoError;  // 明确导入并重命名

// #[derive(Error, Debug)]
// pub enum ClientError {
//     #[error("注册表操作失败: {0}")]
//     Registry(#[from] IoError),  // 现在可以正确识别IoError
    
//     #[error("文件系统错误: {0}")]
//     Io(#[from] IoError),
    
//     #[error("客户端路径不存在")]
//     PathNotFound,
    
//     #[error("客户端未安装")]
//     NotInstalled,
// }

// // 实现 Tauri 命令兼容的错误转换（将 ClientError 转为 String）
// impl From<ClientError> for String {
//     fn from(err: ClientError) -> Self {
//         err.to_string()
//     }
// }
    