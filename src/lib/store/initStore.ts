import { create } from "zustand";
// 暂时不需要存储在本地，因为初始化状态需要计算
// import { persist, createJSONStorage } from 'zustand/middleware';
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { FavoriteHero } from "@/lib/api/info";

interface InitStatus {
  initialized: boolean;
  message: string;
  summoner: Summoner | null;
  filterHeroes: Array<FavoriteHero>;
}

interface Summoner {
  puuid: string;
}

interface InitState {
  isInitialized: boolean;
  initMessage: string;
  summoner: Summoner | null;
  filterHeroes: Array<FavoriteHero>;
  setInitialized: (value: boolean) => void;
  setInitMessage: (message: string) => void;
  setSummoner: (summoner: Summoner | null) => void;
  setFilterHeroes: (filterHeroes: Array<FavoriteHero>) => void;
}

export const useInitStore = create<InitState>()((set) => ({
  isInitialized: false,
  initMessage: "正在初始化...",
  summoner: null,
  filterHeroes: [],
  setInitialized: (value) => set({ isInitialized: value }),
  setInitMessage: (message) => set({ initMessage: message }),
  setSummoner: (summoner) => set({ summoner: summoner }),
  setFilterHeroes: (filterHeroes) => set({ filterHeroes: filterHeroes }),
}));

// 检查后端初始化状态（用于页面刷新后的状态恢复）
async function checkInitStatus() {
  try {
    // 后端返回的是元组 (bool, Summoner)，序列化后是数组 [bool, Summoner]
    const [initialized, summoner] = await invoke<[boolean, Summoner]>(
      "check_init_status"
    );
    // 批量更新状态，避免多次状态更新导致渲染问题
    useInitStore.setState({
      isInitialized: initialized,
      initMessage: initialized ? "初始化完成" : "初始化失败",
      summoner: initialized ? summoner || null : null,
    });
  } catch (error) {
    console.error("检查初始化状态失败:", error);
  }
}

// 设置初始化状态事件监听器
export function setupInitListener() {
  // 先检查当前状态（用于页面刷新后的状态恢复）
  checkInitStatus();

  // 监听初始化状态事件
  listen<InitStatus>("init-status", (event) => {
    // 批量更新状态，避免多次状态更新导致渲染问题
    useInitStore.setState({
      initMessage: event.payload.message,
      isInitialized: event.payload.initialized,
    });

    if (event.payload.summoner) {
      useInitStore.getState().setSummoner(event.payload.summoner);
    }
  }).catch((error) => {
    console.error("设置初始化监听器失败:", error);
  });
}
