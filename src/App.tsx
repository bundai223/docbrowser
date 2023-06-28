// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Sidebar from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll } from '@tauri-apps/api/globalShortcut';

function App() {
  // const [greetMsg, setGreetMsg] = useState("");
  // const [name, setName] = useState("");

  // async function greet() {
  //   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  //   setGreetMsg(await invoke("greet", { name }));
  // }

  (async () => {
    // 既に登録されていたら登録しない処理が必要
    await registerAll(['CommandOrControl+Shift+C', 'CommandOrControl+Shift+1'], (shortcut) => {
      alert(`Shortcut ${shortcut} triggered`);
    });
  })()

  return (
    <div>
      <h1 className="text-3xl font-bold underline">
        Hello DocBrowser!
      </h1>
      <Sidebar />
      <Content />
    </div>
  );
}

export default App;
