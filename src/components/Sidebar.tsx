import { SearchIndex } from "./Header";
import "./Sidebar.css";
import MenuItem, { ClickedHandler } from "./molecules/MenuItem";

export type Docset = {
  name: string
};

export type Item = Docset | SearchIndex;
export type MenuClickedHandler = ClickedHandler;

export type Props = {
  items: Item[]
  menuClicked: MenuClickedHandler
}

function isSearchResult(item: Item): item is SearchIndex {
  return (item as SearchIndex).html_path != undefined;
}

function Sidebar(props: Props) {
  // docsets directory
  // TypeScript.docset/Contents/Resources/Documents
  function itemSelectedHandler(item: SearchIndex): void {
    props.menuClicked(item)
  }

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
