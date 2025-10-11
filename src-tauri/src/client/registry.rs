// // src/client/registry.rs
// use crate::error::ClientError;
// use std::path::PathBuf;
// use winreg::enums::*;
// use winreg::RegKey; // 引用全局错误类型

// /// 从注册表读取 WeGame 安装路径
// pub fn get_wegame_install_path() -> Result<PathBuf, ClientError> {
//     let hkcu = RegKey::predef(HKEY_CURRENT_USER);
//     let wegame_key = hkcu.open_subkey("Software\\wegame")?;

//     // 尝试读取 InstallPath 或从 DefaultIcon 解析
//     let path_str: String = match wegame_key.get_value("InstallPath") {
//         Ok(val) => val,
//         Err(_) => {
//             let icon_path: String = wegame_key.get_value("DefaultIcon")?;
//             icon_path.split(',').next().unwrap_or("").replace("\"", "")
//         }
//     };

//     let path = PathBuf::from(path_str);
//     if path.exists() {
//         Ok(path)
//     } else {
//         Err(ClientError::PathNotFound)
//     }
// }
