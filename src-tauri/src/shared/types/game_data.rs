use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ------------------------------
// 召唤师技能相关类型
// ------------------------------

/// 召唤师技能信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SummonerSpell {
    /// 技能ID
    pub id: u32,
    /// 技能名称
    pub name: String,
    /// 技能描述
    pub description: String,
    /// 解锁所需召唤师等级
    pub summoner_level: u32,
    /// 冷却时间(秒)
    pub cooldown: u32,
    /// 适用游戏模式
    pub game_modes: Vec<String>,
    /// 图标路径
    pub icon_path: String,
}

// ------------------------------
// 符文相关类型
// ------------------------------

/// 符文风格集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Perkstyles {
    ///  schema版本
    pub schema_version: u32,
    /// 符文风格列表
    pub styles: Vec<Style>,
}

/// 符文风格
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    /// 风格ID
    pub id: u32,
    /// 风格名称
    pub name: String,
    /// 提示文本
    pub tooltip: String,
    /// 图标路径
    pub icon_path: String,
    /// 资源映射表
    pub asset_map: AssetMap,
    /// 是否为高级符文
    pub is_advanced: bool,
    /// 允许的子风格ID列表
    pub allowed_sub_styles: Vec<u32>,
    /// 子风格奖励
    pub sub_style_bonus: Vec<SubStyleBonus>,
    /// 符文槽位
    pub slots: Vec<Slot>,
    /// 默认页面名称
    pub default_page_name: String,
    /// 默认子风格
    pub default_sub_style: u32,
    /// 默认符文ID列表
    pub default_perks: Vec<u32>,
    /// 特殊情况下的默认符文
    pub default_perks_when_splashed: Vec<u32>,
    /// 每个子风格的默认属性符文
    pub default_stat_mods_per_sub_style: Vec<DefaultStatModsPerSubStyle>,
}

/// 子风格默认属性符文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DefaultStatModsPerSubStyle {
    /// 子风格ID
    pub id: String,
    /// 符文ID列表
    pub perks: Vec<u32>,
}

/// 符文槽位
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    /// 槽位类型
    pub r#type: String,
    /// 槽位标签
    pub slot_label: String,
    /// 可选符文ID列表
    pub perks: Vec<u32>,
}

/// 子风格奖励
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SubStyleBonus {
    /// 风格ID
    pub style_id: u32,
    /// 符文ID
    pub perk_id: u32,
}

/// 符文资源映射表
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AssetMap {
    /// 各类符文资源路径（根据实际需要扩展）
    #[serde(flatten)]
    pub assets: HashMap<String, String>,
    /// SVG图标
    pub svg_icon: String,
    /// 16x16 SVG图标
    pub svg_icon_16: String,
}

// ------------------------------
// 物品相关类型
// ------------------------------

/// 游戏物品信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    /// 物品ID
    pub id: u32,
    /// 物品名称
    pub name: String,
    /// 物品描述
    pub description: String,
    /// 是否为主动物品
    pub active: bool,
    /// 是否在商店中可购买
    pub in_store: bool,
    /// 合成所需物品ID列表
    pub from: Vec<u32>,
    /// 可合成的物品ID列表
    pub to: Vec<u32>,
    /// 物品分类
    pub categories: Vec<String>,
    /// 最大堆叠数量
    pub max_stacks: u32,
    /// 所需英雄（特定英雄专属）
    pub required_champion: String,
    /// 所需友方英雄（特定组合）
    pub required_ally: String,
    /// 所需buff货币名称
    pub required_buff_currency_name: String,
    /// 所需buff货币数量
    pub required_buff_currency_cost: u32,
    /// 特殊配方ID
    pub special_recipe: u32,
    /// 是否为附魔
    pub is_enchantment: bool,
    /// 基础价格
    pub price: u32,
    /// 总价格（含合成材料）
    pub price_total: u32,
    /// 图标路径
    pub icon_path: String,
}

