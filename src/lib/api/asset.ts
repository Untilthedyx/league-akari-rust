/**
 * 图片资源 API - 前端调用示例
 *
 * 本文件展示了如何从 Tauri 后端获取图片资源并显示在前端
 */

import { invoke } from "@tauri-apps/api/core";
import { useState, useEffect, useMemo } from "react";

/**
 * 获取召唤师头像图标（Base64 编码）
 *
 * @param iconId 头像图标 ID（字符串）
 * @returns Base64 编码的图片数据 URL，可直接用于 `<img src="...">`
 *
 * @example
 * ```tsx
 * const avatarUrl = await getProfileIcon("3460");
 * <img src={avatarUrl} alt="头像" />
 * ```
 */
export async function getProfileIcon(iconId: string): Promise<string> {
  try {
    const base64Url = await invoke<string>("get_profile_icon", {
      iconId: Number(iconId),
    });
    return base64Url;
  } catch (error) {
    console.error("获取头像失败:", error);
    throw error;
  }
}

/**
 * 获取英雄头像图标（Base64 编码）
 *
 * @param championId 英雄 ID（字符串）
 * @returns Base64 编码的图片数据 URL
 *
 * @example
 * ```tsx
 * const championIcon = await getChampionIcon("1");
 * <img src={championIcon} alt="英雄头像" />
 * ```
 */
export async function getChampionIcon(championId: string): Promise<string> {
  try {
    const base64Url = await invoke<string>("get_champion_icon", {
      championId: Number(championId),
    });
    return base64Url;
  } catch (error) {
    console.error("获取英雄头像失败:", error);
    throw error;
  }
}

/**
 * 获取物品图标（Base64 编码）
 *
 * @param itemId 物品 ID（字符串）
 * @returns Base64 编码的图片数据 URL
 *
 * @example
 * ```tsx
 * const itemIcon = await getItemIcon("1001");
 * <img src={itemIcon} alt="物品图标" />
 * ```
 */
export async function getItemIcon(itemId: string): Promise<string> {
  try {
    const base64Url = await invoke<string>("get_item_icon", {
      itemId: Number(itemId),
    });
    return base64Url;
  } catch (error) {
    console.error("获取物品图标失败:", error);
    throw error;
  }
}

/**
 * 获取召唤师技能图标（Base64 编码）
 *
 * @param spellId 召唤师技能 ID（如 "SummonerFlash"）
 * @returns Base64 编码的图片数据 URL
 *
 * @example
 * ```tsx
 * const spellIcon = await getSpellIcon("SummonerFlash");
 * <img src={spellIcon} alt="召唤师技能" />
 * ```
 */
export async function getSpellIcon(spellId: string): Promise<string> {
  try {
    const base64Url = await invoke<string>("get_spell_icon", { spellId });
    return base64Url;
  } catch (error) {
    console.error("获取召唤师技能图标失败:", error);
    throw error;
  }
}

/**
 * 获取符文图标（Base64 编码）
 *
 * @param perkId 符文 ID
 * @returns Base64 编码的图片数据 URL
 *
 * @example
 * ```tsx
 * const perkIcon = await getPerkIcon("1");
 * <img src={perkIcon} alt="符文图标" />
 * ```
 */
export async function getPerkIcon(perkId: string): Promise<string> {
  try {
    const base64Url = await invoke<string>("get_perk_icon", { perkId });
    return base64Url;
  } catch (error) {
    console.error("获取符文图标失败:", error);
    throw error;
  }
}

/**
 * 批量获取图片（并发加载）
 *
 * @param iconIds 图片 ID 数组
 * @param getFn 获取单张图片的函数
 * @returns Promise<Map<ID, URL>> 返回 ID 到 URL 的映射
 *
 * @example
 * ```tsx
 * const images = await batchGetImages(["3460", "3461", "3462"], getProfileIcon);
 * // images.get("3460") => "data:image/jpeg;base64,..."
 * ```
 */
