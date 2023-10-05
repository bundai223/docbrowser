// import React, { useState } from "react";
// import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import SearchPage from "@/components/pages/search_page";
import { registerAll, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { useState, useEffect } from "react";
import { Route, Search } from "@/routes";
import ConfigTop from "./components/pages/config_top";

function App() {
  const [ route, setRoute ] = useState<Route>(Search);

  const routeSetter = (route: Route) => setRoute(route)

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

  const page = (route: Route) => {
    switch(route) {
      case Search:
        return <SearchPage setRoute={routeSetter} />
      default:
        return <ConfigTop setRoute={routeSetter} />
    }
  }

  return (
    <div className="app">
      {page(route)}
    </div>
  );
}

export default App;