// ------------------------------
// 英雄相关类型
// ------------------------------

/// 英雄简要信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionSimple {
    /// 英雄ID
    pub id: u32,
    /// 英雄名称
    pub name: String,
    /// 英雄别名
    pub alias: String,
    /// 方形头像路径
    pub square_portrait_path: String,
    /// 英雄定位
    pub roles: Vec<String>,
}

/// 游戏模式类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GameMode {
    #[serde(rename = "NEXUSBLITZ")]
    NexusBlitz, // 极限闪击
    #[serde(rename = "URF")]
    Urf, // 无限火力 / 无限乱斗
    #[serde(rename = "PRACTICETOOL")]
    PracticeTool, // 训练模式
    #[serde(rename = "SNOWURF")]
    SnowUrf, // 冰雪无限火力
    #[serde(rename = "TUTORIAL")]
    Tutorial, // 新手教程
    #[serde(rename = "CLASSIC")]
    Classic, // 经典
    #[serde(rename = "ARAM")]
    Aram, // 极地大乱斗
    #[serde(rename = "DOOMBOTSTEEMO")]
    DoombotSteemo, // 末日人机 - 提莫
    #[serde(rename = "ULTBOOK")]
    Ultbook, // 终极魔典
    #[serde(rename = "ONEFORALL")]
    OneForAll, // 克隆大作战
    #[serde(rename = "ARSR")]
    Arsr,
    #[serde(rename = "ASSASSINATE")]
    Assassinate, // 血月杀
    #[serde(rename = "FIRSTBLOOD")]
    FirstBlood, // 超载
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "STARGUARDIAN")]
    StarGuardian, // 星之守护者
    #[serde(rename = "BRAWL")]
    Brawl, // 神木之门
    #[serde(rename = "CHERRY")]
    Cherry, // 斗魂竞技场 (Arena)
    #[serde(rename = "STRAWBERRY")]
    Strawberry, // 无尽狂潮 (Swarm)
}

/// 游戏地图信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameMap {
    /// 地图ID
    pub id: u32,
    /// 地图名称
    pub name: String,
    /// 地图描述
    pub description: String,
    /// 地图字符串ID
    pub map_string_id: String,
}

/// 游戏地图资源映射
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameMapAsset {
    /// 地图资源详情（键为地图ID相关字符串）
    #[serde(flatten)]
    pub assets: HashMap<String, Vec<GameMapAssetDetails>>,
}

/// 游戏地图资源详情
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameMapAssetDetails {
    /// 是否为默认地图
    pub is_default: bool,
    /// 描述信息
    pub description: String,
    /// 地图字符串ID
    pub map_string_id: String,
    /// 游戏模式
    pub game_mode: String,
    /// 游戏模式名称
    pub game_mode_name: String,
    /// 游戏模式短名称
    pub game_mode_short_name: String,
    /// 游戏模式描述
    pub game_mode_description: String,
    /// 名称
    pub name: String,
    /// 游戏变异器
    pub game_mutator: String,
    /// 是否为随机游戏模式
    pub is_rgm: bool,
    /// 其他属性
    pub properties: serde_json::Value,
    /// 每个位置推荐的召唤师技能
    pub per_position_required_summoner_spells: serde_json::Value,
    /// 每个位置禁用的召唤师技能
    pub per_position_disallowed_summoner_spells: serde_json::Value,
    /// 资源路径集合
    pub assets: HashMap<String, String>,
    /// 本地化字符串
    pub loc_strings: serde_json::Value,
    /// 分类内容包
    pub categorized_content_bundles: serde_json::Value,
    /// 教程卡片
    pub tutorial_cards: Vec<serde_json::Value>,
}

