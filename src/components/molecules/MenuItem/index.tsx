import { SearchIndex } from "@/components/Header";
import Icon from "@/components/atoms/Icon";
import "./MenuItem.css"
import { MouseEventHandler } from "@/components/components.d";

type Props = {
    item: SearchIndex;
}

const hoge: MouseEventHandler = (e) => {
    console.log(e.currentTarget.getAttribute('href'))
    
    e.preventDefault()
}


function MenuItem(props: Props) {
    return <a className="f-container" href={props.item.htmlpath} onClick={hoge}>
        <Icon name={props.item.doctype}></Icon>
        <div className="name">
            {props.item.name}
        </div>
    </a>
}

export default MenuItem;
