import { useRef, useCallback, useMemo, memo } from "react";
import { useTranslation } from "react-i18next";
import { Shield, Heart, Clock, Building2, Target } from "lucide-react";
import { cn } from "@/lib/utils";
import AssetImage from "@/components/AssetImage";
import type { RecordItem } from "@/lib/api/recordList";
import {
  Tooltip,
  TooltipTrigger,
  TooltipContent,
} from "@/components/ui/tooltip";

interface RecordListProps {
  records: RecordItem[];
  onLoadMore: () => void;
  hasMore: boolean;
  isLoading: boolean;
}

// 预处理记录数据，提取 participant、teammates、enemies
function useProcessedRecords(records: RecordItem[]) {
  return useMemo(() => {
    return records
      .map((record) => {
        // 从 participants 中找到与 record.puuid 匹配的 participant
        const participant = record.participants.find(
          (p) => p.puuid === record.puuid
        );

        // 如果找不到匹配的 participant，返回 null
        if (!participant) {
          return null;
        }

        // 根据 teamId 区分队友和敌人
        const teammates = record.participants.filter(
          (p) => p.teamId === participant.teamId && p.puuid !== participant.puuid
        );
        const enemies = record.participants.filter(
          (p) => p.teamId !== participant.teamId
        );

        return {
          record,
          participant,
          teammates,
          enemies,
        };
      })
      .filter((item): item is NonNullable<typeof item> => item !== null);
  }, [records]);
}

