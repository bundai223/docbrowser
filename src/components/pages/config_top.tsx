import { tauriClient } from "@/client";
import { type RouteSetter, Search } from "@/routes";
import type { Docset } from "@/types/rspc/bindings";
import { core } from "@tauri-apps/api";
import { useEffect, useState } from "react";
import Button from "../atoms/Button";
import DocsetList from "../organisms/DocsetList";

type Props = {
  setRoute: RouteSetter;
};

function ConfigTop(props: Props) {
  const [docsets, setDocsets] = useState<Docset[]>([]);
  console.log("Config top");

  const toSearch = () => {
    console.log("to search.");
    props.setRoute(Search);
  };

  function downloadDocset(docset: Docset) {
    tauriClient
      .mutation([
        "app.download_docset",
        {
          name: docset.name,
          feed_url: docset.feed_url,
        },
      ])
      .then(() => console.log("download done"))
      .catch((e) => console.log(e));
  }

  useEffect(() => {
    tauriClient
      .query(["app.docsets"])
      .then((d) => {
        setDocsets(d);
      })
      .catch((e) => console.log(e));
  }, []);

  return (
    <div className="config">
      this is config top
      <Button onClick={toSearch}>back</Button>
      <DocsetList
        docsets={docsets}
        downloadDocsetHandler={(docset) => {
          downloadDocset(docset);
        }}
      />
    </div>
  );
}

export default ConfigTop;
