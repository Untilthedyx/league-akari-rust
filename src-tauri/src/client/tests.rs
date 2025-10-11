use crate::client::registry::get_lol_paths;

#[test]
fn test_get_lol_paths() {
    let paths = get_lol_paths().unwrap();
    assert_eq!(paths.0, "D:\\WeGameApps\\英雄联盟\\Launcher\\Client.exe");
    assert_eq!(paths.1, "D:\\WeGameApps\\英雄联盟\\WeGameLauncher\\launcher.exe");
}