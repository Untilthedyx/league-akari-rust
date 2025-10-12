use crate::client::commandinfo::{
    get_process_command_line, get_process_pid_by_name, parse_command_line, CommandInfo,
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
fn get_client_info() -> Result<CommandInfo, String> {
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
