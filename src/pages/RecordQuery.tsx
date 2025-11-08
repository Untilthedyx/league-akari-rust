import { useState, useEffect, useCallback, useRef, useMemo } from "react";
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
  const [filters, setFilters] = useState<FilterOptions>({});
  const [isLoading, setIsLoading] = useState(false);
  const [hasMore, setHasMore] = useState(true);
  const currentIndexRef = useRef(0);

  // 加载更多战绩
  const loadMoreRecords = useCallback(async () => {
    if (isLoading || !hasMore) return;

    setIsLoading(true);
    try {
      const begIndex = currentIndexRef.current;
      const endIndex = begIndex + 19;

      const newRecords = await getRecordList(
        "55cc79c4-3d20-535a-9bff-00b1867534d8",
        begIndex,
        endIndex
      );

      setRecords((prev) => {
        // 合并新数据，去重
        let records = [...prev, ...newRecords];
        records = records.filter((record, index, self) =>
          index === self.findIndex((t) => t.gameId === record.gameId)
        );
        const totalLength = records.length;

        // 更新索引：使用去重后的实际长度
        currentIndexRef.current = totalLength;

        // 如果返回的数据少于请求的数量，或者达到上限，标记为没有更多数据
        if (newRecords.length === 0 || newRecords.length < (endIndex - begIndex + 1)) {
          setHasMore(false);
        }

        return records;
      });
    } catch (error) {
      console.error("获取战绩列表失败:", error);
      setHasMore(false);
    } finally {
      setIsLoading(false);
    }
  }, [isLoading, hasMore]);

  // 筛选逻辑（使用 useMemo 优化性能）
  const recordsFilter = useMemo(() => {
    // 如果没有筛选条件，直接返回所有记录
    const hasFilters =
      filters.queueId !== undefined ||
      filters.win ||
      filters.mvp ||
      filters.hero;
    
    if (!hasFilters) {
      return records;
    }

    // 预处理：为每个 record 创建 participant 映射，避免重复查找
    const recordsWithParticipant = records
      .map((record) => {
        const participant = record.participants.find(
          (p) => p.puuid === record.puuid
        );
        return { record, participant };
      })
      .filter((item): item is { record: RecordItem; participant: NonNullable<typeof item.participant> } => 
        item.participant !== undefined
      );

    let filtered = recordsWithParticipant;

    // 根据 queueId 筛选
    if (filters.queueId !== undefined) {
      filtered = filtered.filter(
        (item) => item.record.queueId === filters.queueId
      );
    }

    // 根据 win 筛选
    if (filters.win) {
      filtered = filtered.filter((item) => {
        return filters.win === "true"
          ? item.participant.win
          : !item.participant.win;
      });
    }

    // 根据 mvp 筛选
    if (filters.mvp) {
      filtered = filtered.filter((item) => {
        if (filters.mvp === "mvp") {
          return item.participant.best && item.participant.win;
        } else if (filters.mvp === "svp") {
          return item.participant.best && !item.participant.win;
        } else if (filters.mvp === "other") {
          return !item.participant.best;
        }
        return true;
      });
    }

    // 根据 hero 筛选
    if (filters.hero) {
      filtered = filtered.filter(
        (item) => item.participant.champion.name === filters.hero
      );
    }

    // 只返回 record 数组
    return filtered.map((item) => item.record);
  }, [records, filters]);

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
