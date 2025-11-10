import { useState, useMemo } from "react";
import { useTranslation } from "react-i18next";
import { RotateCcw } from "lucide-react";
import { Button } from "./ui/button";
import { SearchableSelect, SelectOption } from "./ui/searchable-select";
import { useInitStore } from "@/lib/store/initStore";

export interface FilterOptions {
  queueId?: number;
  win?: string;
  mvp?: string;
  hero?: string;
}

export default function RecordFilter({
  onFilterChange,
}: {
  onFilterChange: (filters: FilterOptions) => void;
}) {
  const { t } = useTranslation();
  const [filters, setFilters] = useState<FilterOptions>({});

  const gameModeOptions: SelectOption[] = useMemo(() => {
    const options: SelectOption[] = [{ value: "", label: t("filter.all") }];
    options.push({ value: "420", label: t("queue.420") });
    options.push({ value: "430", label: t("queue.430") });
    options.push({ value: "440", label: t("queue.440") });
    options.push({ value: "450", label: t("queue.450") });
    options.push({ value: "480", label: t("queue.480") });
    options.push({ value: "490", label: t("queue.490") });
    options.push({ value: "890", label: t("queue.890") });
    options.push({ value: "900", label: t("queue.900") });
    options.push({ value: "1700", label: t("queue.1700") });
    options.push({ value: "1900", label: t("queue.1900") });
    options.push({ value: "2300", label: t("queue.2300") });
    options.push({ value: "2400", label: t("queue.2400") });
    options.push({ value: "3100", label: t("queue.3100") });
    options.push({ value: "other", label: t("queue.other") });
    return options;
  }, [t]);

  // 胜负选项
  const winOptions: SelectOption[] = useMemo(
    () => [
      { value: "", label: t("filter.all") },
      { value: "true", label: t("common.win") },
      { value: "false", label: t("common.defeat") },
    ],
    [t]
  );

  // MVP选项
  const mvpOptions: SelectOption[] = useMemo(
    () => [
      { value: "", label: t("filter.all") },
      { value: "mvp", label: t("common.mvp") },
      { value: "svp", label: t("common.svp") },
      { value: "other", label: t("common.other") },
    ],
    [t]
  );

  // 英雄列表（从 records 的所有 participants 中提取唯一的英雄名称，根据 puuid 筛选）
  const filterHeroes = useInitStore((state) => state.filterHeroes);
  const heroOptions: SelectOption[] = useMemo(() => {
    const options: SelectOption[] = [{ value: "", label: t("filter.all") }];
    filterHeroes.forEach((hero) => {
      options.push({
        value: hero.championId.toString(),
        label: hero.championName,
      });
    });
    return options;
  }, [filterHeroes, t]);

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
        placeholder={t("filter.allHero")}
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
