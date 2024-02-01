import { SearchIndex } from "@/components/Header";
import Icon from "@/components/atoms/Icon";
import "./MenuItem.css"

type Props = {
    item: SearchIndex;
}

function MenuItem(props: Props) {
    return <a className="f-container" href={props.item.htmlpath}>
        <Icon name={props.item.doctype}></Icon>
        <div className="name">
            {props.item.name}
        </div>
    </a>
}

export default MenuItem;
