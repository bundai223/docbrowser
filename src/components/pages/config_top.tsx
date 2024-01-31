import { RouteSetter, Search } from "@/routes";
import Button from "../atoms/Button";
import DocsetList from "../organisms/DocsetList";
import { tauriClient } from "@/client";
import { useEffect, useState } from "react";
import { Docset } from "@/types/rspc/bindings";

type Props = {
    setRoute: RouteSetter
}

function ConfigTop(props: Props) {
  const [docsets, setDocsets] = useState<Docset[]>([])
  console.log('Config top')

  const toSearch = () => {
    console.log('to search.')
    props.setRoute(Search)
  }

  useEffect(() =>
  {
    tauriClient.query(['app.docsets']).then(
      d => {
        console.log('hoge')
        setDocsets(d)
      }
    ).catch(e => console.log(e))
  })

  return (
    <div className="config">
      this is config top
      <Button onClick={toSearch}>back</Button>
      <DocsetList docsets={docsets}/>
    </div>
  );
}

export default ConfigTop;
