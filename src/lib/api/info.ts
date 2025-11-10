import { invoke } from "@tauri-apps/api/core";

export interface PlayerInfoData {
  puuid: string;
  gameName: string;
  gameLevel: number;
  profileIconId: number;
  highestRank: {
    rank: string;
    division: string;
  };
  soloRank: {
    rank: string;
    division: string;
    lp?: number;
    wins?: number;
    losses?: number;
  };
  flexRank: {
    rank: string;
    division: string;
    lp?: number;
    wins?: number;
    losses?: number;
  };

  favoriteHeroes: Array<FavoriteHero>;
}

export interface FavoriteHero {
  championId: number;
  championName: string;
  matches: number;
}

export async function getInfo(puuid: string): Promise<PlayerInfoData> {
  try {
    const playerInfo = await invoke<PlayerInfoData>("get_info", { puuid });
    return playerInfo;
  } catch (error) {
    console.error("获取玩家信息失败:", error);
    throw error;
  }
}
