use crate::shared::init::http::get_http_client;
use std::collections::HashMap;

/// must make sure the client is initialized
pub async fn get_rank_info() -> Result<HashMap<String, String>, String> {
    let mut info_map = HashMap::new();

    let client = match get_http_client().await {
        Ok(client) => client,
        Err(e) => return Err(e.to_string()),
    };

    let ranked = client.ranked.get_current_ranked_stats().await.unwrap();

    for queue in ranked.queues {
        if queue.queue_type == "RANKED_SOLO_5x5" || queue.queue_type == "RANKED_FLEX_SR" {
            info_map.insert(queue.queue_type.clone(), queue.highest_tier);
            info_map.insert(
                format!("{}_LP", queue.queue_type.clone()),
                queue.league_points.to_string(),
            );
        }
    }
    Ok(info_map)
}
