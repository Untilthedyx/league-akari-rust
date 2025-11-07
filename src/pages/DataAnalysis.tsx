import React from "react";
import { Database } from "lucide-react";

export default function DataAnalysis() {
  return (
    <div className="h-full w-full p-8 overflow-auto">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <Database className="size-8 text-primary" />
            <h1 className="text-4xl font-bold text-foreground">数据分析</h1>
          </div>
          <p className="text-muted-foreground text-lg">全面数据统计和可视化分析</p>
        </div>
        <div className="bg-card rounded-xl border border-border p-8 shadow-lg">
          <div className="text-center py-12">
            <Database className="size-16 mx-auto mb-4 text-muted-foreground/50" />
            <p className="text-muted-foreground text-lg">数据分析内容区域</p>
            <p className="text-muted-foreground/70 text-sm mt-2">
              在这里添加数据分析的具体功能
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

