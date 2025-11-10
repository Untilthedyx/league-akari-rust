import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./lib/i18n/config";
import { setupInitListener } from "./lib/store/initStore";

// 设置初始化状态监听器
setupInitListener();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
