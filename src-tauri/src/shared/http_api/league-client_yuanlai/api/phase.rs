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
        last_phase: String::new(),
        last_fetch_time: None,
    })
});

/// 检查游戏状态
///
/// 这里如果在大厅则为 None，还有 ["ChampSelect", "InProgress", "PreEndOfGame", "EndOfGame"]; 这四种状态
pub async fn get_phase() -> Result<String, String> {
    // 为了手动限制 MutexGuard 的生命周期，确保在执行 await 之前释放锁，从而避免之前提到的 Send 相关错误。 MutexGuard 没有 Send Trait
    {
        let cache = PHASE_CACHE.lock().unwrap();
        if let Some(last_fetch_time) = cache.last_fetch_time {
            if last_fetch_time.elapsed() < Duration::from_secs(2) {
                return Ok(cache.last_phase.clone());
            }
        }
    }

    let uri = "lol-gameflow/v1/gameflow-phase";
    let phase = lcu_get::<String>(uri).await?;

    let mut cache = PHASE_CACHE.lock().unwrap();
    cache.last_phase = phase.clone();
    cache.last_fetch_time = Some(Instant::now());

    Ok(phase)
}
