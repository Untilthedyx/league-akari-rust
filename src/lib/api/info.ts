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

  favoriteHeroes: Array<{
    championId: number;
    championName: string;
    matches: number;
  }>;
}

export async function getInfo(): Promise<PlayerInfoData> {
  console.log("getInfo");
  return invoke<PlayerInfoData>("get_info");
}
