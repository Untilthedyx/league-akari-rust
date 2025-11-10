import { useMemo, useState, useEffect } from "react";
import {
  getProfileIcon,
  getChampionIcon,
  getItemIcon,
  getSpellIcon,
  getPerkIcon,
} from "@/lib/api/asset";
import { cn } from "@/lib/utils";

/**
 * 图片类型枚举
 */
export type ImageType = "profile" | "champion" | "item" | "spell" | "perk";

/**
 * 缓存键类型
 */
type CacheKey = `${ImageType}:${string}`;

/**
 * 全局图片缓存
 * 使用 Map 存储已加载的图片 URL，避免重复请求
 */
const imageCache = new Map<CacheKey, string>();

/**
 * 正在加载的 Promise 缓存
 * 避免同一张图片同时发起多个请求
 */
const loadingPromises = new Map<CacheKey, Promise<string>>();

/**
 * 图片组件属性
 */
export interface AssetImageProps {
  /** 图片 ID（字符串） */
  id: number;
  /** 图片类型 */
  type: ImageType;
  /** CSS 类名 */
  className?: string;
  /** alt 文本 */
  alt?: string;
  /** 其他 img 标签属性 */
  [key: string]: any;
}

/**
 * 获取图片 URL（带缓存）
 *
 * @param id 图片 ID
 * @param type 图片类型
 * @returns Promise<string> 图片 URL
 */
async function getImageData(id: number, type: ImageType): Promise<string> {
  const cacheKey: CacheKey = `${type}:${id}`;

  // 1. 检查缓存
  if (imageCache.has(cacheKey)) {
    return imageCache.get(cacheKey)!;
  }

  // 2. 检查是否正在加载
  if (loadingPromises.has(cacheKey)) {
    return loadingPromises.get(cacheKey)!;
  }

  // 3. 根据类型选择对应的获取函数
  let fetchPromise: Promise<string>;
  switch (type) {
    case "profile":
      fetchPromise = getProfileIcon(id);
      break;
    case "champion":
      fetchPromise = getChampionIcon(id);
      break;
    case "item":
      fetchPromise = getItemIcon(id);
      break;
    case "spell":
      fetchPromise = getSpellIcon(id);
      break;
    case "perk":
      fetchPromise = getPerkIcon(id);
      break;
  }

  // 4. 创建包装 Promise，处理缓存
  const wrappedPromise = fetchPromise
    .then((url) => {
      // 加载成功后存入缓存
      imageCache.set(cacheKey, url);
      loadingPromises.delete(cacheKey);
      return url;
    })
    .catch((error) => {
      // 加载失败时移除加载中的 Promise
      loadingPromises.delete(cacheKey);
      throw error;
    });

  // 5. 记录正在加载的 Promise
  loadingPromises.set(cacheKey, wrappedPromise);

  return wrappedPromise;
}

/**
 * 封装的图片组件
 *
 * 自动处理图片加载、缓存和错误处理
 *
 * @example
 * ```tsx
 * // 显示头像
 * <AssetImage id="3460" type="profile" className="w-24 h-24" alt="头像" />
 *
 * // 显示英雄头像
 * <AssetImage id="1" type="champion" className="w-56 h-56" alt="英雄" />
 *
 * // 显示物品图标
 * <AssetImage id="1001" type="item" className="w-32 h-32" alt="物品" />
 *
 * // 显示召唤师技能
 * <AssetImage id="SummonerFlash" type="spell" className="w-16 h-16" alt="闪现" />
 * ```
 */
export default function AssetImage({
  id,
  type,
  className = "",
  alt = "",
  ...props
}: AssetImageProps) {
  if (id === 0) {
    return (
      <div
        className={cn("bg-muted/30 border-none", className)}
        style={{ border: "none", outline: "none" }}
        aria-label={alt || "空"}
        {...props}
      />
    );
  }

  // 生成缓存键
  const cacheKey: CacheKey = useMemo(() => {
    return `${type}:${id}` as CacheKey;
  }, [type, id]);

  // 检查缓存中是否已有该图片
  const cachedData = useMemo(() => {
    return imageCache.get(cacheKey) || null;
  }, [cacheKey]);

  // 图片 URL 状态
  const [imageData, setImageData] = useState<string | null>(cachedData);
  const [isLoading, setIsLoading] = useState(!cachedData);
  const [hasError, setHasError] = useState(false);

  // 加载图片
  useEffect(() => {
    // 如果缓存中已有，直接使用
    if (cachedData) {
      setImageData(cachedData);
      setIsLoading(false);
      return;
    }

    // 否则从 API 加载
    let cancelled = false;
    setIsLoading(true);
    setHasError(false);

    getImageData(id, type)
      .then((url) => {
        if (!cancelled) {
          setImageData(url);
          setIsLoading(false);
        }
      })
      .catch((error) => {
        if (!cancelled) {
          console.error(`加载图片失败 [${type}:${id}]:`, error);
          setHasError(true);
          setIsLoading(false);
        }
      });

    return () => {
      cancelled = true;
    };
  }, [id, type, cacheKey, cachedData]);

  // 如果图片加载中或加载失败，显示空的占位元素（保持大小）
  if (isLoading || hasError || !imageData) {
    return (
      <div
        className={cn("bg-muted/30 border-none", className)}
        style={{ border: "none", outline: "none" }}
        aria-label={alt || `${type} ${id}`}
        {...props}
      />
    );
  }

  return (
    <img
      src={imageData}
      alt={alt || `${type} ${id}`}
      className={cn("border-none", className)}
      style={{ border: "none", outline: "none" }}
      onError={() => {
        // 加载失败时标记错误，显示占位元素
        setHasError(true);
        setImageData(null);
      }}
      {...props}
    />
  );
}

/**
 * 清除图片缓存
 *
 * @param type 可选，指定要清除的图片类型
 * @param id 可选，指定要清除的图片 ID
 */
export function clearImageCache(type?: ImageType, id?: string) {
  if (type && id) {
    // 清除特定图片
    const cacheKey: CacheKey = `${type}:${id}` as CacheKey;
    imageCache.delete(cacheKey);
    loadingPromises.delete(cacheKey);
  } else if (type) {
    // 清除特定类型的所有图片
    const keysToDelete: CacheKey[] = [];
    imageCache.forEach((_, key) => {
      if (key.startsWith(`${type}:`)) {
        keysToDelete.push(key);
      }
    });
    keysToDelete.forEach((key) => {
      imageCache.delete(key);
      loadingPromises.delete(key);
    });
  } else {
    // 清除所有缓存
    imageCache.clear();
    loadingPromises.clear();
  }
}

/**
 * 获取缓存统计信息
 */
export function getCacheStats() {
  return {
    cachedImages: imageCache.size,
    loadingImages: loadingPromises.size,
  };
}
