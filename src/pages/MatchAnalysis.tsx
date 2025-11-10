import { BarChart3 } from "lucide-react";

export default function MatchAnalysis() {
  return (
    <div className="h-full w-full p-8 overflow-auto">
      <div className="max-w-7xl mx-auto">
        <div className="mb-8">
          <div className="flex items-center gap-3 mb-2">
            <BarChart3 className="size-8 text-primary" />
            <h1 className="text-4xl font-bold text-foreground">对局分析</h1>
          </div>
          <p className="text-muted-foreground text-lg">深入分析对局数据和表现</p>
        </div>
        <div className="bg-card rounded-xl border border-border p-8 shadow-lg">
          <div className="text-center py-12">
            <BarChart3 className="size-16 mx-auto mb-4 text-muted-foreground/50" />
            <p className="text-muted-foreground text-lg">对局分析内容区域</p>
            <p className="text-muted-foreground/70 text-sm mt-2">
              在这里添加对局分析的具体功能
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

