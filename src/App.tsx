// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Header from "./components/Header";
import Sidebar from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll } from '@tauri-apps/api/globalShortcut';

function App() {
  (async () => {
    // 既に登録されていたら登録しない処理が必要
    await registerAll(["CommandOrControl+Shift+C"], (shortcut) => {
      alert(`Shortcut ${shortcut} triggered`);
    });
  })();

  return (
    <div className="app">
      <Header />
      <Sidebar />
      <Content />
    </div>
  );
}

export default App;
