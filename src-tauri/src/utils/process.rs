use sysinfo::System;

pub fn is_running(process_name: &str) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.processes()
        .values()
        .any(|process| process.name().eq_ignore_ascii_case(process_name))
}