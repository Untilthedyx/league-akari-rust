import { create } from 'zustand';
// 暂时不需要存储在本地，因为初始化状态需要计算
// import { persist, createJSONStorage } from 'zustand/middleware';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';

interface InitStatus {
  initialized: boolean;
  message: string;
}

interface InitState {
  isInitialized: boolean;
  initMessage: string;
  setInitialized: (value: boolean) => void;
  setInitMessage: (message: string) => void;
}

export const useInitStore = create<InitState>()(
  (set) => ({
    isInitialized: false,
    initMessage: '正在初始化...',
    setInitialized: (value) => set({ isInitialized: value }),
    setInitMessage: (message) => set({ initMessage: message }),
  })
);

// 检查后端初始化状态（用于页面刷新后的状态恢复）
async function checkInitStatus() {
  try {
    const initialized = await invoke<boolean>('check_init_status');
    if (initialized) {
      useInitStore.getState().setInitialized(true);
      useInitStore.getState().setInitMessage('初始化完成');
    }
  } catch (error) {
    console.error('检查初始化状态失败:', error);
  }
}

// 设置初始化状态事件监听器
export function setupInitListener() {
  // 先检查当前状态（用于页面刷新后的状态恢复）
  checkInitStatus();

  // 监听初始化状态事件
  listen<InitStatus>('init-status', (event) => {
    useInitStore.getState().setInitMessage(event.payload.message);
    // 根据 initialized 字段更新状态
    useInitStore.getState().setInitialized(event.payload.initialized);
  }).catch((error) => {
    console.error('设置初始化监听器失败:', error);
  });
}

