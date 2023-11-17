import { SearchIndex } from "@/components/Header";
import Icon from "@/components/atoms/Icon";
import "./MenuItem.css"
import { MouseEventHandler } from "@/components/components.d";

export type ClickedHandler = (index: SearchIndex) => void

type Props = {
    item: SearchIndex;
    clickedHandler: ClickedHandler;
}

function MenuItem(props: Props) {
    const MenuItemClickedHandler: MouseEventHandler = (e) => {
        // console.log(e.currentTarget.getAttribute('href'))
        props.clickedHandler(props.item)

        e.preventDefault()
    }


    return <a className="f-container" href={props.item.html_path} onClick={MenuItemClickedHandler}>
        <Icon name={props.item.doctype}></Icon>
        <div className="name">
            {props.item.name}
        </div>
    </a>
}

export default MenuItem;
