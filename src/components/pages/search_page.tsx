import Header, { SearchResult } from "@/components/Header";
import Sidebar, { Item } from "@/components/Sidebar";
import Content from "@/components/Content";
import { useState } from "react";
import { RouteSetter, Config } from "@/routes";

type Props = {
    setRoute: RouteSetter
}

export function SearchPage(props: Props) {
  const [ searchResults, setSearchResult ] = useState<Item[]>([]);

  const updateResult = (result: SearchResult) => {
    setSearchResult(result.indices)
  }

  const toConfig = () => {
    console.log('to config.')
    props.setRoute(Config)
  }

  return (
    <div className="search">
      <Header searchHandler={updateResult} toConfigHandler={toConfig}/>
      <Sidebar items={searchResults}/>
      <Content />
    </div>
  );
}

export default SearchPage;
