import { invoke } from "@tauri-apps/api/core";

// Rust 返回的原始数据结构（date 是时间戳字符串）
interface RecordItemRaw {
  id: string;
  champion: {
    id: string; // puuid
    name: string; // 玩家名称 + tag
    hero: string; // 英雄名称
    heroId: string;
    score: number; // 评分
    kda: {
      kills: number;
      deaths: number;
      assists: number;
    };
  };
  isWin: boolean; // 是否胜利
  isBest: boolean; // 是否是最佳
  date: string; // 游戏时间（毫秒时间戳字符串）
  duration: number; // 游戏时长（秒）
  queueId: number; // 队列ID
  // 6装备 + 1个眼位
  items: Array<{
    id: string;
    name: string;
  }>;
  // 2个召唤师技能
  spells: Array<{
    id: string;
    name: string;
  }>;
  // 2个符文
  perks: Array<{
    id: string;
    name: string;
  }>;
  stats: {
    damage: number; // 伤害数值
    damageShare: number; // 伤害占比
    damageTaken: number; // 抗伤数值
    damageTakenShare: number; // 抗伤占比
    healing: number; // 治疗数值
    healingShare: number; // 治疗占比
  };
  teammates: Array<{
    id: string; // puuid
    name: string; // 玩家名称 + tag
    hero: string; // 英雄名称
    heroId: string;
    score: number; // 评分
    kda: {
      kills: number;
      deaths: number;
      assists: number;
    };
  }>;
  enemies: Array<{
    id: string; // puuid
    name: string; // 玩家名称 + tag
    hero: string; // 英雄名称
    heroId: string; // 英雄ID
    score: number; // 评分
    kda: {
      kills: number;
      deaths: number;
      assists: number;
    };
  }>;
}

// 前端使用的数据结构（date 是 Date 对象）
export interface RecordItem extends Omit<RecordItemRaw, "date"> {
  date: Date; // 游戏时间
}

export async function getRecordList(puuid: string, beg_index: number, end_index: number): Promise<RecordItem[]> {
  console.log(puuid, beg_index, end_index);
  try {
    const recordListRaw = await invoke<RecordItemRaw[]>("get_rank_list", {
      puuid, beg_index, end_index
    });

    // 转换时间戳字符串为 Date 对象
    return recordListRaw.map((record) => ({
      ...record,
      date: new Date(Number(record.date)),
    }));
  } catch (error) {
    console.error("获取战绩列表失败:", error);
    throw error;
  }
}
