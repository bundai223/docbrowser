import { Docset } from "@/types/rspc/bindings"
import Button from "../atoms/Button"

type Props = {
    docsets: Docset[],
    downloadDocsetHandler: (docset: Docset) => void
}

function DocsetList(props: Props) {
    function onDownloadClicked(clickedDocset: Docset) {
        console.log(`DL Clicked: ${clickedDocset.toString()}`)
        props.downloadDocsetHandler(clickedDocset)
    }

    const docsets = props.docsets.map(
        d => {
            const download = d.downloaded ? '○' : <Button onClick={(e) => onDownloadClicked(d)}>DL</Button>
            return <tr key={d.name}>
                <td>{d.name}</td>
                <td>{download}</td>
            </tr>
        }
    )

    return <>
        <table>
            <th>
                <td>name</td>
                <td>status</td>
            </th>
            <tbody>
                {docsets}
            </tbody>
        </table>
    </>
}

export default DocsetList