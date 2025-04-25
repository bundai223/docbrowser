import "./icon.css";

type Props = {
  name: string;
};

function Icon(props: Props) {
  return <div className="icon">{props.name[0]}</div>;
}

export default Icon;
