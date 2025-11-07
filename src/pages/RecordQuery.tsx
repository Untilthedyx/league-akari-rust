import { useState, useEffect, useCallback, useRef } from "react";
import PlayerInfo, { PlayerData } from "@/components/PlayerInfo";
import RecordFilter, { FilterOptions } from "@/components/RecordFilter";
import RecordList from "@/components/RecordList";
import type { RecordItem } from "@/lib/api/recordList";
import { getRecordList } from "@/lib/api/recordList";

// 模拟单个玩家数据
const generateMockPlayer = (): PlayerData => {
  return {
    id: "1",
    avatar: "3460",
    name: "like",
    playerId: "#85845",
    level: 30,
    soloRank: {
      rank: "最强王者",
      lp: 120,
      wins: 68,
    },
    flexRank: {
      rank: "流光翡翠",
      lp: 80,
      wins: 35,
      losses: 18,
    },
    historyStats: {
      avgKills: 8.5,
      avgDeaths: 3.2,
      avgAssists: 6.8,
      avgKDA: 4.78,
      avgGoldShare: 22.5,
      rankedWins: 68,
      rankedLosses: 35,
      totalWins: 103,
      totalLosses: 47,
    },
    favoriteHeroes: [
      {
        name: "李白",
        avatar: "86",
        matches: 45,
      },
      {
        name: "韩信",
        avatar: "87",
        matches: 32,
      },
      {
        name: "诸葛亮",
        avatar: "88",
        matches: 28,
      },
      {
        name: "露娜",
        avatar: "89",
        matches: 22,
      },
      {
        name: "花木兰",
        avatar: "90",
        matches: 18,
      },
    ],
    recentTeammates: [
      {
        name: "计伯言",
        playerId: "#85873",
        rankedWins: 7,
        rankedLosses: 12,
        totalWins: 45,
        totalLosses: 32,
      },
      {
        name: "她已不再孤立无援",
        playerId: "#59539",
        rankedWins: 1,
        rankedLosses: 1,
        totalWins: 23,
        totalLosses: 18,
      },
      {
        name: "游戏高手",
        playerId: "#12345",
        rankedWins: 15,
        rankedLosses: 8,
        totalWins: 89,
        totalLosses: 56,
      },
      {
        name: "刺客之王",
        playerId: "#67890",
        rankedWins: 12,
        rankedLosses: 10,
        totalWins: 67,
        totalLosses: 45,
      },
    ],
  };
};

export default function RecordQuery() {
  const [player] = useState<PlayerData>(generateMockPlayer());
  const [records, setRecords] = useState<RecordItem[]>([]);
  const [recordsFilter, setRecordsFilter] = useState<RecordItem[]>([]);
  const [filters, setFilters] = useState<FilterOptions>({});
  const [isLoading, setIsLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const currentIndexRef = useRef(0);

  // 加载更多战绩
  const loadMoreRecords = useCallback(async () => {
    if (isLoading || !hasMore) return;

    setIsLoading(true);
    try {
      // 模拟API请求延迟
      await new Promise((resolve) => setTimeout(resolve, 500));

      const begIndex = currentIndexRef.current;
      const endIndex = begIndex + 19;

      const newRecords = await getRecordList(
        "55cc79c4-3d20-535a-9bff-00b1867534d8",
        begIndex,
        endIndex
      );

      setRecords((prev) => {
        const totalLength = prev.length + newRecords.length;
        if (newRecords.length === 0 || totalLength >= 50) {
          setHasMore(false);
        }
        // 更新索引引用
        currentIndexRef.current = totalLength;
        return [...prev, ...newRecords];
      });
      setRecordsFilter((prev) => {
        const totalLength = prev.length + newRecords.length;
        if (newRecords.length === 0 || totalLength >= 50) {
          setHasMore(false);
        }
        return [...prev, ...newRecords];
      });
    } catch (error) {
      console.error("获取战绩列表失败:", error);
      setHasMore(false);
    } finally {
      setIsLoading(false);
    }
  }, [isLoading, hasMore]);

  // 筛选变化时重置列表
  useEffect(() => {
    setRecordsFilter(handleFilter(records, filters));
  }, [filters, records]);

  const handleFilter = (records: RecordItem[], filters: FilterOptions) => {
    let filteredRecords = [...records];
    Object.entries(filters).forEach(([key, value]) => {
      if (value === undefined || value === "") {
        return;
      }
      if (key === "queueId") {
        filteredRecords = filteredRecords.filter((record) => record.queueId === value);
      } else if (key === "win") {
        filteredRecords = filteredRecords.filter((record) => record.isWin.toString() === value);
      } else if (key === "mvp") {
        if (value === "mvp") {
          filteredRecords = filteredRecords.filter((record) => record.isBest && record.isWin);
        } else if (value === "svp") {
          filteredRecords = filteredRecords.filter((record) => record.isBest && !record.isWin);
        } else if (value === "other") {
          filteredRecords = filteredRecords.filter((record) => !record.isBest);
        }
      } else if (key === "hero") {
        filteredRecords = filteredRecords.filter((record) => record.champion.hero === value);
      }
    });
    return filteredRecords;
  };

  // 初始加载
  useEffect(() => {
    loadMoreRecords();
  }, []);

  const handleFilterChange = (newFilters: FilterOptions) => {
    setFilters(newFilters);
    console.log(newFilters);
  };

  return (
    <div className="h-full w-full p-6 overflow-hidden flex flex-col bg-background">
      {/* 主要内容区域：居中固定宽度布局 */}
      <div className="flex-1 flex justify-center gap-5 overflow-hidden">
        {/* 左侧列：玩家信息 */}
        <div className="flex flex-col overflow-hidden w-player-info shrink-0">
          <div className="flex-1 overflow-hidden">
            <PlayerInfo player={player} />
          </div>
        </div>

        {/* 右侧列：筛选和战绩列表 */}
        <div className="flex flex-col overflow-hidden w-record-list shrink-0">
          {/* 筛选模块 */}
          <div className="mb-5 shrink-0">
            <RecordFilter
              records={records}
              onFilterChange={handleFilterChange}
            />
          </div>

          {/* 战绩列表模块 */}
          <div className="flex-1 overflow-hidden flex flex-col">
            <div className="flex-1 overflow-y-auto pr-2">
              <RecordList
                records={recordsFilter}
                onLoadMore={loadMoreRecords}
                hasMore={hasMore}
                isLoading={isLoading}
              />
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
