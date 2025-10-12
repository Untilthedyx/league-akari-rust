import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState(false);

  async function fetchClientInfo() {
    try {
      const result: any = await invoke("_get_client_info");
      console.log("获取到的客户端信息:", result);
    } catch (error) {
      console.error("调用失败:", error);
    }
  }

  // 修正后的代码
  useEffect(() => {
    const timer = setInterval(async () => {
      // 直接使用invoke的返回值，而不是依赖状态
      const isRunning: boolean = await invoke("_is_running");
      setGreetMsg(isRunning);

      console.log("当前状态:", isRunning);
      if (isRunning) {
        fetchClientInfo();
      }
    }, 1000);

    return () => clearInterval(timer);
  }, []); // 仍然保持空依赖，因为我们不再依赖greetMsg

  return (
    <main className="container">
      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vite.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <p>{greetMsg}</p>
    </main>
  );
}

export default App;
