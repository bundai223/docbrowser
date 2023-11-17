import "./Content.css";
import { PropsWithChildren } from "./components.d";

type Props = PropsWithChildren;

function Content(props: Props) {
  return (
    <div className="container">
      <div className="content">
        <div className="intro">
          {props.children}
        </div>
      </div>
    </div>
  );
}

export default Content;
