```rust
use phf::phf_map;

// 服务器名称映射
pub static SGP_SERVER_NAME: phf::Map<&'static str, &'static str> = phf_map! {
    "TENCENT_HN1" => "艾欧尼亚",
    "TENCENT_HN10" => "黑色玫瑰",
    "TENCENT_TJ100" => "联盟四区",
    "TENCENT_TJ101" => "联盟五区",
    "TENCENT_NJ100" => "联盟一区",
    "TENCENT_GZ100" => "联盟二区",
    "TENCENT_CQ100" => "联盟三区",
    "TENCENT_BGP2" => "峡谷之巅",
    "TENCENT_PBE" => "体验服",
    "TW2" => "台湾",
    "SG2" => "新加坡",
    "PH2" => "菲律宾",
    "VN2" => "越南",
    "PBE" => "PBE",
};

// 服务器 ID 到名称映射
pub static SGP_SERVER_ID_TO_NAME: phf::Map<&'static str, &'static str> = phf_map! {
    "HN1" => "艾欧尼亚",
    "HN10" => "黑色玫瑰",
    "TJ100" => "联盟四区",
    "TJ101" => "联盟五区",
    "NJ100" => "联盟一区",
    "GZ100" => "联盟二区",
    "CQ100" => "联盟三区",
    "BGP2" => "峡谷之巅",
    "PBE" => "体验服",
    "TW2" => "台湾",
    "SG2" => "新加坡",
    "PH2" => "菲律宾",
    "VN2" => "越南",
    "" => "暂无",
};

// 英文段位到中文映射
pub static TIER_EN_TO_CN: phf::Map<&'static str, &'static str> = phf_map! {
    "UNRANKED" => "无",
    "IRON" => "坚韧黑铁",
    "BRONZE" => "英勇黄铜",
    "SILVER" => "不屈白银",
    "GOLD" => "荣耀黄金",
    "PLATINUM" => "华贵铂金",
    "EMERALD" => "流光翡翠",
    "DIAMOND" => "璀璨钻石",
    "MASTER" => "超凡大师",
    "GRANDMASTER" => "傲世宗师",
    "CHALLENGER" => "最强王者",
    "" => "无",
};

// 队列类型到中文映射
pub static QUEUE_TYPE_TO_CN: phf::Map<&'static str, &'static str> = phf_map! {
    "RANKED_SOLO_5x5" => "单双排",
    "RANKED_FLEX_SR" => "灵活组排",
    "" => "其他",
};

// 队列 ID 到中文映射
pub static QUEUE_ID_TO_CN: phf::Map<u32, &'static str> = phf_map! {
    420u32 => "单双排",
    430u32 => "匹配",
    440u32 => "灵活排",
    450u32 => "大乱斗",
    490u32 => "匹配",
    890u32 => "人机",
    900u32 => "无限乱斗",
    1700u32 => "斗魂竞技场",
    1900u32 => "无限火力",
    0u32 => "其他",
};

// 腾讯服务器 ID 常量
pub const TENCENT_HN1: &str = "TENCENT_HN1";
pub const TENCENT_HN10: &str = "TENCENT_HN10";
pub const TENCENT_TJ100: &str = "TENCENT_TJ100";
pub const TENCENT_TJ101: &str = "TENCENT_TJ101";
pub const TENCENT_NJ100: &str = "TENCENT_NJ100";
pub const TENCENT_GZ100: &str = "TENCENT_GZ100";
pub const TENCENT_CQ100: &str = "TENCENT_CQ100";
pub const TENCENT_BGP2: &str = "TENCENT_BGP2";
pub const TENCENT_PBE: &str = "TENCENT_PBE";

// 服务器 ID 常量
pub const HN1: &str = "HN1";
pub const HN10: &str = "HN10";
pub const TJ100: &str = "TJ100";
pub const TJ101: &str = "TJ101";
pub const NJ100: &str = "NJ100";
pub const GZ100: &str = "GZ100";
pub const CQ100: &str = "CQ100";
pub const BGP2: &str = "BGP2";
pub const PBE: &str = "PBE";
pub const TW2: &str = "TW2";
pub const SG2: &str = "SG2";
pub const PH2: &str = "PH2";
pub const VN2: &str = "VN2";

// 英文段位常量
pub const UNRANKED: &str = "UNRANKED";
pub const IRON: &str = "IRON";
pub const BRONZE: &str = "BRONZE";
pub const SILVER: &str = "SILVER";
pub const GOLD: &str = "GOLD";
pub const PLATINUM: &str = "PLATINUM";
pub const EMERALD: &str = "EMERALD";
pub const DIAMOND: &str = "DIAMOND";
pub const MASTER: &str = "MASTER";
pub const GRANDMASTER: &str = "GRANDMASTER";
pub const CHALLENGER: &str = "CHALLENGER";

// 排位模式类型常量
pub const RANKED_SOLO_5X5: &str = "RANKED_SOLO_5x5";
pub const RANKED_FLEX_SR: &str = "RANKED_FLEX_SR";

// 排位队列 ID 常量
pub const QUEUE_SOLO_5X5: i32 = 420;
pub const QUEUE_MATCH: i32 = 430;
pub const QUEUE_FLEX: i32 = 440;
pub const QUEUE_ARAM: i32 = 450;
pub const QUEUE_MATCH2: i32 = 490;
pub const QUEUE_OD: i32 = 900;
pub const QUEUE_TFT: i32 = 1700;
pub const QUEUE_URF: i32 = 1900;

// 游戏状态常量
pub const MATCHMAKING: &str = "Matchmaking"; // 正在匹配
pub const CHAMPSELECT: &str = "ChampSelect"; // 英雄选择中
pub const READYCHECK: &str = "ReadyCheck"; // 等待接受状态中
pub const INPROGRESS: &str = "InProgress"; // 游戏进行中
pub const ENDOFGAME: &str = "EndOfGame"; // 游戏结算
pub const LOBBY: &str = "Lobby"; // 房间
pub const GAMESTART: &str = "GameStart"; // 游戏开始
pub const NONE: &str = "None"; // 无
pub const RECONNECT: &str = "Reconnect"; // 重新连接
pub const WAITINGFORSTATS: &str = "WaitingForStats"; // 等待结果
pub const PREENDOFGAME: &str = "PreEndOfGame"; // 结束游戏之前
pub const WATCHINPROGRESS: &str = "WatchInProgress"; // 在观战中
pub const TERMINATEDINERROR: &str = "TerminatedInError"; // 错误终止
```

