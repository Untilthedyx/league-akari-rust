import "./App.css";
import { BrowserRouter, Routes, Route, Navigate } from "react-router-dom";
import Sidebar from "./components/Sidebar";
import InitScreen from "./components/InitScreen";
import RecordQuery from "./pages/RecordQuery";
import MatchAnalysis from "./pages/MatchAnalysis";
import WCMode from "./pages/WCMode";
import DataAnalysis from "./pages/DataAnalysis";
import { useInitStore } from "./lib/store/initStore";

function App() {
  const isInitialized = useInitStore((state) => state.isInitialized);

  // 如果未初始化，显示初始化页面
  if (!isInitialized) {
    return <InitScreen />;
  }

  // 初始化完成后显示主应用
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