/// 符文详情
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Perk {
    /// 符文ID
    pub id: u32,
    /// 符文名称
    pub name: String,
    /// 重大变更的版本
    pub major_change_patch_version: String,
    /// 提示文本
    pub tooltip: String,
    /// 短描述
    pub short_desc: String,
    /// 长描述
    pub long_desc: String,
    /// 推荐描述
    pub recommendation_descriptor: String,
    /// 图标路径
    pub icon_path: String,
    /// 赛后统计描述
    pub end_of_game_stat_descs: Vec<String>,
    /// 推荐描述属性
    pub recommendation_descriptor_attributes: RecommendationDescriptorAttributes,
}

/// 推荐描述属性
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationDescriptorAttributes {
    /// 实用性评分
    pub k_utility: Option<f32>,
    /// 爆发伤害评分
    pub k_burst_damage: Option<f32>,
    /// 每秒伤害评分
    pub k_damage_per_second: Option<f32>,
    /// 经济评分
    pub k_gold: Option<f32>,
    /// 移动速度评分
    pub k_move_speed: Option<f32>,
    /// 治疗评分
    pub k_healing: Option<f32>,
    /// 生存能力评分
    pub k_durability: Option<f32>,
    /// 冷却缩减评分
    pub k_cooldown: Option<f32>,
    /// 法力值评分
    pub k_mana: Option<f32>,
}

/// 队列映射表
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SimpleQueueMap {
    /// 队列信息（键为队列ID相关字符串）
    #[serde(flatten)]
    pub queues: HashMap<String, Queue>,
}

/// 英雄详细信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampDetails {
    /// 英雄ID
    pub id: u32,
    /// 英雄名称
    pub name: String,
    /// 英雄别名
    pub alias: String,
    /// 英雄头衔
    pub title: String,
    /// 简短传记
    pub short_bio: String,
    /// 战术信息
    pub tactical_info: ChampTacticalInfo,
    /// 玩法信息
    pub playstyle_info: ChampPlaystyleInfo,
    /// 方形头像路径
    pub square_portrait_path: String,
    /// 技能音效路径
    pub stinger_sfx_path: String,
    /// 选择时的语音路径
    pub choose_vo_path: String,
    /// 被禁用时的语音路径
    pub ban_vo_path: String,
    /// 英雄定位
    pub roles: Vec<String>,
    /// 推荐物品默认值
    pub recommended_item_defaults: Vec<serde_json::Value>,
    /// 皮肤列表
    pub skins: Vec<ChampSkin>,
    /// 被动技能
    pub passive: ChampPassive,
    /// 技能列表
    pub spells: Vec<ChampSpell>,
}

/// 英雄技能
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSpell {
    /// 技能键位（Q/W/E/R）
    pub spell_key: String,
    /// 技能名称
    pub name: String,
    /// 技能图标路径
    pub ability_icon_path: String,
    /// 技能视频路径
    pub ability_video_path: String,
    /// 技能视频封面路径
    pub ability_video_image_path: String,
    /// 技能消耗描述
    pub cost: String,
    /// 技能冷却描述
    pub cooldown: String,
    /// 技能描述
    pub description: String,
    /// 动态技能描述
    pub dynamic_description: String,
    /// 技能范围
    pub range: Vec<u32>,
    /// 消耗系数
    pub cost_coefficients: Vec<f32>,
    /// 冷却系数
    pub cooldown_coefficients: Vec<f32>,
    /// 伤害系数
    pub coefficients: ChampCoefficients,
    /// 效果数值
    pub effect_amounts: ChampEffectAmounts,
    /// 弹药系统（如凯南被动、千珏大招等）
    pub ammo: ChampAmmo,
    /// 最大等级
    pub max_level: u32,
}

/// 技能弹药系统
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampAmmo {
    /// 弹药恢复时间
    pub ammo_recharge_time: Vec<f32>,
    /// 最大弹药量
    pub max_ammo: Vec<u32>,
}

/// 技能效果数值
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampEffectAmounts {
    /// 效果数值映射（键为效果ID）
    #[serde(flatten)]
    pub effects: HashMap<String, Vec<f32>>,
}