还有其他的

```yaml
appName: League Akari
default: 默认

yes: 是
no: 否
confirm: 确认
na: N/A

summonerPlaceholder: 召唤师 {{index}}
summoner: 召唤师

lanes:
  all: 无
  ALL: 无
  top: 上单
  jungle: 打野
  mid: 中单
  middle: 中单
  bot: 下路
  bottom: 下路
  support: 辅助
  utility: 辅助
  TOP: 上单
  JUNGLE: 打野
  MID: 中单
  MIDDLE: 中单
  BOTTOM: 下路
  BOT: 下路
  SUPPORT: 辅助
  UTILITY: 辅助

positionAssignmentReason:
  FILL_SECONDARY: 副选补位
  FILL_PRIMARY: 主选补位
  PRIMARY: 主选
  SECONDARY: 副选
  AUTOFILL: 系统补位
  AUTOFILL_SHORT: 补

teams:
  all: 所有
  unknown: 未知
  100: 蓝队
  200: 红队
  our: 我方
  their: 敌方
  our-1: 我方 (蓝队)
  our-2: 我方 (红队)
  their-1: 敌方 (蓝队)
  their-2: 敌方 (红队)

queueTypes:
  RANKED_SOLO_5x5: 单双排位
  RANKED_FLEX_SR: 灵活排位
  NORMAL: 匹配模式
  ARAM_UNRANKED_5x5: 极地大乱斗
  CHERRY: 斗魂竞技场
  URF: 无限火力 / 无限乱斗
  RANKED_FLEX_TT: 灵活排位 3v3
  NORMAL_TFT: 云顶之弈
  RANKED_TFT: 云顶之弈 排位
  RANKED_TFT_TURBO: 云顶之弈 狂暴模式
  RANKED_TFT_DOUBLE_UP: 云顶之弈 双人作战

gameModes:
  CLASSIC: 经典
  ARAM: 极地大乱斗
  URF: 无限火力
  TFT: 云顶之弈

sgpMatchHistoryTags:
  all: 所有模式
  current: 当前模式
  q_420: 单双排位
  q_430: 匹配模式
  q_440: 灵活排位
  q_450: 极地大乱斗
  q_480: 快速模式
  q_490: 快速匹配
  q_900: 无限乱斗
  q_1700: 斗魂竞技场
  q_1900: 无限火力
  q_2300: 神木之门

tiers:
  UNRANKED: 未定级
  IRON: 坚韧黑铁
  BRONZE: 英勇黄铜
  SILVER: 不屈白银
  GOLD: 荣耀黄金
  PLATINUM: 华贵铂金
  EMERALD: 流光翡翠
  DIAMOND: 璀璨钻石
  MASTER: 超凡大师
  GRANDMASTER: 傲世宗师
  CHALLENGER: 最强王者

shortTiers:
  UNRANKED: 无
  IRON: 黑铁
  BRONZE: 黄铜
  SILVER: 白银
  GOLD: 黄金
  PLATINUM: 铂金
  EMERALD: 翡翠
  DIAMOND: 钻石
  MASTER: 大师
  GRANDMASTER: 宗师
  CHALLENGER: 王者

sgpServers:
  TENCENT_HN10: 黑色玫瑰
  TENCENT_HN1: 艾欧尼亚
  TENCENT_NJ100: 联盟一区
  TENCENT_GZ100: 联盟二区
  TENCENT_CQ100: 联盟三区
  TENCENT_TJ100: 联盟四区
  TENCENT_TJ101: 联盟五区
  TENCENT_BGP2: 峡谷之巅
  TENCENT_PBE: 体验服
  TW2: 台湾
  SG2: 新加坡
  PH2: 菲律宾
  VN2: 越南
  PBE: PBE
  EUN: 欧洲东北
  EUW: 欧洲西
  KR: 韩国
  BR1: 巴西
  LA1: 拉丁美洲
  LA2: 拉丁美洲
  OC1: 大洋洲
  TR1: 土耳其
  RU: 俄罗斯
  JP1: 日本
```

