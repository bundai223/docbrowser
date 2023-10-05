import { RouteSetter, Search } from "@/routes";
import Button from "../atoms/Button";

type Props = {
    setRoute: RouteSetter
}

function ConfigTop(props: Props) {
  const toConfig = () => {
    console.log('to config.')
    props.setRoute(Search)
  }

  return (
    <div className="config">
      this is config top
      <Button onClick={toConfig}>back</Button>
    </div>
  );
}

export default ConfigTop;
