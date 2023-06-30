import "./Sidebar.css";

type Item = {
  name: string
  link: string
};

function Sidebar() {

  const items: Item[] = [
    { name: 'aaaa', link: '' },
    { name: 'bbbb', link: '' }
  ]

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
