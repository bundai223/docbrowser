import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { registerAll } from '@tauri-apps/api/globalShortcut';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  (async () => {
    await registerAll(['CommandOrControl+Shift+C'], (shortcut) => {
      alert(`Shortcut ${shortcut} triggered`);
    });
  })()

  return (
    <div>
      <h1 className="text-3xl font-bold underline">
        Hello DocBrowser!
      </h1>
    </div>
  );
}

export default App;
