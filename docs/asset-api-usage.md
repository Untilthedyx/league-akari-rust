# 图片资源 API 使用指南

## 概述

本文档介绍如何在前端使用 Tauri 命令获取 League Client 的图片资源（头像、英雄图标、物品图标、召唤师技能图标等）。

## 后端命令

已注册的 Tauri 命令：
- `get_profile_icon` - 获取召唤师头像图标
- `get_champion_icon` - 获取英雄头像图标
- `get_item_icon` - 获取物品图标
- `get_spell_icon` - 获取召唤师技能图标

## 前端 API

所有 API 函数都位于 `src/lib/api/asset.ts`：

```typescript
import { getProfileIcon, getChampionIcon, getItemIcon, getSpellIcon, useImage } from "@/lib/api/asset";
```

## 使用方法

### 方法 1: 直接调用（异步函数）

```tsx
import { useState, useEffect } from "react";
import { getProfileIcon } from "@/lib/api/asset";

function AvatarExample() {
  const [avatarUrl, setAvatarUrl] = useState<string>("");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadAvatar() {
      try {
        const url = await getProfileIcon(3460); // 3460 是头像 ID
        setAvatarUrl(url);
      } catch (error) {
        console.error("加载头像失败:", error);
      } finally {
        setIsLoading(false);
      }
    }
    loadAvatar();
  }, []);

  if (isLoading) return <div>加载中...</div>;

  return <img src={avatarUrl} alt="头像" />;
}
```

### 方法 2: 使用 React Hook（推荐）

```tsx
import { useImage } from "@/lib/api/asset";
import { getProfileIcon } from "@/lib/api/asset";

function AvatarExample() {
  const { data: avatarUrl, isLoading, error } = useImage(() => getProfileIcon(3460));

  if (isLoading) return <div>加载中...</div>;
  if (error) return <div>加载失败: {error.message}</div>;

  return <img src={avatarUrl || ''} alt="头像" />;
}
```

### 方法 3: 在现有组件中使用

```tsx
import { useState, useEffect } from "react";
import { getProfileIcon } from "@/lib/api/asset";

interface PlayerAvatarProps {
  iconId: number;
  className?: string;
}

function PlayerAvatar({ iconId, className }: PlayerAvatarProps) {
  const [avatarUrl, setAvatarUrl] = useState<string>("");
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    async function loadAvatar() {
      try {
        const url = await getProfileIcon(iconId);
        setAvatarUrl(url);
      } catch (error) {
        console.error("加载头像失败:", error);
        // 设置默认占位符
        setAvatarUrl("https://via.placeholder.com/96/cccccc/666666?text=头像");
      } finally {
        setIsLoading(false);
      }
    }
    loadAvatar();
  }, [iconId]);

  return (
    <img
      src={avatarUrl}
      alt="头像"
      className={className}
      onError={(e) => {
        (e.target as HTMLImageElement).src =
          "https://via.placeholder.com/96/cccccc/666666?text=头像";
      }}
    />
  );
}
```

## 完整示例：更新 PlayerInfo 组件

假设你想从 League Client API 获取真实的头像，可以这样修改：

```tsx
import { useState, useEffect } from "react";
import { getProfileIcon } from "@/lib/api/asset";
import { invoke } from "@tauri-apps/api/core";

// 假设你有一个获取召唤师信息的命令
async function getCurrentSummoner() {
  return await invoke("get_current_summoner");
}

function PlayerInfo() {
  const [avatarUrl, setAvatarUrl] = useState<string>("");
  const [iconId, setIconId] = useState<number | null>(null);

  useEffect(() => {
    async function loadPlayerData() {
      try {
        // 1. 获取召唤师信息（包含 iconId）
        const summoner = await getCurrentSummoner();
        setIconId(summoner.profileIconId);

        // 2. 获取头像图片
        if (summoner.profileIconId) {
          const url = await getProfileIcon(summoner.profileIconId);
          setAvatarUrl(url);
        }
      } catch (error) {
        console.error("加载玩家信息失败:", error);
      }
    }
    loadPlayerData();
  }, []);

  return (
    <div>
      {avatarUrl && (
        <img
          src={avatarUrl}
          alt="头像"
          className="w-24 h-24 rounded-lg"
          onError={(e) => {
            (e.target as HTMLImageElement).src =
              "https://via.placeholder.com/96/cccccc/666666?text=头像";
          }}
        />
      )}
    </div>
  );
}
```

## 其他资源类型示例

### 获取英雄头像

```tsx
import { getChampionIcon } from "@/lib/api/asset";

const championIcon = await getChampionIcon(1); // 1 是英雄 ID
```

### 获取物品图标

```tsx
import { getItemIcon } from "@/lib/api/asset";

const itemIcon = await getItemIcon(1001); // 1001 是物品 ID
```

### 获取召唤师技能图标

```tsx
import { getSpellIcon } from "@/lib/api/asset";

const flashIcon = await getSpellIcon("SummonerFlash");
const healIcon = await getSpellIcon("SummonerHeal");
```

## 注意事项

1. **Base64 格式**: 所有图片都返回 Base64 编码的 Data URL，可以直接用于 `<img src="...">`
2. **错误处理**: 建议始终添加错误处理和占位符图片
3. **缓存**: 如果需要在多个组件中使用同一张图片，考虑使用缓存机制
4. **性能**: Base64 编码会增加约 33% 的数据大小，适合小图标（< 50KB）

## 故障排除

### 错误: "HTTP client not initialized"
- 确保在调用图片 API 之前，已经初始化了 HTTP 客户端
- 检查 League Client 是否正在运行

### 图片加载失败
- 检查图标 ID 是否有效
- 检查网络连接
- 使用 `onError` 处理程序显示占位符

### 类型错误
- 确保安装了 `@tauri-apps/api` 包
- 检查 TypeScript 类型定义是否正确

