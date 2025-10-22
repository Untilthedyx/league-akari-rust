use serde::{Deserialize, Serialize};

/// 游戏流程阶段类型
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GameflowPhase {
    #[serde(rename = "Matchmaking")]
    Matchmaking, // 正在匹配
    #[serde(rename = "ChampSelect")]
    ChampSelect, // 英雄选择中
    #[serde(rename = "ReadyCheck")]
    ReadyCheck, // 等待接受状态中
    #[serde(rename = "InProgress")]
    InProgress, // 游戏进行中
    #[serde(rename = "EndOfGame")]
    EndOfGame, // 游戏结算
    #[serde(rename = "Lobby")]
    Lobby, // 房间
    #[serde(rename = "GameStart")]
    GameStart, // 游戏开始
    #[default]
    #[serde(rename = "None")]
    None, // 无
    #[serde(rename = "Reconnect")]
    Reconnect, // 重新连接
    #[serde(rename = "WaitingForStats")]
    WaitingForStats, // 等待结果
    #[serde(rename = "PreEndOfGame")]
    PreEndOfGame, // 结束游戏之前
    #[serde(rename = "WatchInProgress")]
    WatchInProgress, // 在观战中
    #[serde(rename = "TerminatedInError")]
    TerminatedInError, // 错误终止
}

/// 游戏流程会话信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameflowSession {
    /// 游戏客户端信息
    pub game_client: GameClient,
    /// 游戏数据信息
    pub game_data: GameData,
    /// 游戏闪避信息
    pub game_dodge: GameDodge,
    /// 地图信息
    pub map: Map,
    /// 当前游戏阶段
    pub phase: GameflowPhase,
}

/// 地图信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    /// 地图资源
    pub assets: Assets,
    /// 分类内容包
    pub categorized_content_bundles: CategorizedContentBundles,
    /// 地图描述
    pub description: String,
    /// 游戏模式
    pub game_mode: String,
    /// 游戏模式名称
    pub game_mode_name: String,
    /// 游戏模式短名称
    pub game_mode_short_name: String,
    /// 游戏变异器
    pub game_mutator: String,
    /// 地图ID
    pub id: u32,
    /// 是否为随机游戏模式
    pub is_rgm: bool,
    /// 地图字符串ID
    pub map_string_id: String,
    /// 地图名称
    pub name: String,
    /// 每个位置禁用的召唤师技能
    pub per_position_disallowed_summoner_spells: CategorizedContentBundles,
    /// 每个位置推荐的召唤师技能
    pub per_position_required_summoner_spells: CategorizedContentBundles,
    /// 平台ID
    pub platform_id: String,
    /// 平台名称
    pub platform_name: String,
    /// 地图属性
    pub properties: Properties,
}

/// 地图属性
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    /// 是否禁用符文天赋
    pub suppress_runes_masteries_perks: bool,
}

/// 分类内容包（暂为空结构，根据实际数据扩展）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CategorizedContentBundles;

/// 地图资源集合
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    /// 英雄选择背景音效
    #[serde(rename = "champ-select-background-sound")]
    pub champ_select_background_sound: String,
    /// 英雄选择弹出框背景
    #[serde(rename = "champ-select-flyout-background")]
    pub champ_select_flyout_background: String,
    /// 英雄选择准备阶段 intro
    #[serde(rename = "champ-select-planning-intro")]
    pub champ_select_planning_intro: String,
    /// 游戏选择图标（激活状态）
    #[serde(rename = "game-select-icon-active")]
    pub game_select_icon_active: String,
    /// 游戏选择图标（激活状态视频）
    #[serde(rename = "game-select-icon-active-video")]
    pub game_select_icon_active_video: String,
    /// 游戏选择图标（默认状态）
    #[serde(rename = "game-select-icon-default")]
    pub game_select_icon_default: String,
    /// 游戏选择图标（禁用状态）
    #[serde(rename = "game-select-icon-disabled")]
    pub game_select_icon_disabled: String,
    /// 游戏选择图标（悬停状态）
    #[serde(rename = "game-select-icon-hover")]
    pub game_select_icon_hover: String,
    /// 游戏选择图标（intro视频）
    #[serde(rename = "game-select-icon-intro-video")]
    pub game_select_icon_intro_video: String,
    /// 游戏流程背景
    #[serde(rename = "gameflow-background")]
    pub gameflow_background: String,
    /// 游戏流程深色背景
    #[serde(rename = "gameflow-background-dark")]
    pub gameflow_background_dark: String,
    /// 游戏选择按钮悬停音效
    #[serde(rename = "gameselect-button-hover-sound")]
    pub gameselect_button_hover_sound: String,
    /// 失败图标
    #[serde(rename = "icon-defeat")]
    pub icon_defeat: String,
    /// 失败图标v2
    #[serde(rename = "icon-defeat-v2")]
    pub icon_defeat_v2: String,
    /// 失败图标视频
    #[serde(rename = "icon-defeat-video")]
    pub icon_defeat_video: String,
    /// 空图标
    #[serde(rename = "icon-empty")]
    pub icon_empty: String,
    /// 悬停图标
    #[serde(rename = "icon-hover")]
    pub icon_hover: String,
    /// 离开者图标
    #[serde(rename = "icon-leaver")]
    pub icon_leaver: String,
    /// 离开者图标v2
    #[serde(rename = "icon-leaver-v2")]
    pub icon_leaver_v2: String,
    /// 失败宽恕图标v2
    #[serde(rename = "icon-loss-forgiven-v2")]
    pub icon_loss_forgiven_v2: String,
    /// 图标v2
    #[serde(rename = "icon-v2")]
    pub icon_v2: String,
    /// 胜利图标
    #[serde(rename = "icon-victory")]
    pub icon_victory: String,
    /// 胜利图标视频
    #[serde(rename = "icon-victory-video")]
    pub icon_victory_video: String,
    /// 地图北侧
    #[serde(rename = "map-north")]
    pub map_north: String,
    /// 地图南侧
    #[serde(rename = "map-south")]
    pub map_south: String,
    /// 排队中循环音效
    #[serde(rename = "music-inqueue-loop-sound")]
    pub music_inqueue_loop_sound: String,
    /// 队伍背景
    #[serde(rename = "parties-background")]
    pub parties_background: String,
    /// 赛后氛围循环音效
    #[serde(rename = "postgame-ambience-loop-sound")]
    pub postgame_ambience_loop_sound: String,
    /// 准备确认背景
    #[serde(rename = "ready-check-background")]
    pub ready_check_background: String,
    /// 准备确认背景音效
    #[serde(rename = "ready-check-background-sound")]
    pub ready_check_background_sound: String,
    /// 赛前氛围循环音效
    #[serde(rename = "sfx-ambience-pregame-loop-sound")]
    pub sfx_ambience_pregame_loop_sound: String,
    /// 社交离开者图标
    #[serde(rename = "social-icon-leaver")]
    pub social_icon_leaver: String,
    /// 社交胜利图标
    #[serde(rename = "social-icon-victory")]
    pub social_icon_victory: String,
}