export async function batchGetImages<T extends string>(
  iconIds: T[],
  getFn: (id: T) => Promise<string>
): Promise<Map<T, string>> {
  // 并发加载所有图片
  const promises = iconIds.map(async (id) => {
    try {
      const url = await getFn(id);
      return { id, url, success: true as const };
    } catch (error) {
      console.error(`加载图片 ${id} 失败:`, error);
      return { id, url: null, success: false as const };
    }
  });

  const results = await Promise.all(promises);
  const imageMap = new Map<T, string>();

  results.forEach((result) => {
    if (result.success && result.url) {
      imageMap.set(result.id, result.url);
    }
  });

  return imageMap;
}

/**
 * 批量获取召唤师头像图标
 *
 * @param iconIds 头像图标 ID 数组（字符串）
 * @returns Promise<Map<string, string>> ID 到 Base64 URL 的映射
 */
export async function batchGetProfileIcons(
  iconIds: string[]
): Promise<Map<string, string>> {
  return batchGetImages(iconIds, getProfileIcon);
}

/**
 * 批量获取英雄头像图标
 *
 * @param championIds 英雄 ID 数组（字符串）
 * @returns Promise<Map<string, string>> ID 到 Base64 URL 的映射
 */
export async function batchGetChampionIcons(
  championIds: string[]
): Promise<Map<string, string>> {
  return batchGetImages(championIds, getChampionIcon);
}

/**
 * React Hook: 获取并缓存图片
 *
 * @param fetchFn 获取图片的函数
 * @returns { data, isLoading, error }
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { data: avatarUrl, isLoading, error } = useImage(() => getProfileIcon("3460"));
 *
 *   if (isLoading) return <div>加载中...</div>;
 *   if (error) return <div>加载失败</div>;
 *
 *   return <img src={avatarUrl || ''} alt="头像" />;
 * }
 * ```
 */
export function useImage(fetchFn: () => Promise<string>) {
  const [data, setData] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    let cancelled = false;

    setIsLoading(true);
    setError(null);

    fetchFn()
      .then((url) => {
        if (!cancelled) {
          setData(url);
          setIsLoading(false);
        }
      })
      .catch((err) => {
        if (!cancelled) {
          setError(err);
          setIsLoading(false);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [fetchFn]);

  return { data, isLoading, error };
}

/**
 * React Hook: 批量获取并缓存图片（并发加载）
 *
 * @param iconIds 图片 ID 数组（字符串）
 * @param getFn 获取单张图片的函数
 * @returns { data, isLoading, error }
 *
 * @example
 * ```tsx
 * function MyComponent() {
 *   const { data: images, isLoading } = useImages(["3460", "3461", "3462"], getProfileIcon);
 *
 *   if (isLoading) return <div>加载中...</div>;
 *
 *   return (
 *     <div>
 *       {["3460", "3461", "3462"].map(id => (
 *         <img key={id} src={images?.get(id) || ''} alt={`头像 ${id}`} />
 *       ))}
 *     </div>
 *   );
 * }
 * ```
 */
export function useImages<T extends string>(
  iconIds: T[],
  getFn: (id: T) => Promise<string>
) {
  const [data, setData] = useState<Map<T, string> | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<Error | null>(null);

  // 使用 useMemo 生成稳定的 key 来比较数组
  const iconIdsKey = useMemo(() => {
    const sorted = [...iconIds].sort((a, b) => a.localeCompare(b));
    return JSON.stringify(sorted);
  }, [iconIds.join(",")]);

  useEffect(() => {
    let cancelled = false;

    // 如果没有需要加载的图片，直接返回
    if (iconIds.length === 0) {
      setData(new Map());
      setIsLoading(false);
      return;
    }

    setIsLoading(true);
    setError(null);

    batchGetImages(iconIds, getFn)
      .then((imageMap) => {
        if (!cancelled) {
          setData(imageMap);
          setIsLoading(false);
        }
      })
      .catch((err) => {
        if (!cancelled) {
          setError(err);
          setIsLoading(false);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [iconIdsKey, getFn]); // 使用 iconIdsKey 来比较数组

  return { data, isLoading, error };
}