// 单个记录项组件（使用 memo 优化）
const RecordItemComponent = memo(
  ({
    record,
    participant,
    teammates,
    enemies,
    isLast,
    onLastElementRef,
  }: {
    record: RecordItem;
    participant: NonNullable<
      ReturnType<typeof useProcessedRecords>[number]
    >["participant"];
    teammates: NonNullable<
      ReturnType<typeof useProcessedRecords>[number]
    >["teammates"];
    enemies: NonNullable<
      ReturnType<typeof useProcessedRecords>[number]
    >["enemies"];
    isLast: boolean;
    onLastElementRef: (node: HTMLDivElement | null) => void;
  }) => {
    const { t } = useTranslation();

    const formatDuration = (seconds: number) => {
      const mins = Math.floor(seconds / 60);
      const secs = seconds % 60;
      return `${mins}:${secs.toString().padStart(2, "0")}`;
    };

    return (
      <div
        ref={isLast ? onLastElementRef : null}
        className={cn(
          "grid gap-2 px-2 py-1.5 hover:bg-muted/30 transition-colors",
          "grid-cols-[auto_auto_auto_auto_auto_auto_auto]",
          participant.win ? "bg-green-500/5" : "bg-red-500/5"
        )}
      >
              {/* 第一列：胜利/失败 + 游戏时长 */}
              <div className="flex flex-col justify-center items-center gap-0.5 w-12 shrink-0 place-self-center">
                <div
                  className={cn(
                    "text-xl font-bold",
                    participant.win ? "text-green-600" : "text-red-600"
                  )}
                >
                  {participant.win ? t("common.win") : t("common.defeat")}
                </div>
                <div className="text-xs text-muted-foreground">
                  {formatDuration(record.duration)}
                </div>
              </div>

              {/* 第二列：KDA 效率值 (k + a) / (d + 1) */}
              <div className="flex flex-col justify-center items-center gap-0.5 w-14 shrink-0 place-self-center">
                <div className="text-lg font-bold text-foreground">
                  {(
                    (participant.kills + participant.assists) /
                    (participant.deaths + 1)
                  ).toFixed(2)}
                </div>
              </div>

              {/* 第三列：英雄图片 + 最佳 */}
              <div className="flex items-center justify-center w-14 shrink-0 place-self-center">
                <Tooltip>
                  <TooltipTrigger asChild>
                    <div className="relative overflow-hidden rounded">
                      <AssetImage
                        id={participant.champion.id}
                        type="champion"
                        alt={participant.champion.name}
                        className="w-12 h-12 object-cover"
                      />
                      {participant.best && (
                        <div
                          className={cn(
                            "absolute top-0 left-0 text-center text-[10px] font-bold rounded-br-sm w-[25px]",
                            participant.win
                              ? "bg-yellow-500 text-yellow-900"
                              : "bg-indigo-500 text-indigo-900"
                          )}
                        >
                          {participant.win ? "MVP" : "SVP"}
                        </div>
                      )}
                    </div>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>{participant.champion.name}</p>
                  </TooltipContent>
                </Tooltip>
              </div>

              {/* 第四列：游戏模式 + 游戏时间 */}
              <div className="flex flex-col justify-center items-center gap-0.5 min-w-[100px] max-w-[120px] place-self-center">
                {/* 第一行：游戏模式 */}
                <div className="text-xm text-foreground font-medium">
                  {t(`queue.${record.queueId}`, {
                    defaultValue: t("queue.other"),
                  })}
                </div>
                {/* 第二行：游戏时间 */}
                <div className="flex items-center gap-1 text-[10px] text-muted-foreground">
                  <Clock className="size-4 shrink-0" />
                  <time className="text-xs">
                    {new Date(record.gameCreation).toLocaleString("zh-CN", {
                      month: "2-digit",
                      day: "2-digit",
                      hour: "2-digit",
                      minute: "2-digit",
                    })}
                  </time>
                </div>
              </div>

              {/* 第五列：KDA + 符文/技能（第一行）+ 装备+眼位（第二行） */}
              <div className="flex flex-col justify-center gap-0.5 w-49 shrink-0 place-self-center">
                {/* 第一行：KDA（左）+ 符文和召唤师技能（右） */}
                <div className="flex items-center justify-between w-49">
                  {/* KDA - 左对齐，彩色显示 */}
                  <div className="flex items-center gap-0.5 shrink-0">
                    <span className="text-xs font-semibold">
                      <span className="text-blue-600">
                        {participant.kills}
                      </span>
                      <span className="text-muted-foreground">/</span>
                      <span className="text-red-600">
                        {participant.deaths}
                      </span>
                      <span className="text-muted-foreground">/</span>
                      <span className="text-green-600">
                        {participant.assists}
                      </span>
                    </span>
                  </div>
                  {/* 符文和召唤师技能 - 右对齐 */}
                  <div className="flex items-center gap-0 shrink-0">
                    {participant.perks.slice(0, 2).map((perk, index) => (
                      <Tooltip key={`perk-tooltip-${perk.id}-${index}`}>
                        <TooltipTrigger asChild>
                          <div>
                            <AssetImage
                              id={perk.id}
                              type="perk"
                              alt={perk.name}
                              className="w-7 h-7 rounded object-cover shrink-0"
                            />
                          </div>
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>{perk.name}</p>
                        </TooltipContent>
                      </Tooltip>
                    ))}
                    {participant.spells.slice(0, 2).map((spell, index) => (
                      <Tooltip key={`spell-tooltip-${spell.id}-${index}`}>
                        <TooltipTrigger asChild>
                          <div>
                            <AssetImage
                              id={spell.id}
                              type="spell"
                              alt={spell.name}
                              className="w-7 h-7 rounded object-cover shrink-0"
                            />
                          </div>
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>{spell.name}</p>
                        </TooltipContent>
                      </Tooltip>
                    ))}
                  </div>
                </div>
                {/* 第二行：装备 + 眼位，与第一行宽度一致 */}
                <div className="flex items-center justify-start gap-0 h-6 w-49">
                  {/* items 数组：前6个是装备，第7个是眼位 */}
                  {participant.items.slice(0, 6).map((item, index) => (
                    <Tooltip key={`item-tooltip-${item.id}-${index}`}>
                      <TooltipTrigger asChild>
                        <div>
                          <AssetImage
                            id={item.id}
                            type="item"
                            alt={item.name}
                            className="w-7 h-7 rounded object-cover shrink-0"
                          />
                        </div>
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>{item.name}</p>
                      </TooltipContent>
                    </Tooltip>
                  ))}
                  {participant.items[6] && (
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <div>
                          <AssetImage
                            id={participant.items[6].id}
                            type="item"
                            alt={participant.items[6].name}
                            className="w-7 h-7 rounded object-cover shrink-0"
                          />
                        </div>
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>{participant.items[6].name}</p>
                      </TooltipContent>
                    </Tooltip>
                  )}
                </div>
              </div>

              {/* 第六列：对塔伤害/对人伤害/抗伤/治疗（2x2网格布局） */}
              <div className="grid grid-cols-2 gap-x-1.5 gap-y-1.5 w-[150px] shrink-0 place-self-center">
                {/* 对塔伤害 */}
                <div className="flex items-center gap-1 h-3.5 w-full">
                  <Building2 className="size-4 text-purple-500 shrink-0" />
                  <div className="flex items-center gap-0.5 shrink-0 text-[9px] flex-1 justify-end">
                    <span className="text-foreground font-medium whitespace-nowrap text-right w-[22px]">
                      {(participant.damageToTurrets / 1000).toFixed(1)}k
                    </span>
                    <span className="text-muted-foreground leading-tight whitespace-nowrap text-right w-[30px]">
                      {(participant.damageToTurretsPercentage * 100).toFixed(1)}%
                    </span>
                  </div>
                </div>
                {/* 对人伤害 */}
                <div className="flex items-center gap-1 h-3.5 w-full">
                  <Target className="size-4 text-orange-500 shrink-0" />
                  <div className="flex items-center gap-0.5 shrink-0 text-[9px] flex-1 justify-end">
                    <span className="text-foreground font-medium whitespace-nowrap text-right w-[22px]">
                      {(participant.damageToChampions / 1000).toFixed(1)}k
                    </span>
                    <span className="text-muted-foreground leading-tight whitespace-nowrap text-right w-[30px]">
                      {(participant.damageToChampionsPercentage * 100).toFixed(1)}%
                    </span>
                  </div>
                </div>
                {/* 抗伤 */}
                <div className="flex items-center gap-1 h-3.5 w-full">
                  <Shield className="size-4 text-blue-500 shrink-0" />
                  <div className="flex items-center gap-0.5 shrink-0 text-[9px] flex-1 justify-end">
                    <span className="text-foreground font-medium whitespace-nowrap text-right w-[22px]">
                      {(participant.damageTaken / 1000).toFixed(1)}k
                    </span>
                    <span className="text-muted-foreground leading-tight whitespace-nowrap text-right w-[30px]">
                      {(participant.damageTakenPercentage * 100).toFixed(1)}%
                    </span>
                  </div>
                </div>
                {/* 治疗 */}
                <div className="flex items-center gap-1 h-3.5 w-full">
                  <Heart className="size-4 text-green-500 shrink-0" />
                  <div className="flex items-center gap-0.5 shrink-0 text-[9px] flex-1 justify-end">
                    <span className="text-foreground font-medium whitespace-nowrap text-right w-[22px]">
                      {(participant.heal / 1000).toFixed(1)}k
                    </span>
                    <span className="text-muted-foreground leading-tight whitespace-nowrap text-right w-[30px]">
                      {(participant.healPercentage * 100).toFixed(1)}%
                    </span>
                  </div>
                </div>
              </div>

              {/* 第七列：我方和敌方英雄阵容（两行） */}
              <div className="flex flex-col justify-center items-center gap-1.5 w-[180px] shrink-0 place-self-center">
                {/* 第一行：我方英雄阵容（5个） */}
                <div className="flex items-center justify-center gap-1 w-full h-7 shrink-0">
                  {[
                    participant,
                    ...teammates,
                  ]
                    .filter(
                      (player, index, self) =>
                        index === self.findIndex((p) => p.puuid === player.puuid)
                    )
                    .slice(0, 5)
                    .map((player, idx) => (
                      <Tooltip key={`teammate-tooltip-${player.puuid || idx}`}>
                        <TooltipTrigger asChild>
                          <div className="flex items-center gap-1 h-full group cursor-pointer shrink-0">
                            <AssetImage
                              id={player.champion.id}
                              type="champion"
                              alt={player.champion.name}
                              className="w-7 h-7 rounded object-cover shrink-0"
                            />
                          </div>
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>
                            {player.name}: {player.kills}/{player.deaths}/
                            {player.assists}
                          </p>
                        </TooltipContent>
                      </Tooltip>
                    ))}
                </div>

                {/* 第二行：敌方英雄阵容（5个） */}
                <div className="flex items-center justify-center gap-1 w-full h-7 shrink-0">
                  {enemies
                    .filter(
                      (enemy, index, self) =>
                        index === self.findIndex((e) => e.puuid === enemy.puuid)
                    )
                    .slice(0, 5)
                    .map((enemy) => (
                      <Tooltip key={`enemy-tooltip-${enemy.puuid}`}>
                        <TooltipTrigger asChild>
                          <div className="flex items-center gap-1 h-full group cursor-pointer shrink-0">
                            <AssetImage
                              id={enemy.champion.id}
                              type="champion"
                              alt={enemy.champion.name}
                              className="w-7 h-7 rounded object-cover shrink-0"
                            />
                          </div>
                        </TooltipTrigger>
                        <TooltipContent>
                          <p>
                            {enemy.name}: {enemy.kills}/{enemy.deaths}/
                            {enemy.assists}
                          </p>
                        </TooltipContent>
                      </Tooltip>
                    ))}
                </div>
              </div>
            </div>
    );
  },
  (prevProps, nextProps) => {
    // 自定义比较函数，只在关键属性变化时重新渲染
    return (
      prevProps.record.gameId === nextProps.record.gameId &&
      prevProps.participant.puuid === nextProps.participant.puuid &&
      prevProps.isLast === nextProps.isLast
    );
  }
);

