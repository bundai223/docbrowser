import "./Sidebar.css";

export type Item = {
  name: string
  link: string
};

export type Props = {
  items: Item[]
}

function Sidebar(props: Props) {

  const list = props.items.map((item) =>
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