/// 技能系数
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampCoefficients {
    /// 系数1
    pub coefficient1: f32,
    /// 系数2
    pub coefficient2: f32,
}

/// 英雄被动技能
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampPassive {
    /// 被动名称
    pub name: String,
    /// 被动图标路径
    pub ability_icon_path: String,
    /// 被动视频路径
    pub ability_video_path: String,
    /// 被动视频封面路径
    pub ability_video_image_path: String,
    /// 被动描述
    pub description: String,
}

/// 英雄皮肤
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSkin {
    /// 皮肤ID
    pub id: u32,
    /// 是否为基础皮肤
    pub is_base: bool,
    /// 皮肤名称
    pub name: String,
    ///  splash图路径
    pub splash_path: String,
    /// 未居中的splash图路径
    pub uncentered_splash_path: String,
    /// 头像路径
    pub tile_path: String,
    /// 加载界面路径
    pub load_screen_path: String,
    /// 皮肤类型
    pub skin_type: String,
    /// 稀有度
    pub rarity: String,
    /// 是否为 legacy皮肤
    pub is_legacy: bool,
    /// splash视频路径（可选）
    pub splash_video_path: Option<String>,
    /// 收藏splash视频路径（可选）
    pub collection_splash_video_path: Option<String>,
    /// 特色文本（可选）
    pub features_text: Option<String>,
    /// 炫彩路径（可选）
    pub chroma_path: Option<String>,
    /// 徽章列表
    pub emblems: Vec<ChampEmblem>,
    /// 地区稀有度ID
    pub region_rarity_id: u32,
    /// 稀有度宝石路径（可选）
    pub rarity_gem_path: Option<String>,
    /// 皮肤系列
    pub skin_lines: Option<Vec<ChampSkinLine>>,
    /// 皮肤描述（可选）
    pub description: Option<String>,
    /// 旧版加载界面路径（可选）
    pub load_screen_vintage_path: Option<String>,
    /// 炫彩列表
    pub chromas: Option<Vec<ChampChroma>>,
    /// 任务皮肤信息（可选）
    pub quest_skin_info: Option<QuestSkinInfo>,
    /// 皮肤增强效果
    pub skin_augments: Option<SkinAugments>,
}

/// 任务皮肤信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuestSkinInfo {
    /// 名称
    pub name: String,
    /// 产品类型
    pub product_type: String,
    /// 收藏描述
    pub collection_description: String,
    /// 描述信息
    pub description_info: Vec<serde_json::Value>,
    /// splash图路径
    pub splash_path: String,
    /// 未居中的splash图路径
    pub uncentered_splash_path: String,
    /// 头像路径
    pub tile_path: String,
    /// 收藏卡片路径
    pub collection_card_path: String,
    /// 皮肤等级
    pub tiers: Vec<Tier>,
}

/// 皮肤等级
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    /// ID
    pub id: u32,
    /// 名称
    pub name: String,
    /// 阶段
    pub stage: u32,
    /// 描述
    pub description: String,
    /// splash图路径
    pub splash_path: String,
    /// 未居中的splash图路径
    pub uncentered_splash_path: String,
    /// 头像路径
    pub tile_path: String,
    /// 加载界面路径
    pub load_screen_path: String,
    /// 短名称
    pub short_name: String,
    /// splash视频路径
    pub splash_video_path: Option<String>,
    /// 收藏splash视频路径
    pub collection_splash_video_path: Option<String>,
    /// 收藏卡片悬停视频路径
    pub collection_card_hover_video_path: Option<String>,
    /// 皮肤增强效果
    pub skin_augments: SkinAugments,
}

/// 覆盖层
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    /// 居中的LC覆盖层路径
    pub centered_lc_overlay_path: String,
    /// 未居中的LC覆盖层路径
    pub uncentered_lc_overlay_path: String,
    /// 社交卡片LC覆盖层路径
    pub social_card_lc_overlay_path: String,
    /// 头像LC覆盖层路径
    pub tile_lc_overlay_path: String,
}

