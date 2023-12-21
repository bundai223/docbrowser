// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import Header, { SearchIndex, SearchResult } from "./components/Header";
import Sidebar, { Item, MenuClickedHandler } from "./components/Sidebar";
import Content from "./components/Content";
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { useEffect, useState } from "react";
import parse from 'html-react-parser';
import { tauriClient } from "./client";

function App() {
  const [ searchResults, setSearchResult ] = useState<Item[]>([]);
  const [ content, setContent ] = useState<string>('this is state content.');
  const [ selectedAnchor, setSelectedAnchor ] = useState<string | null>(null);

  const menuClicked: MenuClickedHandler = (index: SearchIndex) => {
    console.log(index)

    const file_path = index.html_path.split('#')[0]
    const anchor = index.html_path.split('#')[1]
    tauriClient.query(['app.read_html', { docset_name: index.docset_name, rel_path: file_path }]).then((html) => {
      const c = `<p>${html}</p>`
      setContent(c)
      setSelectedAnchor(anchor)
    })
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

  useEffect(() => {
    console.log(`updated content or anchor: ${selectedAnchor}`)

    // $('dashAnchor[=selectedAnchor]')までスクロールする
    const offset = 0;
    const anchor = document.querySelector(`.dashAnchor[name="${selectedAnchor}"]`);
    if (!anchor) {
      return
    }
    const offsetTop = anchor.getBoundingClientRect().top + window.scrollY;
    window.scroll({
      top: offsetTop - offset,
      behavior: 'instant'
    })
  }, [content, selectedAnchor])

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
