import "./App.css";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import { useEffect } from "react";
import Sidebar from "./components/Sidebar";
import RecordQuery from "./pages/RecordQuery";
import MatchAnalysis from "./pages/MatchAnalysis";
import WCMode from "./pages/WCMode";
import DataAnalysis from "./pages/DataAnalysis";

function App() {
  // 禁用右键菜单和开发者工具快捷键
  // useEffect(() => {
  //   // 禁用右键菜单
  //   const handleContextMenu = (e: MouseEvent) => {
  //     e.preventDefault();
  //     return false;
  //   };

  //   // 禁用开发者工具快捷键
  //   const handleKeyDown = (e: KeyboardEvent) => {
  //     // 禁用 F12
  //     if (e.key === "F12") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+Shift+I (开发者工具)
  //     if (e.ctrlKey && e.shiftKey && e.key === "I") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+Shift+J (控制台)
  //     if (e.ctrlKey && e.shiftKey && e.key === "J") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+Shift+C (检查元素)
  //     if (e.ctrlKey && e.shiftKey && e.key === "C") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+U (查看源代码)
  //     if (e.ctrlKey && e.key === "U") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+S (保存页面)
  //     if (e.ctrlKey && e.key === "S") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // 禁用 Ctrl+P (打印)
  //     if (e.ctrlKey && e.key === "P") {
  //       e.preventDefault();
  //       return false;
  //     }

  //     // Mac 系统的快捷键 (Cmd 替代 Ctrl)
  //     if (e.metaKey && e.shiftKey && (e.key === "I" || e.key === "J" || e.key === "C")) {
  //       e.preventDefault();
  //       return false;
  //     }
  //   };

  //   // 添加全局事件监听器
  //   document.addEventListener("contextmenu", handleContextMenu);
  //   document.addEventListener("keydown", handleKeyDown);

  //   // 清理函数
  //   return () => {
  //     document.removeEventListener("contextmenu", handleContextMenu);
  //     document.removeEventListener("keydown", handleKeyDown);
  //   };
  // }, []);

  return (
    <BrowserRouter>
      <div className="flex h-screen w-screen overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto bg-background">
          <Routes>
            <Route path="/" element={<Navigate to="/record-query" replace />} />
            <Route path="/record-query" element={<RecordQuery />} />
            <Route path="/match-analysis" element={<MatchAnalysis />} />
            <Route path="/wc-mode" element={<WCMode />} />
            <Route path="/data-analysis" element={<DataAnalysis />} />
          </Routes>
        </main>
      </div>
    </BrowserRouter>
  );
}

export default App;
