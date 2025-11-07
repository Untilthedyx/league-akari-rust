import { useState, useEffect, useRef } from "react";
import { Copy, Check, Trophy } from "lucide-react";
import AssetImage from "@/components/AssetImage";

export interface PlayerData {
  id: string;
  avatar: string;
  name: string;
  playerId: string; // 玩家ID（如 #85845）
  level: number;
  // 单双排位信息
  soloRank: {
    rank: string;
    lp?: number;
    wins?: number;
  };
  // 灵活排位信息
  flexRank: {
    rank: string;
    lp?: number;
    wins?: number;
    losses?: number;
  };
  // 历史统计信息
  historyStats: {
    avgKills: number;
    avgDeaths: number;
    avgAssists: number;
    avgKDA: number;
    avgGoldShare: number;
    rankedWins: number;
    rankedLosses: number;
    totalWins: number;
    totalLosses: number;
  };
  // 常用英雄（最多5个）
  favoriteHeroes: Array<{
    name: string;
    avatar: string;
    matches: number; // 使用场数
  }>;
  // 近期队友
  recentTeammates: Array<{
    name: string;
    playerId: string;
    rankedWins: number;
    rankedLosses: number;
    totalWins: number;
    totalLosses: number;
  }>;
}

interface PlayerInfoProps {
  player: PlayerData;
}

