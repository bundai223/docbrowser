import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

import { registerAll } from '@tauri-apps/api/globalShortcut';
await registerAll(['CommandOrControl+Shift+C', 'Control+C', 'c', 'C'], (shortcut) => {
  console.log(`Shortcut ${shortcut} triggered`);
});
