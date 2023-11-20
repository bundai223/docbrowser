// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Header, { SearchIndex, SearchResult } from "./components/Header";
import Sidebar, { Item, MenuClickedHandler } from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { useEffect, useState } from "react";
import parse from 'html-react-parser';

function App() {
  const [ searchResults, setSearchResult ] = useState<Item[]>([]);
  const [ content, setContent ] = useState<string>('this is state content.');

  const menuClicked: MenuClickedHandler = (index: SearchIndex) => {
    console.log(index)
    const c = `<p>${index.html_path}</p>`
    setContent(c)
  }
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
      <Sidebar items={searchResults} menuClicked={menuClicked}/>
      <Content>
          {parse(content)}
      </Content>
    </div>
  );
}

export default App;
