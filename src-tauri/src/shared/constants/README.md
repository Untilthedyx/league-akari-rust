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

