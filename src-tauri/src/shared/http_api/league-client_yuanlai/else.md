# League Client Yuanlai (原始版本)

## 概述

`league-client_yuanlai` 目录包含原始版本的 League Client API 实现。这是一个较早期的实现版本，可能用于：
- 代码参考和兼容性
- 保持旧版 API 接口的可用性
- 作为迁移到新版本的过渡方案

**注意**: "yuanlai" 是中文"原来"的拼音，表示这是原始/旧版本的实现。

## 目录结构

```
league-client_yuanlai/
├── mod.rs              # 模块导出
├── https.rs            # HTTPS 客户端实现
├── constant.rs         # 常量定义
├── api/                # API 实现目录
│   ├── mod.rs          # API 模块导出
│   ├── summoner.rs     # 召唤师 API
│   ├── session.rs      # 会话 API
│   ├── rank.rs         # 排位 API
│   ├── phase.rs        # 阶段 API
│   ├── model.rs        # 数据模型
│   ├── game_detail.rs  # 游戏详情 API
│   ├── match_history.rs # 比赛历史 API
│   ├── lobby.rs        # 大厅 API
│   ├── champion_select.rs # 英雄选择 API
│   ├── asset.rs        # 资源 API
│   └── test.rs         # 测试相关
└── README.md           # 本文档
```

## 核心模块

### https.rs

提供 HTTPS 客户端封装，用于与本地 League Client API 通信。

**主要功能：**
- 处理自签名证书
- 基本认证（riot:token）
- HTTP 请求封装

### constant.rs

定义该模块使用的常量，如 API 端点路径等。

### api/ 目录

包含各个功能模块的 API 实现：

#### summoner.rs
获取召唤师信息：
- 根据名字获取召唤师
- 根据 PUUID 获取召唤师
- 获取当前召唤师信息

**实现说明：**
- 使用 `LazyLock` 进行延迟初始化
- `LazyLock` 相当于 `OnceLock<Mutex>`，专门为延迟初始化设计
- 初始化完成后，不需要每次都访问锁，性能更好

#### session.rs
会话管理相关功能。

#### rank.rs
排位赛相关功能。

#### phase.rs
游戏阶段/流程管理。

#### lobby.rs
游戏大厅功能。

#### champion_select.rs
英雄选择功能。

#### match_history.rs
比赛历史查询。

#### game_detail.rs
游戏详情信息。

#### asset.rs
游戏资源/素材相关。

## 与新版本的区别

与 `league_client/` 目录的新版本相比，该目录的特点：

1. **不同的 HTTP 客户端实现**: 使用自定义的 `https.rs` 而非统一的 `HttpClient`
2. **更简单的 API 结构**: 可能缺少某些新版本的功能
3. **不同的类型定义**: 使用自己的数据模型

## 使用建议

如果可能，建议优先使用 `league_client/` 目录中的新版本实现，因为：
- 更统一的错误处理
- 更好的类型定义
- 更完整的 API 覆盖
- 统一的客户端管理

## 相关文档

- [新版本 League Client API](../league_client/README.md)
- [类型定义](../../types/league_client/README.md)
