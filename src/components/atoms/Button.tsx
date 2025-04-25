import type { MouseEventHandler, PropsWithChildren } from "../components.d";

type Props = {
  onClick: MouseEventHandler;
};

function Button({ onClick, children }: PropsWithChildren<Props>) {
  return (
    <button type="button" onClick={onClick}>
      {children}
    </button>
  );
}

export default Button;
