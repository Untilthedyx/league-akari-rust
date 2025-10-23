use crate::shared::http_api::league_client::httpclient::HttpClient;
use crate::shared::types::league_client::summoner::{SummonerInfo, SummonerProfile};
use crate::utils::error::http_error::HttpError;
use serde::Serialize;
use urlencoding::encode;

/// 召唤师相关的 HTTP API 客户端
pub struct SummonerHttpApi {
    client: HttpClient,
}

impl SummonerHttpApi {
    /// 创建新的 SummonerHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取当前召唤师信息
    pub async fn get_current_summoner(&self) -> Result<SummonerInfo, HttpError> {
        let url = "/lol-summoner/v1/current-summoner";
        self.client.get(url).await
    }

    /// 根据召唤师 ID 获取信息
    pub async fn get_summoner(&self, id: u64) -> Result<SummonerInfo, HttpError> {
        let url = format!("/lol-summoner/v1/summoners/{}", id);
        self.client.get(&url).await
    }

    /// 根据 PUUID 获取召唤师信息
    pub async fn get_summoner_by_puuid(&self, puuid: &str) -> Result<SummonerInfo, HttpError> {
        let url = format!("/lol-summoner/v2/summoners/puuid/{}", encode(puuid));
        self.client.get(&url).await
    }

    /// 根据名称获取召唤师信息
    pub async fn get_summoner_by_name(&self, name: &str) -> Result<SummonerInfo, HttpError> {
        let encoded_name = encode(name);
        let url = format!("/lol-summoner/v1/summoners?name={}", encoded_name);
        self.client.get(&url).await
    }

    /// 检查名称可用性
    pub async fn check_availability(&self, name: &str) -> Result<bool, HttpError> {
        let encoded_name = encode(name);
        let url = format!(
            "/lol-summoner/v1/check-name-availability-new-summoners/{}",
            encoded_name
        );
        self.client.get(&url).await
    }

    /// 更新召唤师资料
    pub async fn update_summoner_profile(
        &self,
        inventory: Option<&String>,
        key: &String,
        value: &serde_json::Value,
    ) -> Result<serde_json::Value, HttpError> {
        #[derive(Serialize)]
        struct UpdateProfileDto {
            #[serde(skip_serializing_if = "Option::is_none")]
            inventory: Option<String>,
            key: String,
            value: serde_json::Value,
        }

        let data = UpdateProfileDto {
            inventory: inventory.cloned(),
            key: key.clone(),
            value: value.clone(),
        };

        let url = "/lol-summoner/v1/current-summoner/summoner-profile";
        self.client.post(url, Some(&data)).await
    }

    /// 更新召唤师名称
    pub async fn update_summoner_name(
        &self,
        name: &String,
    ) -> Result<serde_json::Value, HttpError> {
        let url = "/lol-summoner/v1/current-summoner/name";
        self.client.post(url, Some(&name)).await
    }

    /// 创建新召唤师名称
    pub async fn new_summoner_name(&self, name: &str) -> Result<serde_json::Value, HttpError> {
        let data = serde_json::json!({ "name": name });
        let url = "/lol-summoner/v1/summoners";
        self.client.post(url, Some(&data)).await
    }

    /// 设置召唤师背景皮肤
    pub async fn set_summoner_background_skin(
        &self,
        skin_id: u32,
    ) -> Result<serde_json::Value, HttpError> {
        let data = serde_json::json!({
            "key": "backgroundSkinId",
            "value": skin_id
        });
        let url = "/lol-summoner/v1/current-summoner/summoner-profile";
        self.client.post(url, Some(&data)).await
    }

    /// 设置召唤师背景增强
    pub async fn set_summoner_background_augments(
        &self,
        augment_id: &str,
    ) -> Result<serde_json::Value, HttpError> {
        let data = serde_json::json!({
            "key": "backgroundSkinAugments",
            "value": augment_id
        });
        let url = "/lol-summoner/v1/current-summoner/summoner-profile";
        self.client.post(url, Some(&data)).await
    }

    /// 批量获取召唤师别名信息
    pub async fn get_summoner_aliases(
        &self,
        name_tag_list: &[(String, String)], // (gameName, tagLine)
    ) -> Result<Vec<SummonerInfo>, HttpError> {
        #[derive(Serialize)]
        struct NameTag {
            game_name: String,
            tag_line: String,
        }

        let data: Vec<NameTag> = name_tag_list
            .iter()
            .map(|(game_name, tag_line)| NameTag {
                game_name: game_name.clone(),
                tag_line: tag_line.clone(),
            })
            .collect();

        let url = "/lol-summoner/v1/summoners/aliases";
        self.client.post(url, Some(&data)).await
    }

    /// 获取单个召唤师别名信息
    pub async fn get_summoner_alias(
        &self,
        name: &str,
        tag: &str,
    ) -> Result<Option<SummonerInfo>, HttpError> {
        let response = self
            .get_summoner_aliases(&[(name.to_string(), tag.to_string())])
            .await?;
        Ok(response.into_iter().next())
    }

    /// 获取当前召唤师资料
    pub async fn get_current_summoner_profile(&self) -> Result<SummonerProfile, HttpError> {
        let url = "/lol-summoner/v1/current-summoner/summoner-profile";
        self.client.get(url).await
    }

    /// 根据 PUUID 获取召唤师资料
    pub async fn get_summoner_profile(&self, puuid: &str) -> Result<SummonerProfile, HttpError> {
        let encoded_puuid = encode(puuid);
        let url = format!("/lol-summoner/v1/summoner-profile?puuid={}", encoded_puuid);
        self.client.get(&url).await
    }
}
