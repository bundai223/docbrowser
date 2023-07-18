// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Header, { SearchResult } from "./components/Header";
import Sidebar, { Item } from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api";
import { tauriClient } from '@/client'

const getAppName = (): Promise<string> => {
	// return await invoke('get_app_name')
	return tauriClient.query(['app.getAppName']) // エディタ補完が効く
}

function App() {
  const [ searchResults, setSearchResult ] = useState<Item[]>([]);

  (async () => {
    const a = await getAppName();
    console.log('aaaaaaaa' + a)
  })();

  (async () => {
    console.log('unregister')
    await unregisterAll()

    console.log('register')
    // 既に登録されていたら登録しない処理が必要
    await registerAll(["CommandOrControl+Shift+C"], (shortcut) => {
      alert(`Shortcut ${shortcut} triggered`);
    });
  })();

  useEffect(() => {
    console.log('mounted')

    // 初期データとしてdocsets一覧
    invoke('docsets').then((docsets) => {
      // console.log(docsets)

      if(docsets instanceof Array<string>) {
        setSearchResult(docsets.map((name: string) => { return { name: name, link: '' } }))
      }
    })
  }, [])

  const updateResult = (result: SearchResult[]) => {
    console.log('this is test')
    console.log(result)

    setSearchResult(result.map((result) => { return { name: result.name, link: '' }} ))
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
