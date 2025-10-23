use serde::{Deserialize, Serialize};

/// 继承自 ReplayDownloadProgress 的 ReplayMetadata 接口
/// 由于 Rust 没有继承，直接复用 ReplayDownloadProgress 的字段
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ReplayMetadata(pub ReplayDownloadProgress);

/// 回放下载进度接口（对应 ReplayDownloadProgress）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ReplayDownloadProgress {
    /// 下载进度（checking/found 状态时为大数字，用途未明确）
    pub download_progress: f64, // 用 f64 兼容可能的浮点数进度

    /// 游戏 ID
    pub game_id: u64, // 游戏 ID 通常为大整数，用 u64 更合适

    /// 回放状态（TypeScript 联合类型对应 Rust 枚举）
    pub state: ReplayState,
}

/// 回放状态枚举（对应 TypeScript 的 'checking' | 'found' | ... 联合类型）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReplayState {
    #[default]
    Checking,
    Found,
    Download,
    Downloading,
    Watch,
    Incompatible,
}

/// 回放配置接口（对应 ReplayConfiguration）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ReplayConfiguration {
    /// 游戏版本
    pub game_version: String,

    /// 是否为锦标赛
    pub is_in_tournament: bool,

    /// 是否已登录
    pub is_logged_in: bool,

    /// 是否正在补丁更新
    pub is_patching: bool,

    /// 是否正在进行游戏
    pub is_playing_game: bool,

    /// 是否正在播放回放
    pub is_playing_replay: bool,

    /// 是否启用回放功能
    pub is_replays_enabled: bool,

    /// 是否启用游戏结束回放
    pub is_replays_for_end_of_game_enabled: bool,

    /// 是否启用比赛记录回放
    pub is_replays_for_match_history_enabled: bool,

    /// 最低服务器版本
    pub min_server_version: String,

    /// 回放被视为丢失的分钟数
    pub minutes_until_replay_considered_lost: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RequestMetadata {
    pub game_version: Option<String>,
    pub game_type: Option<String>,
    pub queue_id: Option<u32>,
    pub game_end: Option<u32>,
}
