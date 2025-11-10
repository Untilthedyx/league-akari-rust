import { invoke } from "@tauri-apps/api/core";

// Rust 返回的原始数据结构（date 是时间戳字符串）
export interface RecordItem {
  gameId: string; // 游戏 ID
  puuid: string; // 玩家 ID
  gameCreation: number; // 游戏创建时间（毫秒时间戳）
  duration: number; // 游戏时长（秒）
  queueId: number; // 队列ID(游戏模式)
  participants: Participant[];
}

export interface Participant {
  puuid: string;
  name: string;
  tag: string;
  teamId: number;
  win: boolean;
  lane: string;
  // 这个暂时无效
  best: boolean;

  // Infomation
  champion: Item;
  spells: Item[];
  perks: Item[];
  items: Item[];

  // Stats
  damageToTurrets: number;
  damageToTurretsPercentage: number;
  damageToChampions: number;
  damageToChampionsPercentage: number;
  damageTaken: number;
  damageTakenPercentage: number;
  heal: number;
  healPercentage: number;

  // KDA
  kills: number;
  deaths: number;
  assists: number;
  kda: number;
}

interface Item {
  id: number;
  name: string;
}

export async function getRecordList(
  puuid: string,
  begIndex: number,
  endIndex: number
): Promise<RecordItem[]> {
  try {
    const recordListRaw = await invoke<RecordItem[]>("get_rank_list", {
      puuid,
      begIndex,
      endIndex,
    });
    return recordListRaw;
  } catch (error) {
    console.error("获取战绩列表失败:", error);
    throw error;
  }
}
