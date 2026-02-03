import React from "react";
import ReactDOM from "react-dom/client";
import { ConfigProvider, theme } from "antd";
import zhCN from "antd/locale/zh_CN";
import App from "./App";
import UpdateChecker from "./components/UpdateChecker";
import "./styles/global.css";

// 统一的主题色
const primaryColor = "#6366f1";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ConfigProvider
      locale={zhCN}
      theme={{
        algorithm: theme.darkAlgorithm,
        token: {
          colorPrimary: primaryColor,
          colorBgContainer: "rgba(30, 41, 59, 0.5)",
          colorBgElevated: "rgba(30, 41, 59, 0.95)",
          colorBorder: "rgba(99, 102, 241, 0.2)",
          colorBorderSecondary: "rgba(99, 102, 241, 0.15)",
          borderRadius: 8,
          colorText: "rgba(255, 255, 255, 0.85)",
          colorTextSecondary: "rgba(255, 255, 255, 0.65)",
          colorTextTertiary: "rgba(255, 255, 255, 0.45)",
          colorTextQuaternary: "rgba(255, 255, 255, 0.25)",
          colorBgBase: "#0a0e1a",
          colorFillSecondary: "rgba(99, 102, 241, 0.1)",
          colorFillTertiary: "rgba(99, 102, 241, 0.08)",
          motionDurationMid: "0.2s",
          motionDurationSlow: "0.3s",
        },
        components: {
          Input: {
            activeBorderColor: primaryColor,
            hoverBorderColor: "rgba(99, 102, 241, 0.5)",
            activeShadow: `0 0 0 2px rgba(99, 102, 241, 0.2)`,
          },
          Select: {
            optionSelectedBg: "rgba(99, 102, 241, 0.2)",
          },
          Table: {
            headerBg: "rgba(30, 41, 59, 0.8)",
            rowHoverBg: "rgba(99, 102, 241, 0.1)",
          },
          Button: {
            primaryShadow: "0 0 20px rgba(99, 102, 241, 0.3)",
          },
          Modal: {
            contentBg: "rgba(30, 41, 59, 0.95)",
            headerBg: "transparent",
          },
        },
      }}
    >
      <App />
      <UpdateChecker />
    </ConfigProvider>
  </React.StrictMode>,
);