/// 边框
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Borders {
    /// 第0层边框
    pub layer0: Vec<Layer0>,
    /// 第1层边框（可选）
    pub layer1: Option<Vec<Layer0>>,
}

/// 边框层
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Layer0 {
    /// 内容ID
    pub content_id: String,
    /// 层级
    pub layer: u32,
    /// 优先级
    pub priority: u32,
    /// 边框路径
    pub border_path: String,
}

/// 皮肤增强效果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkinAugments {
    /// 边框
    pub borders: Borders,
    /// 增强效果（可选）
    pub augments: Option<Vec<Augment2>>,
}

/// 增强效果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Augment2 {
    /// 内容ID
    pub content_id: String,
    /// 覆盖层
    pub overlays: Vec<Overlay>,
}

/// 英雄炫彩
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampChroma {
    /// 炫彩ID
    pub id: u32,
    /// 炫彩名称
    pub name: String,
    /// 炫彩路径
    pub chroma_path: String,
    /// 颜色列表
    pub colors: Vec<String>,
    /// 描述列表
    pub descriptions: Vec<ChampDescription>,
    /// 稀有度列表
    pub rarities: Vec<ChampRarity>,
}

/// 地区稀有度
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampRarity {
    /// 地区
    pub region: String,
    /// 稀有度
    pub rarity: u32,
}

/// 地区描述
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampDescription {
    /// 地区
    pub region: String,
    /// 描述
    pub description: String,
}

/// 皮肤系列
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSkinLine {
    /// 系列ID
    pub id: u32,
}

/// 英雄徽章
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampEmblem {
    /// 徽章名称
    pub name: String,
    /// 徽章路径
    pub emblem_path: ChampEmblemPath,
    /// 位置信息
    pub positions: ChampPositions,
}

/// 徽章位置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampPositions {}

/// 徽章路径
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampEmblemPath {
    /// 大尺寸徽章
    pub large: String,
    /// 小尺寸徽章
    pub small: String,
}

/// 英雄玩法信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampPlaystyleInfo {
    /// 伤害评分
    pub damage: u32,
    /// 生存能力评分
    pub durability: u32,
    /// 控制能力评分
    pub crowd_control: u32,
    /// 机动性评分
    pub mobility: u32,
    /// 辅助能力评分
    pub utility: u32,
}

/// 英雄战术信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampTacticalInfo {
    /// 风格
    pub style: u32,
    /// 难度
    pub difficulty: u32,
    /// 伤害类型
    pub damage_type: String,
}

// ------------------------------
// 队列相关类型
// ------------------------------

/// 队列枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueEnum {
    BotIntro = 830,        // 新手
    BotIntermediate = 840, // 中等
    BotBeginner = 850,     // 普通
    Aram = 450,            // 极地大乱斗
    Custom = 0,            // 自定义
    RankSolo = 420,        // 单排
    RankFlex = 440,        // 灵活
    Arurf = 900,           // 无限火力
}

impl QueueEnum {
    /// 判断是否为人机队列
    pub fn is_bot_queue(&self) -> bool {
        matches!(
            self,
            QueueEnum::BotBeginner | QueueEnum::BotIntermediate | QueueEnum::BotIntro
        )
    }
}

/// 旧版队列信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueueLegacy {
    /// 名称
    pub name: String,
    /// 短名称
    pub short_name: String,
    /// 描述
    pub description: String,
    /// 详细描述
    pub detailed_description: String,
}

/// 队列信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    /// 队列ID
    pub id: u32,
    /// 名称
    pub name: String,
    /// 短名称
    pub short_name: String,
    /// 描述
    pub description: String,
    /// 详细描述
    pub detailed_description: String,
    /// 游戏选择模式组
    pub game_select_mode_group: String,
    /// 游戏选择分类
    pub game_select_category: String,
    /// 游戏选择优先级
    pub game_select_priority: u32,
}

