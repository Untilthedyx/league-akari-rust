# League Client HTTP API

## 概述

`league_client` 目录包含完整的 League of Legends 客户端本地 HTTP API 实现。该模块提供了与游戏客户端所有功能模块交互的 HTTP 接口封装。

## 目录结构

该目录包含 30+ 个 API 模块文件，每个文件对应一个游戏功能模块：

- `champ_select.rs` - 英雄选择相关 API
- `champion_mastery.rs` - 英雄熟练度 API
- `chat.rs` - 聊天功能 API
- `end_of_game.rs` - 游戏结束相关 API
- `entitlements.rs` - 权限/授权 API
- `event_hub.rs` - 事件中心 API
- `game_data.rs` - 游戏数据 API
- `gameflow.rs` - 游戏流程状态 API
- `honor.rs` - 荣誉系统 API
- `league_session.rs` - 会话管理 API
- `loadouts.rs` - 装备配置 API
- `lobby.rs` - 大厅功能 API
- `lobby_team_builder.rs` - 队伍构建器 API
- `login.rs` - 登录相关 API
- `loot.rs` - 战利品/奖励 API
- `match_history.rs` - 比赛历史 API
- `matchmaking.rs` - 匹配系统 API
- `missions.rs` - 任务系统 API
- `perks.rs` - 符文配置 API
- `player_notifications.rs` - 玩家通知 API
- `pre_end_of_game.rs` - 游戏结束前 API
- `process_control.rs` - 进程控制 API
- `ranked.rs` - 排位赛相关 API
- `regalia.rs` - 装饰/徽章 API
- `remedy.rs` - 修复/恢复 API
- `replays.rs` - 回放系统 API
- `reward_track.rs` - 奖励追踪 API
- `rewards.rs` - 奖励系统 API
- `riotclient.rs` - Riot 客户端 API
- `spectator.rs` - 观战系统 API
- `store.rs` - 商店功能 API
- `summoner.rs` - 召唤师信息 API

## 核心结构

### LeagueClientHttpApiAxiosHelper

这是主要的 API 助手结构体，聚合了所有功能模块的 API 实例：

```rust
pub struct LeagueClientHttpApiAxiosHelper {
    pub champ_select: ChampSelectHttpApi,
    pub champion_mastery: ChampionMasteryHttpApi,
    pub chat: ChatHttpApi,
    // ... 所有其他模块
}
```

**初始化方式：**
```rust
let api = LeagueClientHttpApiAxiosHelper::new(port, token, accept_invalid_certs);
```

## API 模块设计模式

每个 API 模块都遵循相同的设计模式：

1. **结构体定义**: 每个模块都有一个对应的结构体（如 `ChampSelectHttpApi`）
2. **客户端注入**: 通过构造函数接收 `HttpClient` 实例
3. **方法封装**: 每个方法对应一个特定的 API 端点
4. **类型安全**: 使用定义在 `types/league_client/` 中的类型

**示例结构：**
```rust
pub struct ChampSelectHttpApi {
    client: HttpClient,
}

impl ChampSelectHttpApi {
    pub fn new(client: HttpClient) -> Self {
        Self { client }
    }

    pub async fn get_session(&self) -> Result<ChampSelectSession, HttpError> {
        let url = "/lol-champ-select/v1/session";
        self.client.get(&url).await
    }
}
```

## 主要功能模块

### 英雄选择 (champ_select)

提供英雄选择阶段的完整控制：
- 获取选择会话信息
- 执行选择/禁用操作
- 交换英雄
- 获取网格英雄列表
- 完成动作等

### 游戏流程 (gameflow)

监控和管理游戏状态流程：
- 获取当前游戏阶段
- 监听阶段变化

### 召唤师 (summoner)

获取和管理召唤师信息：
- 当前召唤师信息
- 召唤师统计等

### 大厅 (lobby)

管理游戏大厅：
- 创建/加入/离开大厅
- 大厅成员管理
- 游戏模式设置

### 匹配历史 (match_history)

访问比赛历史记录：
- 获取比赛列表
- 比赛详情查询

### 聊天 (chat)

聊天功能集成：
- 发送消息
- 获取聊天状态等

## 使用示例

```rust
use crate::shared::http_api::league_client::LeagueClientHttpApiAxiosHelper;

// 初始化 API 助手
let api = LeagueClientHttpApiAxiosHelper::new(port, token, true);

// 获取英雄选择会话
let session = api.champ_select.get_session().await?;

// 获取当前游戏阶段
let phase = api.gameflow.get_phase().await?;

// 获取当前召唤师信息
let summoner = api.summoner.get_current_summoner().await?;
```

## 类型定义

所有 API 使用的类型定义位于：
- `../../types/league_client/` 目录
- 每个模块都有对应的类型文件（如 `champ_select.rs` 对应 `types/league_client/champ_select.rs`）

## 注意事项

1. **端口和令牌**: 所有 API 都需要有效的端口号和认证令牌
2. **证书验证**: 本地 API 使用自签名证书，需要设置 `accept_invalid_certs: true`
3. **异步操作**: 所有 API 方法都是异步的，需要使用 `.await`
4. **错误处理**: 所有方法返回 `Result<T, HttpError>`，需要适当处理错误

## 相关文档

- [HTTP 客户端文档](../README.md#httprs---http-客户端封装)
- [类型定义文档](../../types/league_client/README.md)
