import { SearchIndex } from "@/components/Header";
import Icon from "@/components/atoms/Icon";
import "./MenuItem.css"

type Props = {
    item: SearchIndex;
}

function MenuItem(props: Props) {
    return <div className="f-container">
        <Icon name={props.item.doctype}></Icon>
        <div className="name">
            {props.item.name}
        </div>
    </div>
}

export default MenuItem;