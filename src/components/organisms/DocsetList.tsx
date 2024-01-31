import { Docset } from "@/types/rspc/bindings"

type Props = {
    docsets: Docset[]
}

function DocsetList(props: Props) {
    let docsets = props.docsets.map(d => <li>{d.name}, {d.downloaded ? '○': '×'}</li>)

    return <>
        <ul>
            {docsets}
        </ul>
    </>
}

export default DocsetList