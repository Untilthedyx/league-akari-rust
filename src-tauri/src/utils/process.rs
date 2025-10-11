use sysinfo::System;

/// 检测指定名称的进程是否正在运行
pub fn is_running(process_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes()
        .values()
        .any(|process| process.name().eq_ignore_ascii_case(process_name))
}

#[test]
fn test_is_running() {
    let process_name = "notepad.exe";
    assert_eq!(is_running(process_name), false);
}
