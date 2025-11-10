import { useState, useRef } from "react";
import { Copy, Check, Trophy } from "lucide-react";
import AssetImage from "@/components/AssetImage";
import type { PlayerInfoData } from "@/lib/api/info";
import { useTranslation } from "react-i18next";

interface PlayerInfoProps {
  playerInfo: PlayerInfoData | undefined;
}

export default function PlayerInfo({ playerInfo }: PlayerInfoProps) {
  if (!playerInfo) return null;
  const { t } = useTranslation();
  const [copiedName, setCopiedName] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleCopy = async (text: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedName(true);
      setTimeout(() => setCopiedName(false), 2000);
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
              id={playerInfo.profileIconId}
              type="profile"
              alt={playerInfo.gameName}
              className="w-24 h-24 rounded-lg border-2 border-border object-cover"
            />
            <div className="absolute -bottom-1 -right-1 bg-primary text-primary-foreground text-xs font-bold rounded-full w-6 h-6 flex items-center justify-center border-2 border-card">
              {playerInfo.gameLevel}
            </div>
          </div>

          {/* 第二列：名称和段位信息 */}
          <div className="flex-1 flex flex-col justify-between min-w-0">
            {/* 第一行：名称和复制图标 */}
            <div className="flex items-center gap-2">
              <span className="font-semibold text-foreground truncate text-base">
                {playerInfo.gameName}
              </span>
              <button
                onClick={() => handleCopy(playerInfo.gameName)}
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

            {/* 第二行：历史最高段位 */}
            <div className="flex items-center gap-2">
              <Trophy className="size-3.5 text-purple-500 shrink-0" />
              <span className="text-sm text-foreground shrink-0 min-w-[100px]">
                历史最高：
                {playerInfo.highestRank?.rank && playerInfo.highestRank?.division
                  ? `${t(`tiers.${playerInfo.highestRank.rank}`)} ${playerInfo.highestRank.division}`
                  : t("tiers.UNRANKED")}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* 2. 段位模块 */}
      {
        <div className="bg-card border border-border rounded-lg p-4">
          <h3 className="text-sm font-semibold text-foreground mb-4">段位</h3>
        </div>
      }

      {/* 3. 常用英雄模块 */}
      {
        <div className="bg-card border border-border rounded-lg p-4">
          <h3 className="text-sm font-semibold text-foreground mb-4">
            常用英雄
          </h3>
          <div className="flex gap-2 justify-start">
            {playerInfo.favoriteHeroes.slice(0, 5).map((hero, idx) => (
              <div
                key={idx}
                className="relative shrink-0 w-14 h-14 rounded-lg border-2 border-border overflow-hidden bg-muted group"
              >
                <AssetImage
                  id={hero.championId}
                  type="champion"
                  alt={hero.championName}
                  className="w-full h-full object-cover"
                />
                {/* 场数显示 */}
                <div className="absolute bottom-0 right-0 bg-black/70 text-white text-[10px] font-semibold px-1 rounded-tl">
                  {hero.matches}
                </div>
                {/* 英雄名称提示 */}
                <div className="absolute inset-0 bg-black/80 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                  <span className="text-xs text-white font-medium text-center px-1">
                    {hero.championName}
                  </span>
                </div>
              </div>
            ))}
          </div>
        </div>
      }
    </div>
  );
}
