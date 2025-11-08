use crate::shared::http_api::lcu::http::HttpClient;
use crate::shared::init::game_data::{
    get_champion_info_cache, get_item_info_cache, get_perk_info_cache, get_spell_info_cache,
};
use crate::utils::error::http_error::HttpError;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

/// Asset HTTP API 封装结构体
#[derive(Clone)]
pub struct AssetHttpApi {
    client: HttpClient,
}

impl AssetHttpApi {
    /// 创建新的 AssetHttpApi 实例
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    /// 获取头像图标（Base64 编码）
    ///
    /// # 参数
    /// - `icon_id`: 头像图标 ID
    ///
    /// # 返回
    /// - Base64 编码的图片数据 URL，可直接用于 `<img src="...">`
    /// - 格式：`data:image/jpeg;base64,...`
    ///
    pub async fn get_profile_icon_base64(&self, icon_id: u32) -> Result<String, HttpError> {
        let uri = format!("/lol-game-data/assets/v1/profile-icons/{}.jpg", icon_id);
        self.get_image_as_base64(&uri).await
    }

    /// 获取英雄头像图标（Base64 编码）
    ///
    /// # 参数
    /// - `champion_id`: 英雄 ID
    ///
    /// # 返回
    /// - Base64 编码的图片数据 URL
    pub async fn get_champion_icon_base64(&self, champion_id: u32) -> Result<String, HttpError> {
        let cache = get_champion_info_cache().await;
        let item = cache
            .get(&champion_id.to_string())
            .ok_or_else(|| HttpError::NotFound(format!("未找到英雄图标: {}", champion_id)))?;

        self.get_image_as_base64(&item.icon_path.as_str()).await
    }

    /// 获取物品图标（Base64 编码）
    ///
    /// # 参数
    /// - `item_id`: 物品 ID
    ///
    /// # 返回
    /// - Base64 编码的图片数据 URL
    ///
    /// # 性能优化
    /// - 使用全局缓存存储图标 URL 路径（避免一次性加载所有图片）
    /// - 按需将 URL 转换为 base64，只在需要时才加载图片数据
    pub async fn get_item_icon_base64(&self, item_id: u32) -> Result<String, HttpError> {
        // 获取或初始化 URL 缓存
        let cache = get_item_info_cache().await;

        // 从缓存获取图标路径
        let item = cache
            .get(&item_id.to_string())
            .ok_or_else(|| HttpError::NotFound(format!("未找到物品图标: {}", item_id)))?;

        // 按需转换为 base64
        self.get_image_as_base64(&item.icon_path.as_str()).await
    }

    /// 获取召唤师技能图标（Base64 编码）
    ///
    /// # 参数
    /// - `spell_id`: 召唤师技能 ID（字符串格式，如 "1" 或 "SummonerFlash"）
    ///
    /// # 返回
    /// - Base64 编码的图片数据 URL
    ///
    /// # 性能优化
    /// - 使用全局缓存存储图标 URL 路径（避免一次性加载所有图片）
    /// - 按需将 URL 转换为 base64，只在需要时才加载图片数据
    pub async fn get_spell_icon_base64(&self, spell_id: &str) -> Result<String, HttpError> {
        // 获取或初始化 URL 缓存
        let cache = get_spell_info_cache().await;

        // 从缓存获取图标路径
        let spell = cache
            .get(spell_id)
            .ok_or_else(|| HttpError::NotFound(format!("未找到召唤师技能图标: {}", spell_id)))?;

        // 按需转换为 base64
        self.get_image_as_base64(&spell.icon_path.as_str()).await
    }

    /// 获取符文图标（Base64 编码）
    ///
    /// # 参数
    /// - `perk_id`: 符文 ID（字符串格式）
    ///
    /// # 返回
    /// - Base64 编码的图片数据 URL
    ///
    /// # 性能优化
    /// - 使用全局缓存存储图标 URL 路径（避免一次性加载所有图片）
    /// - 按需将 URL 转换为 base64，只在需要时才加载图片数据
    pub async fn get_perk_icon_base64(&self, perk_id: &str) -> Result<String, HttpError> {
        // 获取或初始化 URL 缓存
        let cache = get_perk_info_cache().await;

        // 从缓存获取图标路径
        let item = cache
            .get(perk_id)
            .ok_or_else(|| HttpError::NotFound(format!("未找到符文图标: {}", perk_id)))?;

        // 按需转换为 base64
        self.get_image_as_base64(&item.icon_path.as_str()).await
    }

    /// 获取图片并转换为 Base64 编码
    ///
    /// **推荐用于小体积图片（< 50KB）**，如头像、图标等
    ///
    /// # 优势
    /// - 前端可直接使用：`<img src="data:image/png;base64,..." />`
    /// - Tauri 通信简单：字符串类型，无需处理二进制数据
    ///
    /// # 参数
    /// - `uri`: 图片资源的 URI 路径
    ///
    /// # 返回
    /// - Base64 编码的数据 URL 字符串
    pub async fn get_image_as_base64(&self, uri: &str) -> Result<String, HttpError> {
        // 使用 HttpClient 的 get_image 方法获取二进制数据
        let (bytes, content_type) = self.client.get_image(uri).await?;

        // 转换为 Base64
        let base64_str = STANDARD.encode(&bytes);

        // 返回 Data URL 格式
        Ok(format!("data:{};base64,{}", content_type, base64_str))
    }
}
