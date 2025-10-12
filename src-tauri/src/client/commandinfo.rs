use regex::Regex;
use winapi::shared::minwindef::{BYTE, DWORD, FALSE};
use winapi::shared::ntdef::{NTSTATUS, UNICODE_STRING};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_LIMITED_INFORMATION};

/// 获取进程快照
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

/// 打开指定进程
use winapi::um::processthreadsapi::OpenProcess;

// 这里即为 NtQueryInformationProcess 的 process_information_class 参数
//
// 此处为「进程命令行信息」
pub const PROCESS_COMMAND_LINE_INFORMATION: i32 = 60;

#[link(name = "ntdll")]
unsafe extern "system" {
    /// 在 rust 中声明并链接 windows 系统的 NtQueryInformationProcess 函数
    pub unsafe fn NtQueryInformationProcess(
        process_handle: HANDLE,
        process_information_class: i32,
        process_infomation: *mut std::ffi::c_void,
        process_information_length: usize,
        return_length: *mut u32,
    ) -> NTSTATUS;
}

/// 必须实现 Drop trait，否则在程序退出时，进程句柄不会被关闭
///
/// the `Drop` trait may only be implemented for local structs, enums, and unions
///
/// 因此这里必须使用一个结构体包裹 HANDLE
struct ProcessHandle(HANDLE);
impl Drop for ProcessHandle {
    fn drop(&mut self) {
        if !self.0.is_null() && self.0 != INVALID_HANDLE_VALUE {
            unsafe { CloseHandle(self.0) };
        }
    }
}

pub fn get_process_pid_by_name(name: &str) -> Result<Vec<DWORD>, String> {
    let name = name.to_lowercase();
    let mut pids: Vec<DWORD> = Vec::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot == INVALID_HANDLE_VALUE {
            return Err(format!(
                "无法创建进程快照:{}",
                std::io::Error::last_os_error()
            ));
        }
        let _snapshot_handle = ProcessHandle(snapshot);

        let mut entry: PROCESSENTRY32W = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32; // 是否可以将 u32 替换为 DWORD 这里表示计算 PROCESSENTRY32W 在内存中的大小

        if Process32FirstW(snapshot, &mut entry) == FALSE {
            return Err(format!(
                "无法获取第一个进程:{}",
                std::io::Error::last_os_error()
            ));
        }

        loop {
            let exe_file = &entry.szExeFile;
            let exe_name = String::from_utf16_lossy(
                &exe_file[..exe_file
                    .iter()
                    .position(|&x| x == 0)
                    .unwrap_or(exe_file.len())],
            )
            .to_lowercase(); // 处理宽字符

            if exe_name.contains(&name) {
                pids.push(entry.th32ProcessID);
            }

            if Process32NextW(snapshot, &mut entry) == FALSE {
                break;
            }
        }
    }

    Ok(pids)
}

pub fn get_process_command_line(pid: DWORD) -> Result<String, String> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid);
        if handle.is_null() {
            return Err(format!(
                "无法打开进程 Pid={}: {}",
                pid,
                std::io::Error::last_os_error()
            ));
        }
        let _process_handle = ProcessHandle(handle);

        let initial_size = 8192u32;
        let mut buffer: Vec<BYTE> = vec![0; initial_size as usize]; // 申请内存
        let mut return_size: DWORD = 0;

        NtQueryInformationProcess(
            handle,
            PROCESS_COMMAND_LINE_INFORMATION,
            buffer.as_mut_ptr() as *mut _,
            initial_size as _,
            &mut return_size as &mut _,
        );

        println!("{return_size}");

        buffer.truncate(return_size as usize);

        let ucs = &*(buffer.as_ptr() as *const UNICODE_STRING);
        if ucs.Buffer.is_null() || ucs.Length == 0 {
            return Err(format!("无法获取进程命令行信息, 信息不存在或为空值"));
        }

        // 这里将 unicode 值转化为 utf-16 然后将 utf-16 转化为 string
        let slice = std::slice::from_raw_parts(ucs.Buffer, (ucs.Length / 2) as usize);
        let cmd_line = String::from_utf16_lossy(slice);

        Ok(cmd_line)
    }
}

#[derive(Debug, Clone)]
pub struct CommandInfo {
    pub port: u32,
    pub pid: u32,
    pub auth_token: String,
    pub rso_platform_id: String,
    pub region: String,
    pub certificate: String,
    pub riot_client_port: u32,
    pub riot_client_auth_token: String,
}

const RIOT_CERTIFICATE: &str = "riot-certificate";

pub fn parse_command_line(s: &str) -> Result<CommandInfo, String> {
    let port_regex = Regex::new(r"--app-port=([0-9]+)").unwrap();
    let remoting_auth_regex = Regex::new(r"--remoting-auth-token=([\w\-_]+)").unwrap();
    let pid_regex = Regex::new(r"--app-pid=([0-9]+)").unwrap();
    let rso_platform_id_regex = Regex::new(r"--rso_platform_id=([\w\-_]+)").unwrap();
    let region_regex = Regex::new(r"--region=([\w\-_]+)").unwrap();
    let riot_client_port_regex = Regex::new(r"--riotclient-app-port=([0-9]+)").unwrap();
    let riot_client_auth_regex = Regex::new(r"--riotclient-auth-token=([\w\-_]+)").unwrap();

    let port = port_regex
        .captures(s)
        .and_then(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
        .ok_or_else(|| "未能解析端口号".to_string())?;

    let password = remoting_auth_regex
        .captures(s)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .ok_or_else(|| "未能解析认证令牌".to_string())?;

    let pid = pid_regex
        .captures(s)
        .and_then(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
        .ok_or_else(|| "未能获取进程号".to_string())?;

    let rso_platform_id = rso_platform_id_regex
        .captures(s)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_default();

    let region = region_regex
        .captures(s)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_default();

    let riot_client_port = riot_client_port_regex
        .captures(s)
        .and_then(|cap| cap.get(1).and_then(|m| m.as_str().parse().ok()))
        .unwrap_or_default();

    let riot_client_auth_token = riot_client_auth_regex
        .captures(s)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
        .unwrap_or_default();

    if port == 0 {
        return Err(format!(
            "port 获取失败!:{}",
            std::io::Error::last_os_error()
        ));
    }

    Ok(CommandInfo {
        port,
        pid,
        auth_token: password,
        rso_platform_id,
        region,
        certificate: RIOT_CERTIFICATE.to_string(),
        riot_client_port,
        riot_client_auth_token,
    })
}