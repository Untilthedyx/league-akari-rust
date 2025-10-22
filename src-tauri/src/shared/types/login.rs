use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoginQueueState {
    pub approximate_wait_time_seconds: u32,
    pub estimated_position_in_queue: u32,
    pub max_displayed_position: u32,
    pub max_displayed_wait_time_seconds: u32,
}
