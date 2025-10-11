use crate::utils::process::is_running;

#[test]
fn test_is_running() {
    let process_name = "notepad.exe";
    assert_eq!(is_running(process_name), false);
}
