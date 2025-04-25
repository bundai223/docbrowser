import type { SearchIndex } from "@/components/Header";
import Icon from "@/components/atoms/Icon";
import "./MenuItem.css";

type Props = {
  item: SearchIndex;
};

function MenuItem(props: Props) {
  return (
    <a className="f-container" href={props.item.html_path}>
      <Icon name={props.item.doctype} />
      <div className="name">{props.item.name}</div>
    </a>
  );
}

export default MenuItem;
