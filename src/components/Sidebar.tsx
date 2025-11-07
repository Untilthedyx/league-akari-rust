import { NavLink, useLocation } from "react-router-dom";
import { Search, BarChart3, TrendingUp, Database } from "lucide-react";
import { cn } from "@/lib/utils";

const menuItems = [
  {
    id: "record-query",
    label: "战绩查询",
    icon: Search,
    path: "/record-query",
  },
  {
    id: "match-analysis",
    label: "对局分析",
    icon: BarChart3,
    path: "/match-analysis",
  },
  {
    id: "wc-mode",
    label: "WC模式",
    icon: TrendingUp,
    path: "/wc-mode",
  },
  {
    id: "data-analysis",
    label: "数据分析",
    icon: Database,
    path: "/data-analysis",
  },
];

export default function Sidebar() {
  const location = useLocation();

  return (
    <aside className="w-sidebar min-w-sidebar h-full bg-sidebar border-r border-sidebar-border flex flex-col shadow-lg">
      {/* Logo/Header */}
      <div className="h-16 flex items-center justify-center border-b border-sidebar-border bg-sidebar-accent/30">
        <h2 className="text-xl font-bold text-sidebar-foreground tracking-wide">
          游戏助手
        </h2>
      </div>

      {/* Navigation Menu */}
      <nav className="flex-1 p-4 space-y-1 overflow-y-auto">
        {menuItems.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.path;
          return (
            <NavLink
              key={item.id}
              to={item.path}
              className={({ isActive: navIsActive }) =>
                cn(
                  "group flex items-center gap-3 px-4 py-3 rounded-lg transition-all duration-200",
                  "text-sidebar-foreground/80 hover:bg-sidebar-accent hover:text-sidebar-accent-foreground",
                  "hover:translate-x-1 hover:shadow-sm",
                  (isActive || navIsActive) &&
                    "bg-sidebar-primary text-sidebar-primary-foreground shadow-md scale-[1.02] translate-x-1"
                )
              }
            >
              <Icon 
                className={cn(
                  "size-5 shrink-0 transition-transform duration-200",
                  (location.pathname === item.path) && "scale-110"
                )} 
              />
              <span className="font-medium text-sm">{item.label}</span>
            </NavLink>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="h-16 border-t border-sidebar-border flex items-center justify-center">
        <p className="text-xs text-sidebar-foreground/60">版本 1.0.0</p>
      </div>
    </aside>
  );
}