/// 选人模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PickMode {
    #[serde(rename = "AllRandomPickStrategy")]
    AllRandomPickStrategy,
    #[serde(rename = "SimulPickStrategy")]
    SimulPickStrategy,
}

/// 强化符文
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Augment {
    /// ID
    pub id: u32,
    /// 名称（TRA语言）
    pub name_tra: String,
    /// 小图标路径
    pub augment_small_icon_path: String,
    /// 稀有度
    pub rarity: String,
}

// ------------------------------
// 无尽狂潮相关类型
// ------------------------------

/// 无尽狂潮中心数据
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StrawberryHub {
    /// 允许使用的英雄
    pub allowed_champions: StAllowedChampions,
    /// 地图显示信息列表
    pub map_display_info_list: Vec<MapDisplayInfoList>,
    /// 进度组
    pub progress_groups: Vec<ProgressGroup>,
    /// 强化组
    pub power_up_groups: Vec<PowerUpGroup>,
    /// 赛后叙事语音
    pub eo_g_narrative_barks: Vec<EoGNarrativeBark>,
}

/// 赛后叙事语音
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EoGNarrativeBark {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 语音内容
    pub value: EoGNarrativeBarkValue,
}

/// 赛后叙事语音内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EoGNarrativeBarkValue {
    /// 奖励组
    pub reward_group: RewardGroup,
    /// 标题
    pub title: String,
    /// 子标题
    pub sub_header: String,
    /// 内容
    pub content: String,
    /// 详情文本1
    pub detail_text_line1: String,
    /// 详情文本2
    pub detail_text_line2: String,
    /// 详情文本3
    pub detail_text_line3: String,
    /// 图片路径
    pub image: String,
    /// 图标路径
    pub icon_image: String,
    /// 是否为原始生物
    pub is_primordian: bool,
}

/// 奖励组
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardGroup {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 奖励策略
    pub reward_strategy: String,
    /// 选择策略配置
    pub selection_strategy_config: SelectionStrategyConfig2,
    /// 奖励列表
    pub rewards: Vec<RewardGroupReward>,
}

/// 奖励组奖励
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RewardGroupReward {
    /// 标题
    pub title: String,
    /// 详情
    pub details: String,
    /// 媒体资源
    pub media: Option<Media>,
    /// 物品ID
    pub item_id: String,
    /// 物品类型（可选）
    pub item_type: Option<String>,
}

/// 强化组
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PowerUpGroup {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 强化组内容
    pub value: PowerUpGroupValue,
}

/// 强化组内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PowerUpGroupValue {
    /// 名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 图标路径
    pub icon_image: String,
    /// 增益列表
    pub boons: Vec<Boon>,
    /// 前置增益（可选）
    pub prerequisite_boon: Option<PrerequisiteBoon>,
}

/// 增益
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Boon {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 增益内容
    pub value: PrerequisiteBoon,
}

/// 进度组
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProgressGroup {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 进度组内容
    pub value: ProgressGroupValue,
}

/// 进度组内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProgressGroupValue {
    /// 名称
    pub name: String,
    /// 图标路径
    pub icon_image: String,
    /// 里程碑列表
    pub milestones: Vec<Milestone>,
    /// 前置增益（可选）
    pub prerequisite_boon: Option<PrerequisiteBoon>,
}

/// 前置增益
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrerequisiteBoon {
    /// 内容ID
    pub content_id: String,
    /// 商品ID
    pub offer_id: String,
    /// 物品ID
    pub item_id: u32,
    /// 商品价格
    pub offer_price: u32,
    /// 简短价值摘要
    pub short_value_summary: String,
}

/// 里程碑
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Milestone {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 里程碑内容
    pub value: MilestoneValue,
}