RecordItemComponent.displayName = "RecordItemComponent";

export default function RecordList({
  records,
  onLoadMore,
  hasMore,
  isLoading,
}: RecordListProps) {
  const observerRef = useRef<IntersectionObserver | null>(null);

  // 预处理记录数据
  const processedRecords = useProcessedRecords(records);

  const lastRecordElementRef = useCallback(
    (node: HTMLDivElement | null) => {
      if (isLoading) return;
      if (observerRef.current) observerRef.current.disconnect();
      observerRef.current = new IntersectionObserver((entries) => {
        if (entries[0].isIntersecting && hasMore) {
          onLoadMore();
        }
      });
      if (node) observerRef.current.observe(node);
    },
    [isLoading, hasMore, onLoadMore]
  );

  return (
    <div className="bg-card border border-border rounded-lg overflow-hidden">
      <div className="divide-y divide-border">
        {processedRecords.map(({ record, participant, teammates, enemies }, index) => (
          <RecordItemComponent
            key={record.gameId}
            record={record}
            participant={participant}
            teammates={teammates}
            enemies={enemies}
            isLast={index === processedRecords.length - 1}
            onLastElementRef={lastRecordElementRef}
          />
        ))}
      </div>

      {/* 加载状态 */}
      {isLoading && (
        <div className="text-center py-8 text-muted-foreground border-t border-border">
          加载中...
        </div>
      )}

      {/* 没有更多数据 */}
      {!hasMore && records.length > 0 && (
        <div className="text-center py-8 text-muted-foreground border-t border-border">
          没有更多战绩了
        </div>
      )}

      {/* 空状态 */}
      {records.length === 0 && !isLoading && (
        <div className="text-center py-12 text-muted-foreground">
          暂无战绩记录
        </div>
      )}
    </div>
  );
}
