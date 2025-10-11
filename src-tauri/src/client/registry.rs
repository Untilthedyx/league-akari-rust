use crate::error::error::LolPathError;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

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
