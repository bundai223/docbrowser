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

  class Item {
    name: string;
    link: string;

    constructor(_name: string, _link: string) {
      this.name = _name;
      this.link = _link;
    }
  }

  const links = [
    new Item('hoge', 'hoge'),
    new Item('fuga', 'fuga')
  ].map((item) =>
    <li key={item.name}>
      <a href={item.link}>{item.name}</a>
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
