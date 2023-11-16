import { SearchIndex } from "./Header";
import "./Sidebar.css";
import MenuItem from "./molecules/MenuItem";

export type Docset = {
  name: string
};

export type Item = Docset | SearchIndex;

export type Props = {
  items: Item[]
}

// docsets directory
// TypeScript.docset/Contents/Resources/Documents
function itemSelectedHandler(item: SearchIndex): void {
  console.log(item)
}

function isSearchResult(item: Item): item is SearchIndex {
  return (item as SearchIndex).html_path != undefined;
}

function Sidebar(props: Props) {
  const list = props.items.map((item) =>
    {
      if (isSearchResult(item)) {
        return <MenuItem key={item.id} item={item} clickedHandler={itemSelectedHandler}></MenuItem>;
      } else {
        return <div key={item.name}>
          {item.name}
        </div>
      }
    }
  )

  return (
    <div className="sidebar">
      {list}
    </div>
  );
}

export default Sidebar;
