import Content from "@/components/Content";
import Header, { type SearchResult } from "@/components/Header";
import Sidebar, { type Item } from "@/components/Sidebar";
import { Config, type RouteSetter } from "@/routes";
import { useState } from "react";

type Props = {
  setRoute: RouteSetter;
};

export function SearchPage(props: Props) {
  const [searchResults, setSearchResult] = useState<Item[]>([]);

  const updateResult = (result: SearchResult) => {
    setSearchResult(result.indices);
  };

  const toConfig = () => {
    console.log("to config.");
    props.setRoute(Config);
  };

  return (
    <div className="search">
      <Header searchHandler={updateResult} toConfigHandler={toConfig} />
      <Sidebar items={searchResults} />
      <Content />
    </div>
  );
}

export default SearchPage;
