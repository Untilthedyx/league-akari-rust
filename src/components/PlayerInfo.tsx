import { useState, useRef } from "react";
import { Copy, Check, Trophy, Award, TrendingUp, Users } from "lucide-react";
import AssetImage from "@/components/AssetImage";
import type { PlayerInfoData } from "@/lib/api/info";
import { useTranslation } from "react-i18next";

interface PlayerInfoProps {
  playerInfo: PlayerInfoData | undefined;
}

export default function PlayerInfo({ playerInfo }: PlayerInfoProps) {
  // 所有 Hooks 必须在条件检查之前调用，保持调用顺序一致
  const { t } = useTranslation();
  const [copiedName, setCopiedName] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // 分割游戏名称（按 # 分割）
  const gameNameParts = playerInfo?.gameName?.includes("#")
    ? playerInfo.gameName.split("#")
    : null;

  // 如果 playerInfo 不存在，在 Hooks 之后返回
  if (!playerInfo) return null;

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
            <div className="flex items-center gap-2 min-w-0">
              {gameNameParts ? (
                <span className="font-semibold text-foreground truncate text-base flex items-baseline gap-1">
                  <span className="truncate">{gameNameParts[0]}</span>
                  <span className="text-muted-foreground font-normal shrink-0 text-sm">
                    #{gameNameParts[1]}
                  </span>
                </span>
              ) : (
                <span className="font-semibold text-foreground truncate text-base">
                  {playerInfo.gameName}
                </span>
              )}
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
                {playerInfo.highestRank?.rank &&
                playerInfo.highestRank?.division
                  ? `${t(`tiers.${playerInfo.highestRank.rank}`)} ${
                      playerInfo.highestRank.division
                    }`
                  : t("tiers.UNRANKED")}
              </span>
            </div>
          </div>
        </div>
      </div>

      {/* 2. 段位模块 */}
      <div className="bg-card border border-border rounded-lg p-4">
        <h3 className="text-sm font-semibold text-foreground mb-4 flex items-center gap-2">
          <Award className="size-4 text-primary" />
          段位信息
        </h3>
        <div className="flex flex-col gap-4">
          {/* 单双排位 */}
          <div className="bg-muted/50 border border-border rounded-lg p-4">
            <div className="flex items-center gap-2 mb-3">
              <TrendingUp className="size-4 text-blue-500" />
              <span className="text-sm font-semibold text-foreground">
                单双排位
              </span>
            </div>
            {playerInfo.soloRank?.rank &&
            playerInfo.soloRank?.rank !== "UNRANKED" ? (
              <div className="space-y-2">
                <div className="flex items-baseline gap-2">
                  <span className="text-lg font-bold text-foreground">
                    {t(`tiers.${playerInfo.soloRank.rank}`)}
                  </span>
                  {playerInfo.soloRank.division && (
                    <span className="text-sm text-muted-foreground">
                      {playerInfo.soloRank.division}
                    </span>
                  )}
                </div>
                {playerInfo.soloRank.lp !== undefined && (
                  <div className="text-sm text-muted-foreground">
                    胜点：
                    <span className="text-foreground font-medium">
                      {playerInfo.soloRank.lp} LP
                    </span>
                  </div>
                )}
                {playerInfo.soloRank.wins !== undefined &&
                  playerInfo.soloRank.losses !== undefined && (
                    <div className="flex items-center gap-4 text-sm">
                      <div className="text-muted-foreground">
                        胜场：
                        <span className="text-green-500 font-medium">
                          {playerInfo.soloRank.wins}
                        </span>
                      </div>
                      <div className="text-muted-foreground">
                        负场：
                        <span className="text-red-500 font-medium">
                          {playerInfo.soloRank.losses}
                        </span>
                      </div>
                      <div className="text-muted-foreground">
                        胜率：
                        <span className="text-foreground font-medium ml-1">
                          {Math.round(
                            (playerInfo.soloRank.wins /
                              (playerInfo.soloRank.wins +
                                playerInfo.soloRank.losses)) *
                              100
                          )}
                          %
                        </span>
                      </div>
                    </div>
                  )}
              </div>
            ) : (
              <div className="text-sm text-muted-foreground">
                {t("tiers.UNRANKED")}
              </div>
            )}
          </div>

          {/* 灵活排位 */}
          <div className="bg-muted/50 border border-border rounded-lg p-4">
            <div className="flex items-center gap-2 mb-3">
              <Users className="size-4 text-purple-500" />
              <span className="text-sm font-semibold text-foreground">
                灵活排位
              </span>
            </div>
            {playerInfo.flexRank?.rank &&
            playerInfo.flexRank?.rank !== "UNRANKED" ? (
              <div className="space-y-2">
                <div className="flex items-baseline gap-2">
                  <span className="text-lg font-bold text-foreground">
                    {t(`tiers.${playerInfo.flexRank.rank}`)}
                  </span>
                  {playerInfo.flexRank.division && (
                    <span className="text-sm text-muted-foreground">
                      {playerInfo.flexRank.division}
                    </span>
                  )}
                </div>
                {playerInfo.flexRank.lp !== undefined && (
                  <div className="text-sm text-muted-foreground">
                    胜点：
                    <span className="text-foreground font-medium">
                      {playerInfo.flexRank.lp} LP
                    </span>
                  </div>
                )}
                {playerInfo.flexRank.wins !== undefined &&
                  playerInfo.flexRank.losses !== undefined && (
                    <div className="flex items-center gap-4 text-sm">
                      <div className="text-muted-foreground">
                        胜场：
                        <span className="text-green-500 font-medium">
                          {playerInfo.flexRank.wins}
                        </span>
                      </div>
                      <div className="text-muted-foreground">
                        负场：
                        <span className="text-red-500 font-medium">
                          {playerInfo.flexRank.losses}
                        </span>
                      </div>
                      <div className="text-muted-foreground">
                        胜率：
                        <span className="text-foreground font-medium ml-1">
                          {Math.round(
                            (playerInfo.flexRank.wins /
                              (playerInfo.flexRank.wins +
                                playerInfo.flexRank.losses)) *
                              100
                          )}
                          %
                        </span>
                      </div>
                    </div>
                  )}
              </div>
            ) : (
              <div className="text-sm text-muted-foreground">
                {t("tiers.UNRANKED")}
              </div>
            )}
          </div>
        </div>
      </div>

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
