// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Header, { SearchResult } from "./components/Header";
import Sidebar, { Item } from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { useEffect, useState } from "react";

function App() {
  const [ searchResults, setSearchResult ] = useState<Item[]>([]);

  // (async () => {
  //   console.log('unregister')
  //   await unregisterAll()

  //   console.log('register')
  //   // 既に登録されていたら登録しない処理が必要
  //   await registerAll(["CommandOrControl+Shift+C"], (shortcut) => {
  //     alert(`Shortcut ${shortcut} triggered`);
  //   });
  // })();

  useEffect(() => {
    console.log('mounted')

    // // 初期データとしてdocsets一覧
    // invoke('docsets').then((docsets) => {
    //   // console.log(docsets)

    //   if(docsets instanceof Array<string>) {
    //     setSearchResult(docsets.map((name: string) => { return { name: name, link: '' } }))
    //   }
    // })
  }, [])

  const updateResult = (result: SearchResult) => {
    setSearchResult(result.indices)
  }

  return (
    <div className="app">
      <Header searchHandler={updateResult}/>
      <Sidebar items={searchResults}/>
      <Content />
    </div>
  );
}

export default App;