/// 游戏闪避信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameDodge {
    /// 闪避者ID列表
    pub dodge_ids: Vec<serde_json::Value>,
    /// 闪避阶段
    pub phase: String,
    /// 闪避状态
    pub state: String,
}

/// 游戏数据信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    /// 游戏ID
    pub game_id: u64,
    /// 游戏名称
    pub game_name: String,
    /// 是否为自定义游戏
    pub is_custom_game: bool,
    /// 游戏密码（如果有）
    pub password: String,
    /// 玩家英雄选择信息
    pub player_champion_selections: Vec<PlayerChampionSelection>,
    /// 队列信息
    pub queue: Queue,
    /// 是否允许观战
    pub spectators_allowed: bool,
    /// 一队玩家
    pub team_one: Vec<TeamPlayer>,
    /// 二队玩家
    pub team_two: Vec<TeamPlayer>,
}

/// 玩家英雄选择信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerChampionSelection {
    /// 英雄ID
    pub champion_id: u32,
    /// 选中的皮肤索引
    pub selected_skin_index: u32,
    /// 召唤师技能1 ID
    pub spell1_id: u32,
    /// 召唤师技能2 ID
    pub spell2_id: u32,
    /// 召唤师ID（旧ID系统）
    pub summoner_id: u64,
    /// 召唤师PUUID（新ID系统）
    pub puuid: String,
    /// 召唤师内部名称
    pub summoner_internal_name: String,
}

/// 队伍玩家信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TeamPlayer {
    /// 英雄ID
    pub champion_id: u32,
    /// 账户ID
    pub account_id: u64,
    /// 调整标记
    pub adjustment_flags: u32,
    /// 人机难度（如果是人机）
    pub bot_difficulty: String,
    /// 客户端是否同步
    pub client_in_synch: bool,
    /// 游戏自定义信息
    pub game_customization: GameCustomization,
    /// 索引位置
    pub index: u32,
    /// 上次选择的皮肤索引
    pub last_selected_skin_index: u32,
    /// 本地化（可选）
    pub locale: Option<serde_json::Value>,
    /// 是否为次要玩家
    pub minor: bool,
    /// 原始账户编号
    pub original_account_number: u64,
    /// 原始平台ID（可选）
    pub original_platform_id: Option<serde_json::Value>,
    /// 伙伴ID
    pub partner_id: String,
    /// 选择模式
    pub pick_mode: u32,
    /// 选择轮次
    pub pick_turn: u32,
    /// 头像ID
    pub profile_icon_id: u32,
    /// 召唤师PUUID
    pub puuid: String,
    /// 队列评级
    pub queue_rating: u32,
    /// 是否为排名队伍访客
    pub ranked_team_guest: bool,
    /// 选中的位置
    pub selected_position: String,
    /// 选中的角色
    pub selected_role: String,
    /// 召唤师ID
    pub summoner_id: u64,
    /// 召唤师内部名称
    pub summoner_internal_name: String,
    /// 召唤师名称
    pub summoner_name: String,
    /// 是否为队伍所有者
    pub team_owner: bool,
    /// 队伍参与者ID
    pub team_participant_id: u32,
    /// 队伍评级
    pub team_rating: u32,
    /// 加入队列时间
    pub time_added_to_queue: u64,
    /// 英雄选择开始时间
    pub time_champion_select_start: u64,
    /// 游戏创建时间
    pub time_game_created: u64,
    /// 匹配开始时间
    pub time_matchmaking_start: u64,
    /// 投票者评级
    pub voter_rating: u32,
}