# Constants 目录

## 概述

`constants` 目录包含应用程序中使用的全局常量定义。这些常量在整个应用中被引用，用于配置 URL、标识符等固定值。

## 目录结构

```
constants/
├── mod.rs          # 模块导出文件
├── common.rs       # 通用常量定义
└── README.md       # 本文档
```

## 功能说明

### common.rs

包含应用程序中使用的通用常量：

- **LEAGUE_AKARI_GITHUB**: LeagueAkari 项目的 GitHub 仓库地址
  - 类型: `&str`
  - 值: `"https://github.com/LeagueAkari/LeagueAkari"`

- **LEAGUE_AKARI_CHECK_ANNOUNCEMENT_URL**: 用于检查公告的 API 端点
  - 类型: `&str`
  - 值: `"https://api.github.com/repos/LeagueAkari/LeagueAkari/contents/docs/announcement.md"`

- **EMPTY_PUUID**: 空玩家唯一标识符的占位符
  - 类型: `&str`
  - 值: `"00000000-0000-0000-0000-000000000000"`
  - 用途: 用于表示未设置或无效的玩家 UUID

## 使用示例

```rust
use crate::shared::constants::common::*;

// 访问 GitHub 仓库地址
let repo_url = LEAGUE_AKARI_GITHUB;

// 检查公告
let announcement_url = LEAGUE_AKARI_CHECK_ANNOUNCEMENT_URL;

// 使用空 UUID 占位符
let empty_id = EMPTY_PUUID;
```

## 设计说明

- 所有常量都使用 `pub const` 声明，确保在编译时确定值
- 常量名称使用大写字母和下划线，遵循 Rust 命名约定
- 这些常量主要用于配置外部服务 URL 和默认值

