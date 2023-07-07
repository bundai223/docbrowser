import "./Sidebar.css";
import { invoke } from '@tauri-apps/api/tauri'
import { useState } from "react";

type Item = {
  name: string
  link: string
};

function Sidebar() {

  const [items, setItems] = useState<Item[]>([
    { name: 'aaaa', link: '' },
    { name: 'bbbb', link: '' }
  ])

  invoke('docsets').then((docsets) => {
    console.log(docsets)

    if(docsets instanceof Array<string>) {
      setItems(
        docsets.map((name: string) => { return { name: name, link: '' } })
      )
    }
  })

  const list = items.map((item) =>
    <li key={item.name}>
      {item.name}
    </li>
  )

  return (
    <div className="sidebar">
      <ul>
        {list}
      </ul>
    </div>
  );
}

export default Sidebar;
