# League Client Types

## 概述

`league_client` 类型目录包含所有与 League of Legends 客户端 API 交互时使用的数据类型定义。这些类型与 `http_api/league_client/` 目录中的 API 实现相对应。

## 目录结构

该目录包含 25+ 个类型文件，每个文件对应一个功能模块：

- `champ_select.rs` - 英雄选择相关类型
- `champion_mastery.rs` - 英雄熟练度类型
- `chat.rs` - 聊天类型
- `entitlements.rs` - 权限类型
- `event_hub.rs` - 事件中心类型
- `game_data.rs` - 游戏数据类型
- `gameflow.rs` - 游戏流程类型
- `honor.rs` - 荣誉系统类型
- `lobby_team_builder.rs` - 队伍构建器类型
- `lobby.rs` - 大厅类型
- `login.rs` - 登录类型
- `loot.rs` - 战利品类型
- `match_history.rs` - 比赛历史类型
- `matchmaking.rs` - 匹配类型
- `missions.rs` - 任务类型
- `perks.rs` - 符文类型
- `player_notifications.rs` - 玩家通知类型
- `ranked.rs` - 排位赛类型
- `regalia.rs` - 装饰/徽章类型
- `replays.rs` - 回放类型
- `rewards.rs` - 奖励类型
- `store.rs` - 商店类型
- `summoner.rs` - 召唤师类型

## 类型设计特点

### 1. Serde 序列化支持

所有类型都实现了 `Serialize` 和 `Deserialize` trait：

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectSession {
    pub allow_duplicate_picks: bool,
    // ...
}
```

### 2. 命名约定

使用 `#[serde(rename_all = "camelCase")]` 确保 Rust 的 snake_case 字段名与 API 的 camelCase JSON 键名正确映射。

### 3. 默认值支持

所有主要类型都实现了 `Default` trait，允许创建默认实例：

```rust
let session = ChampSelectSession::default();
```

### 4. 可选字段

使用 `Option<T>` 表示可能缺失的字段：

```rust
pub struct SomeResponse {
    pub required_field: String,
    pub optional_field: Option<String>,
}
```

### 5. 嵌套结构

支持复杂的嵌套数据结构，包括：
- 结构体嵌套
- 数组和向量
- HashMap
- 枚举类型
- 扁平化结构（使用 `#[serde(flatten)]`）

## 主要类型示例

### 英雄选择 (champ_select.rs)

```rust
// 会话结构
pub struct ChampSelectSession {
    pub allow_duplicate_picks: bool,
    pub bench_champions: Vec<BenchChampion>,
    pub actions: Vec<Vec<Action>>,
    pub my_team: Vec<ChampSelectTeam>,
    pub their_team: Vec<ChampSelectTeam>,
    // ...
}

// 动作结构
pub struct Action {
    pub actor_cell_id: i32,
    pub champion_id: i32,
    pub completed: bool,
    pub id: i32,
    pub is_ally_action: bool,
    // ...
}
```

### 召唤师 (summoner.rs)

```rust
pub struct Summoner {
    pub account_id: i64,
    pub display_name: String,
    pub internal_name: String,
    pub puuid: String,
    pub summoner_id: i64,
    pub summoner_level: i32,
    // ...
}
```

### 游戏流程 (gameflow.rs)

```rust
// 游戏阶段枚举
pub enum GameflowPhase {
    None,
    Lobby,
    Matchmaking,
    ReadyCheck,
    ChampSelect,
    GameStart,
    InProgress,
    WaitingForStats,
    PreEndOfGame,
    EndOfGame,
}
```

## 类型与 API 的对应关系

每个类型文件都与 `http_api/league_client/` 目录中的对应 API 模块文件相关联：

| 类型文件 | API 文件 | 说明 |
|---------|---------|------|
| `champ_select.rs` | `champ_select.rs` | 英雄选择相关的请求/响应类型 |
| `summoner.rs` | `summoner.rs` | 召唤师信息类型 |
| `gameflow.rs` | `gameflow.rs` | 游戏流程状态类型 |
| ... | ... | ... |

## 使用方式

类型主要在以下场景中使用：

### 1. API 请求参数

```rust
use crate::shared::types::league_client::champ_select::Action;

let action = Action {
    champion_id: 1,
    completed: false,
    // ...
};

api.champ_select.action(action_id, &action).await?;
```

### 2. API 响应解析

```rust
use crate::shared::types::league_client::champ_select::ChampSelectSession;

let session: ChampSelectSession = api.champ_select.get_session().await?;
```

### 3. 类型检查和模式匹配

```rust
match session.phase {
    Some(phase) => println!("Current phase: {:?}", phase),
    None => println!("No phase information"),
}
```

## 类型生成说明

这些类型定义大多是通过 AI 根据 Riot Games 的 API 文档生成的，确保：
- ✅ 与官方 API 响应格式完全匹配
- ✅ 字段类型映射正确
- ✅ 支持所有 API 端点使用的数据结构

## 维护建议

1. **同步更新**: 当 API 发生变化时，需要同步更新对应的类型定义
2. **向后兼容**: 添加新字段时使用 `Option<T>` 保持向后兼容
3. **文档注释**: 为复杂类型添加文档注释说明用途

## 相关文档

- [League Client API 实现](../http_api/league_client/README.md)
- [类型目录总览](../README.md)