/// 里程碑内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MilestoneValue {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 触发值
    pub trigger_value: u32,
    /// 属性列表
    pub properties: Vec<Property>,
    /// 计数器
    pub counter: Counter,
}

/// 计数器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
}

/// 属性
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    /// ID
    pub id: String,
    /// 名称
    pub name: String,
    /// 描述
    pub description: String,
    /// 奖励策略
    pub reward_strategy: String,
    /// 选择策略配置
    pub selection_strategy_config: SelectionStrategyConfig2,
    /// 奖励列表
    pub rewards: Vec<PropertyReward>,
}

/// 属性奖励
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyReward {
    /// 标题（可选）
    pub title: Option<String>,
    /// 详情（可选）
    pub details: Option<String>,
    /// 媒体资源（可选）
    pub media: Option<Media>,
    /// 物品ID（可选）
    pub item_id: Option<String>,
    /// 物品类型（可选）
    pub item_type: Option<String>,
    /// 战利品奖励（可选）
    pub loot_reward: Option<LootReward>,
}

/// 战利品奖励
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LootReward {
    /// ID
    pub id: String,
    /// 本地化标题
    pub localized_title: String,
    /// 本地化详情
    pub localized_details: String,
    /// 奖励类型
    pub reward_type: String,
    /// 数量
    pub quantity: u32,
    /// 媒体资源
    pub media: Media,
    /// 授予的战利品（空）
    pub loot_item_to_grant: Option<serde_json::Value>,
    /// 旧版战利品
    pub legacy_loot_item: String,
}

/// 媒体资源
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Media {
    /// 图标路径
    pub icon_path: IconPath,
}

/// 图标路径
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IconPath {
    /// 图片路径
    pub image: String,
    /// Splash图路径
    pub splash_image: String,
}

/// 选择策略配置（外层）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectionStrategyConfig2 {
    /// 选择策略配置
    pub selection_strategy_config: SelectionStrategyConfig,
}

/// 选择策略配置（内层）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SelectionStrategyConfig {
    /// 最小可选数量
    pub min_selections_allowed: u32,
    /// 最大可选数量
    pub max_selections_allowed: u32,
}

/// 地图显示信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapDisplayInfoList {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 地图显示信息内容
    pub value: MapDisplayInfoListValue,
}

/// 地图显示信息内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct MapDisplayInfoListValue {
    /// 名称
    pub name: String,
    /// 提示文本
    pub bark: String,
    /// 提示图片
    pub bark_image: String,
    /// 地图
    pub map: StMap,
    /// 完成地图增益（空）
    pub completed_map_boon: Option<serde_json::Value>,
}

/// 无尽狂潮地图
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StMap {
    /// 显示名称
    pub display_name: String,
    /// 内容ID
    pub content_id: String,
    /// 商品ID
    pub offer_id: String,
    /// 物品ID
    pub item_id: u32,
}

/// 允许使用的英雄
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StAllowedChampions {
    /// 英雄列表
    pub champions: Vec<StAllowedChampion>,
}

/// 允许使用的英雄详情
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StAllowedChampion {
    /// ID
    pub id: String,
    /// 排序值
    pub o: u32,
    /// 英雄值
    pub value: ChampionValue,
}

/// 英雄值
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampionValue {
    /// 内容ID
    pub content_id: String,
    /// 商品ID
    pub offer_id: String,
    /// 物品ID
    pub item_id: u32,
}

// ------------------------------
// 账户配置相关类型
// ------------------------------

/// 账户范围配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AccountScopeLoadouts {
    /// ID
    pub id: String,
    /// 物品ID（空）
    pub item_id: Option<serde_json::Value>,
    /// 配置内容
    pub loadout: Loadout,
    /// 名称
    pub name: String,
    /// 刷新时间
    pub refresh_time: String,
    /// 范围
    pub scope: String,
}

