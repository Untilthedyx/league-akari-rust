/// 游戏阶段

use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use crate::lcu::https::lcu_get;

#[derive(Debug, Clone)]
struct PhaseCache {
    last_phase: String,
    last_fetch_time: Option<Instant>,
}

static PHASE_CACHE: LazyLock<Mutex<PhaseCache>> = LazyLock::new(|| {
    Mutex::new(PhaseCache {
        last_phase: "".to_string(),
        last_fetch_time: None,
    })
});

/// 检查游戏状态
///
/// 这里如果在大厅则为 None，还有 ["ChampSelect", "InProgress", "PreEndOfGame", "EndOfGame"]; 这四种状态
pub async fn get_phase() -> Result<String, String> {
    let mut cache = PHASE_CACHE.lock().unwrap();
    if let Some(last_fetch_time) = cache.last_fetch_time {
        if last_fetch_time.elapsed() < Duration::from_secs(2) {
            return Ok(cache.last_phase.clone());
        }
    }

    let uri = format!("lol-gameflow/v1/gameflow-phase");
    let phase = lcu_get::<String>(&uri).await?;

    cache.last_phase = phase.clone();
    cache.last_fetch_time = Some(Instant::now());

    Ok(phase)
}