/// 游戏自定义信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameCustomization {
    /// 伙伴
    pub companions: String,
    /// 金色铲子俱乐部
    pub golden_spatula_club: String,
    /// 徽章
    pub regalia: String,
    /// 挑战
    pub challenges: String,
    /// 已拥有英雄
    pub champion_owned: String,
    /// 符文
    pub perks: String,
    /// 排名
    pub ranked: String,
    /// 状态石
    pub statstones: String,
    /// 召唤师表情
    pub summoner_emotes: String,
    /// 召唤师奖杯
    pub summoner_trophy: String,
    /// 经典皮肤
    pub vintage_skin: String,
}

/// 队列信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Queue {
    /// 允许的 premade 队伍规模
    pub allowable_premade_sizes: Vec<serde_json::Value>,
    /// 是否允许免费英雄
    pub are_free_champions_allowed: bool,
    /// 资源变异器
    pub asset_mutator: String,
    /// 分类
    pub category: String,
    /// 游玩所需英雄数量
    pub champions_required_to_play: u32,
    /// 描述
    pub description: String,
    /// 详细描述
    pub detailed_description: String,
    /// 游戏模式
    pub game_mode: String,
    /// 游戏类型配置
    pub game_type_config: GameTypeConfig,
    /// 队列ID
    pub id: u32,
    /// 是否为排名队列
    pub is_ranked: bool,
    /// 是否由队伍构建器管理
    pub is_team_builder_managed: bool,
    /// 上次关闭时间
    pub last_toggled_off_time: u64,
    /// 上次开启时间
    pub last_toggled_on_time: u64,
    /// 地图ID
    pub map_id: u32,
    /// 最大参与者列表规模
    pub maximum_participant_list_size: u32,
    /// 最低等级
    pub min_level: u32,
    /// 最小参与者列表规模
    pub minimum_participant_list_size: u32,
    /// 名称
    pub name: String,
    /// 每队玩家数量
    pub num_players_per_team: u32,
    /// 队列可用性
    pub queue_availability: String,
    /// 队列奖励
    pub queue_rewards: QueueRewards,
    /// 是否允许中途退出游戏
    pub removal_from_game_allowed: bool,
    /// 中途退出游戏延迟（分钟）
    pub removal_from_game_delay_minutes: u32,
    /// 短名称
    pub short_name: String,
    /// 是否显示位置选择器
    pub show_position_selector: bool,
    /// 是否启用观战
    pub spectator_enabled: bool,
    /// 类型
    pub r#type: String,
}

/// 队列奖励
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueueRewards {
    /// 是否启用英雄点数
    pub is_champion_points_enabled: bool,
    /// 是否启用IP（已废弃）
    pub is_ip_enabled: bool,
    /// 是否启用经验值
    pub is_xp_enabled: bool,
    /// 队伍规模IP奖励
    pub party_size_ip_rewards: Vec<serde_json::Value>,
}

/// 游戏类型配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameTypeConfig {
    /// 高级学习任务
    pub advanced_learning_quests: bool,
    /// 是否允许交换
    pub allow_trades: bool,
    /// 禁用模式
    pub ban_mode: String,
    /// 禁用计时器持续时间（秒）
    pub ban_timer_duration: u32,
    /// 战斗增强
    pub battle_boost: bool,
    /// 跨队英雄池
    pub cross_team_champion_pool: bool,
    /// 死亡竞赛模式
    pub death_match: bool,
    /// 不移除
    pub do_not_remove: bool,
    /// 重复选择
    pub duplicate_pick: bool,
    /// 专属选择
    pub exclusive_pick: bool,
    /// ID
    pub id: u32,
    /// 学习任务
    pub learning_quests: bool,
    /// 主要选择计时器持续时间（秒）
    pub main_pick_timer_duration: u32,
    /// 最大允许禁用数量
    pub max_allowable_bans: u32,
    /// 名称
    pub name: String,
    /// 新手合作模式
    pub onboard_coop_beginner: bool,
    /// 选择模式
    pub pick_mode: String,
    /// 后选择计时器持续时间（秒）
    pub post_pick_timer_duration: u32,
    /// 重选
    pub reroll: bool,
    /// 队伍英雄池
    pub team_champion_pool: bool,
}

/// 游戏客户端信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameClient {
    /// 观察者服务器IP
    pub observer_server_ip: String,
    /// 观察者服务器端口
    pub observer_server_port: u32,
    /// 客户端是否正在运行
    pub running: bool,
    /// 服务器IP
    pub server_ip: String,
    /// 服务器端口
    pub server_port: u32,
    /// 客户端是否可见
    pub visible: bool,
}