/// 配置内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Loadout {
    /// 宠物槽位
    #[serde(rename = "COMPANION_SLOT")]
    pub companion_slot: CompanionSlot,
    /// ACE表情
    #[serde(rename = "EMOTES_ACE")]
    pub emotes_ace: CompanionSlot,
    /// 一血表情
    #[serde(rename = "EMOTES_FIRST_BLOOD")]
    pub emotes_first_blood: CompanionSlot,
    /// 开始表情
    #[serde(rename = "EMOTES_START")]
    pub emotes_start: CompanionSlot,
    /// 胜利表情
    #[serde(rename = "EMOTES_VICTORY")]
    pub emotes_victory: CompanionSlot,
    /// 表情轮盘中心
    #[serde(rename = "EMOTES_WHEEL_CENTER")]
    pub emotes_wheel_center: CompanionSlot,
    /// 表情轮盘左
    #[serde(rename = "EMOTES_WHEEL_LEFT")]
    pub emotes_wheel_left: CompanionSlot,
    /// 表情轮盘下
    #[serde(rename = "EMOTES_WHEEL_LOWER")]
    pub emotes_wheel_lower: CompanionSlot,
    /// 表情轮盘左下
    #[serde(rename = "EMOTES_WHEEL_LOWER_LEFT")]
    pub emotes_wheel_lower_left: CompanionSlot,
    /// 表情轮盘右下
    #[serde(rename = "EMOTES_WHEEL_LOWER_RIGHT")]
    pub emotes_wheel_lower_right: CompanionSlot,
    /// 表情轮盘右
    #[serde(rename = "EMOTES_WHEEL_RIGHT")]
    pub emotes_wheel_right: CompanionSlot,
    /// 表情轮盘上
    #[serde(rename = "EMOTES_WHEEL_UPPER")]
    pub emotes_wheel_upper: CompanionSlot,
    /// 表情轮盘左上
    #[serde(rename = "EMOTES_WHEEL_UPPER_LEFT")]
    pub emotes_wheel_upper_left: CompanionSlot,
    /// 表情轮盘右上
    #[serde(rename = "EMOTES_WHEEL_UPPER_RIGHT")]
    pub emotes_wheel_upper_right: CompanionSlot,
    /// 徽章旗帜槽位
    #[serde(rename = "REGALIA_BANNER_SLOT")]
    pub regalia_banner_slot: CompanionSlot,
    /// 徽章纹章槽位
    #[serde(rename = "REGALIA_CREST_SLOT")]
    pub regalia_crest_slot: CompanionSlot,
    /// 无尽狂潮难度
    #[serde(rename = "STRAWBERRY_DIFFICULTY")]
    pub strawberry_difficulty: CompanionSlot,
    /// 无尽狂潮地图槽位
    #[serde(rename = "STRAWBERRY_MAP_SLOT")]
    pub strawberry_map_slot: CompanionSlot,
    /// TFT地图皮肤槽位
    #[serde(rename = "TFT_MAP_SKIN_SLOT")]
    pub tft_map_skin_slot: CompanionSlot,
    /// TFT剧本槽位
    #[serde(rename = "TFT_PLAYBOOK_SLOT")]
    pub tft_playbook_slot: CompanionSlot,
    ///  tournament奖杯
    #[serde(rename = "TOURNAMENT_TROPHY")]
    pub tournament_trophy: CompanionSlot,
    /// 守卫皮肤槽位
    #[serde(rename = "WARD_SKIN_SLOT")]
    pub ward_skin_slot: CompanionSlot,
}

/// 槽位内容
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompanionSlot {
    /// 内容ID
    pub content_id: String,
    /// 库存类型
    pub inventory_type: String,
    /// 物品ID
    pub item_id: u32,
}

/// 判断是否为无限狂潮模式的英雄
pub fn maybe_pve_champion(id: u32) -> bool {
    id >= 3000 && id < 4000
}
