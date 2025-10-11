use crate::utils::process::is_running;

pub fn detect_league() -> bool {
    is_running("LeagueClient.exe")
}