export default function PlayerInfo({ player }: PlayerInfoProps) {
  const [copiedName, setCopiedName] = useState(false);
  const [copiedIds, setCopiedIds] = useState<Set<string>>(new Set());
  const containerRef = useRef<HTMLDivElement>(null);
  const [visibleModules, setVisibleModules] = useState({
    history: true,
    heroes: true,
    teammates: true,
  });

  // 使用ResizeObserver检测容器高度
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const resizeObserver = new ResizeObserver((entries) => {
      for (const entry of entries) {
        const availableHeight = entry.contentRect.height;
        // 定义高度阈值（像素，包含间距）
        // 人物模块约 ~120px (包含间距 ~136px)
        // 历史信息模块约 ~220px (包含间距)
        // 常用英雄模块约 ~120px (包含间距)
        // 近期队友模块约 ~80px + (队友数量 * 50px) (包含间距)

        const teammateModuleHeight = 80 + player.recentTeammates.length * 50;

        // 根据可用高度决定显示哪些模块
        // 优先显示重要模块：人物 > 历史信息 > 常用英雄 > 近期队友
        if (availableHeight < 136) {
          // 高度太小，只显示人物模块
          setVisibleModules({
            history: false,
            heroes: false,
            teammates: false,
          });
        } else if (availableHeight < 356) {
          // 显示人物 + 历史信息 (136 + 220)
          setVisibleModules({ history: true, heroes: false, teammates: false });
        } else if (availableHeight < 476) {
          // 显示人物 + 历史信息 + 常用英雄 (136 + 220 + 120)
          setVisibleModules({ history: true, heroes: true, teammates: false });
        } else {
          // 显示所有模块
          const neededHeight = 136 + 220 + 120 + teammateModuleHeight;
          if (availableHeight >= neededHeight) {
            setVisibleModules({ history: true, heroes: true, teammates: true });
          } else {
            // 如果高度不够显示所有模块，至少显示前三个
            setVisibleModules({
              history: true,
              heroes: true,
              teammates: false,
            });
          }
        }
      }
    });

    resizeObserver.observe(container);

    return () => {
      resizeObserver.disconnect();
    };
  }, [player.recentTeammates.length]);

  const handleCopy = async (
    text: string,
    type: "name" | "id" = "name",
    playerId?: string
  ) => {
    try {
      await navigator.clipboard.writeText(text);
      if (type === "name") {
        setCopiedName(true);
        setTimeout(() => setCopiedName(false), 2000);
      } else if (type === "id" && playerId) {
        setCopiedIds((prev) => new Set(prev).add(playerId));
        setTimeout(() => {
          setCopiedIds((prev) => {
            const next = new Set(prev);
            next.delete(playerId);
            return next;
          });
        }, 2000);
      }
    } catch (err) {
      console.error("复制失败:", err);
    }
  };

  return (
    <div ref={containerRef} className="h-full overflow-y-auto space-y-4">
      {/* 1. 人物模块 */}
      <div className="bg-card border border-border rounded-lg p-4">
        <div className="flex gap-4 h-24">
          {/* 第一列：头像和等级 */}
          <div className="relative shrink-0">
            <AssetImage
              id={String(player.avatar)}
              type="profile"
              alt={player.name}
              className="w-24 h-24 rounded-lg border-2 border-border object-cover"
            />
            <div className="absolute -bottom-1 -right-1 bg-primary text-primary-foreground text-xs font-bold rounded-full w-6 h-6 flex items-center justify-center border-2 border-card">
              {player.level}
            </div>
          </div>

          {/* 第二列：名称和段位信息 */}
          <div className="flex-1 flex flex-col justify-between min-w-0">
            {/* 第一行：名称和复制图标 */}
            <div className="flex items-center gap-2">
              <span className="font-semibold text-foreground truncate text-base">
                {player.name}
              </span>
              <button
                onClick={() => handleCopy(player.name, "name")}
                className="shrink-0 p-1 hover:bg-accent rounded transition-colors"
                title="复制名称"
              >
                {copiedName ? (
                  <Check className="size-3.5 text-green-500" />
                ) : (
                  <Copy className="size-3.5 text-muted-foreground" />
                )}
              </button>
            </div>

            {/* 第二行：单双排位段位 */}
            <div className="flex items-center gap-2">
              <Trophy className="size-3.5 text-yellow-500 shrink-0" />
              <span className="text-sm text-foreground">
                单双排位：{player.soloRank.rank}
                {player.soloRank.lp !== undefined && (
                  <>
                    <span className="inline-block items-center justify-center font-mono w-8 text-center text-yellow-800">
                      {String(player.soloRank.lp).padStart(3, "\u00A0")}
                    </span>
                    <span className="font-mono">LP</span>
                  </>
                )}
              </span>
            </div>

            {/* 第三行：灵活排位段位 + 图表 */}
            <div className="flex items-center gap-2">
              <Trophy className="size-3.5 text-purple-500 shrink-0" />
              <span className="text-sm text-foreground shrink-0 min-w-[100px]">
                灵活排位：{player.flexRank.rank}
                {player.flexRank.lp !== undefined && (
                  <>
                    <span className="inline-block items-center justify-center font-mono w-8 text-center text-purple-800">
                      {String(player.flexRank.lp).padStart(3, "\u00A0")}
                    </span>
                    <span className="font-mono">LP</span>
                  </>
                )}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* 2. 历史信息模块 */}
      {visibleModules.history && (
        <div className="bg-card border border-border rounded-lg p-4">
          <h3 className="text-sm font-semibold text-foreground mb-4">信息</h3>
          <div className="space-y-3">
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">场均击杀</span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.avgKills.toFixed(1)}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">场均死亡</span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.avgDeaths.toFixed(1)}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">场均助攻</span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.avgAssists.toFixed(1)}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">场均 KDA</span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.avgKDA.toFixed(2)}
              </span>
            </div>
            <div className="flex items-center justify-between">
              <span className="text-sm text-muted-foreground">
                场均经济占比
              </span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.avgGoldShare.toFixed(1)}%
              </span>
            </div>
            <div className="flex items-center justify-between pt-2 border-t border-border">
              <span className="text-sm text-muted-foreground">排位胜场</span>
              <span className="text-sm font-semibold text-foreground">
                {player.historyStats.rankedWins}胜
                {player.historyStats.rankedLosses}负
              </span>
            </div>
          </div>
        </div>
      )}

      {/* 3. 常用英雄模块 */}
      {visibleModules.heroes && (
        <div className="bg-card border border-border rounded-lg p-4">
          <h3 className="text-sm font-semibold text-foreground mb-4">
            常用英雄
          </h3>
          <div className="flex gap-2 justify-start">
            {player.favoriteHeroes.slice(0, 5).map((hero, idx) => (
              <div
                key={idx}
                className="relative shrink-0 w-14 h-14 rounded-lg border-2 border-border overflow-hidden bg-muted group"
              >
                <AssetImage
                  id={String(hero.avatar)}
                  type="champion"
                  alt={hero.name}
                  className="w-full h-full object-cover"
                />
                {/* 场数显示 */}
                <div className="absolute bottom-0 right-0 bg-black/70 text-white text-[10px] font-semibold px-1 rounded-tl">
                  {hero.matches}
                </div>
                {/* 英雄名称提示 */}
                <div className="absolute inset-0 bg-black/80 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                  <span className="text-xs text-white font-medium text-center px-1">
                    {hero.name}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 4. 近期队友模块 */}
      {visibleModules.teammates && (
        <div className="bg-card border border-border rounded-lg p-4">
          <h3 className="text-sm font-semibold text-foreground mb-4">
            近期队友
          </h3>
          <div className="space-y-2.5">
            {player.recentTeammates.map((teammate, idx) => (
              <div
                key={idx}
                className="flex items-center justify-between gap-3 py-2 border-b border-border/50 last:border-0 text-sm"
              >
                <div className="flex items-center gap-2 min-w-0 flex-1">
                  <span className="text-sm text-foreground font-medium truncate">
                    {teammate.name}
                  </span>
                  <span className="text-xs text-muted-foreground shrink-0">
                    {teammate.playerId}
                  </span>
                  <button
                    onClick={() =>
                      handleCopy(teammate.playerId, "id", teammate.playerId)
                    }
                    className="shrink-0 p-1 hover:bg-accent rounded transition-colors"
                    title="复制ID"
                  >
                    {copiedIds.has(teammate.playerId) ? (
                      <Check className="size-3 text-green-500" />
                    ) : (
                      <Copy className="size-3 text-muted-foreground" />
                    )}
                  </button>
                </div>
                <div className="flex items-center gap-4 shrink-0 text-xs text-muted-foreground">
                  <span className="whitespace-nowrap">
                    {teammate.rankedWins}胜{teammate.rankedLosses}负
                  </span>
                  <span className="whitespace-nowrap">
                    {teammate.totalWins}胜{teammate.totalLosses}负
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
