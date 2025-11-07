import { useRef, useCallback } from "react";
import { useTranslation } from "react-i18next";
import { Sword, Shield, Heart, Clock } from "lucide-react";
import { cn } from "@/lib/utils";
import AssetImage from "@/components/AssetImage";
import type { RecordItem } from "@/lib/api/recordList";

interface RecordListProps {
  records: RecordItem[];
  onLoadMore: () => void;
  hasMore: boolean;
  isLoading: boolean;
}

export default function RecordList({
  records,
  onLoadMore,
  hasMore,
  isLoading,
}: RecordListProps) {
  const { t } = useTranslation();
  const observerRef = useRef<IntersectionObserver | null>(null);
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

  const formatDuration = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  return (
    <div className="bg-card border border-border rounded-lg overflow-hidden">
      <div className="divide-y divide-border">
        {records.map((record, index) => (
          <div
            key={`${record.id}-${index}`}
            ref={index === records.length - 1 ? lastRecordElementRef : null}
            className={cn(
              "grid gap-2 px-2 py-1.5 hover:bg-muted/30 transition-colors",
              "grid-cols-[auto_auto_auto_auto_auto_auto_auto]",
              record.isWin ? "bg-green-500/5" : "bg-red-500/5"
            )}
          >
            {/* 第一列：胜利/失败 + 游戏时长 */}
            <div className="flex flex-col justify-center items-center gap-0.5 w-12 shrink-0 place-self-center">
              <div
                className={cn(
                  "text-xl font-bold",
                  record.isWin ? "text-green-600" : "text-red-600"
                )}
              >
                {record.isWin ? t("common.win") : t("common.defeat")}
              </div>
              <div className="text-xs text-muted-foreground">
                {formatDuration(record.duration)}
              </div>
            </div>

            {/* 第二列：KDA 效率值 (k + a) / (d + 1) */}
            <div className="flex flex-col justify-center items-center gap-0.5 w-14 shrink-0 place-self-center">
              <div className="text-lg font-bold text-foreground">
                {(
                  (record.champion.kda.kills + record.champion.kda.assists) /
                  (record.champion.kda.deaths + 1)
                ).toFixed(2)}
              </div>
            </div>

            {/* 第三列：英雄图片 + 最佳 */}
            <div className="flex items-center justify-center w-14 shrink-0 place-self-center">
              <div className="relative overflow-hidden rounded">
                <AssetImage
                  id={record.champion.heroId}
                  type="champion"
                  alt={record.champion.hero}
                  className="w-12 h-12 object-cover"
                />
                {record.isBest && (
                  <div
                    className={cn(
                      "absolute top-0 left-0 text-center text-[10px] font-bold rounded-br-sm w-[25px]",
                      record.isWin
                        ? "bg-yellow-500 text-yellow-900"
                        : "bg-indigo-500 text-indigo-900"
                    )}
                  >
                    {record.isWin ? "MVP" : "SVP"}
                  </div>
                )}
              </div>
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
                  {record.date.toLocaleString("zh-CN", {
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
                      {record.champion.kda.kills}
                    </span>
                    <span className="text-muted-foreground">/</span>
                    <span className="text-red-600">
                      {record.champion.kda.deaths}
                    </span>
                    <span className="text-muted-foreground">/</span>
                    <span className="text-green-600">
                      {record.champion.kda.assists}
                    </span>
                  </span>
                </div>
                {/* 符文和召唤师技能 - 右对齐 */}
                <div className="flex items-center gap-0 shrink-0">
                  {record.perks.slice(0, 2).map((perk, index) => (
                    <AssetImage
                      key={`${perk.id}-${index}`}
                      id={perk.id}
                      type="perk"
                      alt={perk.name}
                      className="w-7 h-7 rounded object-cover shrink-0"
                      title={perk.name}
                    />
                  ))}
                  {record.spells.slice(0, 2).map((spell, index) => (
                    <AssetImage
                      key={`${spell.id}-${index}`}
                      id={spell.id}
                      type="spell"
                      alt={spell.name}
                      className="w-7 h-7 rounded object-cover shrink-0"
                      title={spell.name}
                    />
                  ))}
                </div>
              </div>
              {/* 第二行：装备 + 眼位，与第一行宽度一致 */}
              <div className="flex items-center justify-start gap-0 h-6 w-49">
                {/* items 数组：前6个是装备，第7个是眼位 */}
                {record.items.slice(0, 6).map((item, index) => (
                  <AssetImage
                    key={`${item.id}-${index}`}
                    id={item.id}
                    type="item"
                    alt={item.name}
                    className="w-7 h-7 rounded object-cover shrink-0"
                    title={item.name}
                  />
                ))}
                {record.items[6] && (
                  <AssetImage
                    id={record.items[6].id}
                    type="item"
                    alt={record.items[6].name}
                    className="w-7 h-7 rounded object-cover shrink-0"
                    title={record.items[6].name}
                  />
                )}
              </div>
            </div>

            {/* 第六列：伤害/抗伤/治疗（三行，每行包含图标、进度条、数值、占比） */}
            <div className="flex flex-col justify-center items-center gap-1 w-[130px] shrink-0 place-self-center">
              {/* 伤害 */}
              <div className="flex items-center gap-1.5 h-4">
                <Sword className="size-3.5 text-orange-500 shrink-0" />
                <div className="flex-1 min-w-0 h-full flex items-center">
                  <div className="w-full h-1.5 bg-muted rounded-full overflow-hidden">
                    <div
                      className="h-full bg-orange-500 transition-all"
                      style={{
                        width: `${Math.min(
                          record.stats.damageShare * 100,
                          100
                        )}%`,
                      }}
                    />
                  </div>
                </div>
                <div className="flex items-center gap-0.5 shrink-0 text-[10px] min-w-[50px] justify-end">
                  <span className="text-foreground font-medium">
                    {(record.stats.damage / 1000).toFixed(1)}k
                  </span>
                  <span className="text-muted-foreground leading-tight">
                    {(record.stats.damageShare * 100).toFixed(1)}%
                  </span>
                </div>
              </div>
              {/* 抗伤 */}
              <div className="flex items-center gap-1.5 h-4">
                <Shield className="size-3.5 text-blue-500 shrink-0" />
                <div className="flex-1 min-w-0 h-full flex items-center">
                  <div className="w-full h-1.5 bg-muted rounded-full overflow-hidden">
                    <div
                      className="h-full bg-blue-500 transition-all"
                      style={{
                        width: `${Math.min(
                          record.stats.damageTakenShare,
                          100
                        )}%`,
                      }}
                    />
                  </div>
                </div>
                <div className="flex items-center gap-0.5 shrink-0 text-[10px] min-w-[50px] justify-end">
                  <span className="text-foreground font-medium">
                    {(record.stats.damageTaken / 1000).toFixed(1)}k
                  </span>
                  <span className="text-muted-foreground">
                    {(record.stats.damageTakenShare * 100).toFixed(1)}%
                  </span>
                </div>
              </div>
              {/* 治疗 */}
              <div className="flex items-center gap-1.5 h-4">
                <Heart className="size-3.5 text-green-500 shrink-0" />
                <div className="flex-1 min-w-0 h-full flex items-center">
                  <div className="w-full h-1.5 bg-muted rounded-full overflow-hidden">
                    <div
                      className="h-full bg-green-500 transition-all"
                      style={{
                        width: `${Math.min(record.stats.healingShare, 100)}%`,
                      }}
                    />
                  </div>
                </div>
                <div className="flex items-center gap-0.5 shrink-0 text-[10px] min-w-[50px] justify-end">
                  <span className="text-foreground font-medium">
                    {(record.stats.healing / 1000).toFixed(1)}k
                  </span>
                  <span className="text-muted-foreground leading-tight">
                    {(record.stats.healingShare * 100).toFixed(1)}%
                  </span>
                </div>
              </div>
            </div>

            {/* 第七列：我方和敌方英雄阵容（两行） */}
            <div className="flex flex-col justify-center items-center gap-1.5 w-[180px] shrink-0 place-self-center">
              {/* 第一行：我方英雄阵容（5个） */}
              <div className="flex items-center justify-center gap-1 w-full h-7 shrink-0">
                {[
                  record.champion,
                  ...record.teammates.filter(
                    (t) => t.id !== record.champion.id
                  ),
                ]
                  .filter(
                    (player, index, self) =>
                      index === self.findIndex((p) => p.id === player.id)
                  )
                  .slice(0, 5)
                  .map((player, idx) => (
                    <div
                      key={player.id || idx}
                      className="flex items-center gap-1 h-full group cursor-pointer shrink-0"
                      title={`${player.name}: ${player.kda.kills}/${player.kda.deaths}/${player.kda.assists}`}
                    >
                      <AssetImage
                        id={player.heroId}
                        type="champion"
                        alt={player.hero}
                        className="w-7 h-7 rounded object-cover shrink-0"
                      />
                    </div>
                  ))}
              </div>

              {/* 第二行：敌方英雄阵容（5个） */}
              <div className="flex items-center justify-center gap-1 w-full h-7 shrink-0">
                {record.enemies
                  .filter(
                    (enemy, index, self) =>
                      index === self.findIndex((e) => e.id === enemy.id)
                  )
                  .slice(0, 5)
                  .map((enemy) => (
                    <div
                      key={enemy.id}
                      className="flex items-center gap-1 h-full group cursor-pointer shrink-0"
                      title={`${enemy.name}: ${enemy.kda.kills}/${enemy.kda.deaths}/${enemy.kda.assists}`}
                    >
                      <AssetImage
                        id={enemy.heroId}
                        type="champion"
                        alt={enemy.hero}
                        className="w-7 h-7 rounded object-cover shrink-0"
                      />
                    </div>
                  ))}
              </div>
            </div>
          </div>
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
