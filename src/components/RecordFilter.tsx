import { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { RotateCcw } from "lucide-react";
import { Button } from "./ui/button";
import { SearchableSelect, SelectOption } from "./ui/searchable-select";
import type { RecordItem } from "@/lib/api/recordList";

export interface FilterOptions {
  queueId?: number;
  win?: string;
  mvp?: string;
  hero?: string;
}

export default function RecordFilter({
  records,
  onFilterChange,
}: {
  records: RecordItem[];
  onFilterChange: (filters: FilterOptions) => void;
}) {
  const { t } = useTranslation();
  const [filters, setFilters] = useState<FilterOptions>({});

  // 游戏模式选项（从 records 中提取唯一的 queueId）
  const gameModeOptions: SelectOption[] = useMemo(() => {
    const options: SelectOption[] = [
      { value: "", label: t("filter.all") },
    ];

    const uniqueQueueIds = Array.from(
      new Set(records.map((record) => record.queueId))
    ).sort((a, b) => a - b);

    uniqueQueueIds.forEach((queueId) => {
      options.push({
        value: queueId.toString(),
        label: t(`queue.${queueId}`, { defaultValue: t("queue.other") }),
      });
    });

    return options;
  }, [records, t]);

  // 胜负选项
  const winOptions: SelectOption[] = useMemo(
    () => [
      { value: "", label: t("filter.all") },
      { value: "true", label: t("common.win") },
      { value: "false", label: t("common.defeat") },
    ],
    []
  );

  // MVP选项
  const mvpOptions: SelectOption[] = useMemo(
    () => [
      { value: "", label: t("filter.all") },
      { value: "mvp", label: t("common.mvp") },
      { value: "svp", label: t("common.svp") },
      { value: "other", label: t("common.other") },
    ],
    []
  );

  // 英雄列表（从 records 的所有 participants 中提取唯一的英雄名称，根据 puuid 筛选）
  const heroOptions: SelectOption[] = useMemo(() => {
    const options: SelectOption[] = [
      { value: "", label: t("filter.all") },
    ];

    // 从所有 records 的所有 participants 中，找到与 record.puuid 匹配的 participant
    // 然后提取唯一的英雄名称
    const uniqueHeroes = Array.from(
      new Set(
        records
          .map((record) => {
            // 找到与 record.puuid 匹配的 participant
            const participant = record.participants.find(
              (p) => p.puuid === record.puuid
            );
            return participant?.champion.name;
          })
          .filter((name): name is string => !!name) // 过滤掉 undefined
      )
    ).sort();

    uniqueHeroes.forEach((hero) => {
      options.push({
        value: hero,
        label: hero,
      });
    });

    return options;
  }, [records, t]);

  const handleGameModeChange = (value: string) => {
    const newFilters = {
      ...filters,
      queueId: value ? Number(value) : undefined,
    };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  const handleWinChange = (value: string) => {
    const newFilters = {
      ...filters,
      win: value || undefined,
    };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  const handleMVPChange = (value: string) => {
    const newFilters = {
      ...filters,
      mvp: value || undefined,
    };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  const handleHeroChange = (value: string) => {
    const newFilters = {
      ...filters,
      hero: value || undefined,
    };
    setFilters(newFilters);
    onFilterChange(newFilters);
  };

  const clearFilters = () => {
    const emptyFilters: FilterOptions = {};
    setFilters(emptyFilters);
    onFilterChange(emptyFilters);
  };

  const hasActiveFilters =
    filters.queueId !== undefined || filters.win || filters.mvp || filters.hero;

  return (
    <div className="flex items-center gap-3">
      {/* 游戏模式筛选 */}
      <SearchableSelect
        options={gameModeOptions}
        value={filters.queueId?.toString() || ""}
        onChange={handleGameModeChange}
        placeholder={t("filter.gameMode")}
        className="w-[180px]"
      />

      {/* 胜负筛选 */}
      <SearchableSelect
        options={winOptions}
        value={filters.win || ""}
        onChange={handleWinChange}
        placeholder={t("filter.result")}
        className="w-[140px]"
      />

      {/* MVP筛选 */}
      <SearchableSelect
        options={mvpOptions}
        value={filters.mvp || ""}
        onChange={handleMVPChange}
        placeholder={t("filter.bestPlayer")}
        className="w-[140px]"
      />

      {/* 英雄筛选 */}
      <SearchableSelect
        options={heroOptions}
        value={filters.hero || ""}
        onChange={handleHeroChange}
        placeholder={t("filter.selectHero")}
        className="w-[140px]"
      />

      {/* 重置按钮 */}
      <Button
        variant="ghost"
        size="sm"
        onClick={clearFilters}
        disabled={!hasActiveFilters}
        className="h-10 px-4"
      >
        <RotateCcw className="size-4 mr-2" />
        {t("common.reset")}
      </Button>
    </div>
  );
}
