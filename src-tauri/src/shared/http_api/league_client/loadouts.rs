use crate::{
    shared::http_api::league_client::httpclient::HttpClient,
    shared::types::game_data::AccountScopeLoadouts,
    utils::error::http_error::HttpError,
};
use serde::Serialize;

/// 表情类型枚举（对应 TypeScript 的 EmoteType 联合类型）
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmoteType {
    EmotesWheelCenter,
    EmotesWheelUpper,
    EmotesWheelRight,
    EmotesWheelUpperRight,
    EmotesWheelUpperLeft,
    EmotesWheelLower,
    EmotesWheelLeft,
    EmotesWheelLowerRight,
    EmotesWheelLowerLeft,
    EmotesStart,
    EmotesFirstBlood,
    EmotesAce,
    EmotesVictory,
}

/// 草莓难度配置项
#[derive(Debug, Serialize)]
struct StrawberryDifficultyLoadout {
    #[serde(rename = "STRAWBERRY_DIFFICULTY")]
    strawberry_difficulty: StrawberryDifficultyItem,
}

/// 草莓难度物品结构
#[derive(Debug, Serialize)]
struct StrawberryDifficultyItem {
    inventory_type: String,
    item_id: i32,
}

/// 表情配置项
#[derive(Debug, Serialize)]
struct EmoteLoadoutItem {
    inventory_type: String,
    item_id: i32,
}

/// 表情负载结构（动态键值对）
#[derive(Debug, Serialize)]
struct EmoteLoadout(serde_json::Value);

/// 通用负载请求结构
#[derive(Debug, Serialize)]
struct LoadoutPatchRequest<T> {
    loadout: T,
}

pub struct LoadoutsHttpApi {
    client: HttpClient,
}

impl LoadoutsHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 设置草莓难度
    pub async fn set_strawberry_difficulty(
        &self,
        content_id: &str,
        difficulty: i32,
    ) -> Result<(), HttpError> {
        let url = format!("/lol-loadouts/v4/loadouts/{}", content_id);
        let request = LoadoutPatchRequest {
            loadout: StrawberryDifficultyLoadout {
                strawberry_difficulty: StrawberryDifficultyItem {
                    inventory_type: "STRAWBERRY_LOADOUT_ITEM".to_string(),
                    item_id: difficulty,
                },
            },
        };
        self.client.patch(&url, Some(&request)).await
    }

    /// 设置表情
    pub async fn set_emotes(
        &self,
        content_id: &str,
        emotes: impl IntoIterator<Item = (EmoteType, i32)>,
    ) -> Result<(), HttpError> {
        let url = format!("/lol-loadouts/v4/loadouts/{}", content_id);
        
        // 构建表情键值对（转换为 JSON 结构）
        let mut emote_map = serde_json::Map::new();
        for (emote_type, item_id) in emotes {
            let emote_json = serde_json::to_value(EmoteLoadoutItem {
                inventory_type: "EMOTE".to_string(),
                item_id,
            }).map_err(|e| HttpError::JsonParse(e))?;
            // 将枚举转换为对应的字符串键（如 EmotesWheelCenter -> "EMOTES_WHEEL_CENTER"）
            let key = serde_json::to_string(&emote_type).map_err(|e| HttpError::JsonParse(e))?;
            emote_map.insert(key.trim_matches('"').to_string(), emote_json);
        }

        let request = LoadoutPatchRequest {
            loadout: EmoteLoadout(serde_json::Value::Object(emote_map)),
        };
        self.client.patch(&url, Some(&request)).await
    }

    /// 通用负载更新
    pub async fn patch_loadout(
        &self,
        content_id: &str,
        loadout: serde_json::Value,
    ) -> Result<(), HttpError> {
        let url = format!("/lol-loadouts/v4/loadouts/{}", content_id);
        let request = LoadoutPatchRequest { loadout };
        self.client.patch(&url, Some(&request)).await
    }

    /// 获取账户范围的负载配置
    pub async fn get_account_scope_loadouts(
        &self,
    ) -> Result<Vec<AccountScopeLoadouts>, HttpError> {
        let url = "/lol-loadouts/v4/loadouts/scope/account";
        self.client.get(url).await
    }
}