# Types 目录

## 概述

`types` 目录包含应用程序中使用的所有数据类型定义。这些类型主要用于：
- HTTP API 请求和响应的数据结构
- 任务运行器的类型定义
- 各个功能模块的数据模型

## 目录结构

```
types/
├── mod.rs                    # 模块导出文件
├── league_client/            # League Client 相关类型
│   ├── mod.rs
│   ├── champ_select.rs      # 英雄选择类型
│   ├── champion_mastery.rs  # 英雄熟练度类型
│   ├── chat.rs              # 聊天类型
│   ├── entitlements.rs      # 权限类型
│   ├── event_hub.rs        # 事件中心类型
│   ├── game_data.rs        # 游戏数据类型
│   ├── gameflow.rs         # 游戏流程类型
│   ├── honor.rs            # 荣誉类型
│   ├── lobby_team_builder.rs # 队伍构建器类型
│   ├── lobby.rs            # 大厅类型
│   ├── login.rs            # 登录类型
│   ├── loot.rs             # 战利品类型
│   ├── match_history.rs    # 比赛历史类型
│   ├── matchmaking.rs      # 匹配类型
│   ├── missions.rs         # 任务类型
│   ├── perks.rs            # 符文类型
│   ├── player_notifications.rs # 玩家通知类型
│   ├── ranked.rs           # 排位类型
│   ├── regalia.rs          # 装饰类型
│   ├── replays.rs          # 回放类型
│   ├── rewards.rs          # 奖励类型
│   ├── store.rs            # 商店类型
│   ├── summoner.rs         # 召唤师类型
│   └── readme.md           # 类型说明文档
├── task_runner/             # 任务运行器类型
│   ├── mod.rs
│   └── task.rs
└── README.md                # 本文档
```

## 功能说明

### league_client/ 目录

包含所有 League Client API 相关的数据类型定义。每个文件对应一个功能模块，定义该模块使用的请求和响应结构体。

**特点：**
- 使用 `serde` 进行序列化/反序列化
- 支持 `Serialize` 和 `Deserialize` trait
- 使用 `#[serde(rename_all = "camelCase")]` 处理命名约定
- 提供 `Default` trait 实现以便创建默认值
- 大多数类型都是通过 AI 生成的，确保与 API 响应格式一致

**主要类型类别：**

1. **会话类型** (如 `ChampSelectSession`): 表示当前游戏会话的状态
2. **请求类型**: 用于 POST/PUT/PATCH 请求的数据结构
3. **响应类型**: 对应 API 响应的数据结构
4. **枚举类型**: 表示各种状态、选项等

### task_runner/ 目录

包含任务运行器相关的类型定义，用于管理和执行异步任务。

详见：[task_runner/README.md](./task_runner/README.md)

## 类型设计原则

1. **类型安全**: 所有类型都明确定义，避免使用通用 `Value` 类型
2. **可选字段**: 使用 `Option<T>` 表示可能缺失的字段
3. **默认值**: 实现 `Default` trait 以便创建默认实例
4. **序列化兼容**: 确保与 League Client API 的 JSON 格式兼容

## 使用示例

```rust
use crate::shared::types::league_client::champ_select::ChampSelectSession;

// 类型在 HTTP API 中使用
pub async fn get_session(&self) -> Result<ChampSelectSession, HttpError> {
    let url = "/lol-champ-select/v1/session";
    self.client.get(&url).await  // 自动反序列化为 ChampSelectSession
}
```

## 类型生成

大部分类型定义是通过 AI 根据 API 文档生成的，确保：
- 字段名称与 API 响应完全匹配
- 类型映射正确（数字、字符串、布尔值、数组、对象）
- 嵌套结构正确处理

## 相关文档

- [League Client API 文档](../http_api/league_client/README.md)
- [任务运行器文档](./task_runner/README.md)

