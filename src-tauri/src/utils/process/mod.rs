#[cfg(test)]
pub mod tests;

pub mod process_info;
use crate::utils::error::path_error::LolPathError;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

use crate::utils::process::process_info::{
    get_process_command_line, get_process_pid_by_name, parse_command_line, ProcessInfo,
};

/// 检测进程是否在运行
pub fn is_running() -> bool {
    let pids = get_process_pid_by_name("LeagueClientUx.exe").unwrap();
    if pids.is_empty() {
        return false;
    }
    true
}

/// 获取客户端信息
pub fn get_client_info() -> Result<ProcessInfo, String> {
    let pids = get_process_pid_by_name("LeagueClientUx.exe")?;

    if pids.is_empty() {
        return Err("未能找到英雄联盟客户端进程".to_string());
    }

    let mut cmd_line = String::new();
    let mut found_valid_process = false;

    for pid in pids {
        match get_process_command_line(pid) {
            Ok(temp_cmd_line) => {
                cmd_line = temp_cmd_line;
                found_valid_process = true;
                break;
            }
            Err(_) => {}
        }
    }

    if found_valid_process {
        return parse_command_line(&cmd_line);
    } else {
        return Err("未能获取到英雄联盟客户端进程的命令行信息".to_string());
    }
}

/// 从注册表检索英雄联盟的安装路径
pub fn get_lol_paths() -> Result<(String, String), LolPathError> {
    let possible_reg_paths = [
        (HKEY_CURRENT_USER, "Software\\Tencent\\LOL"),
        (HKEY_LOCAL_MACHINE, "Software\\Tencent\\LOL"),
    ];

    let install_path = possible_reg_paths
        .iter()
        .find_map(
            |&(hive, path)| match RegKey::predef(hive).open_subkey(path) {
                Ok(key) => key.get_value::<String, _>("InstallPath").ok(),
                Err(_) => None,
            },
        )
        .ok_or(LolPathError::InstallPathNotFound)?;

    let client_path_1 = Path::new(&install_path).join("Launcher").join("Client.exe");
    let client_path_2 = Path::new(&install_path)
        .join("WeGameLauncher")
        .join("launcher.exe");

    if !client_path_1.exists() {
        return Err(LolPathError::ExecutableNotFound(client_path_1.clone()));
    }

    let path1_str = client_path_1
        .to_str()
        .ok_or(LolPathError::InvalidUtf8Path)?
        .to_string();

    let path2_str = client_path_2
        .to_str()
        .ok_or(LolPathError::InvalidUtf8Path)?
        .to_string();

    Ok((path1_str, path2_str))
}
