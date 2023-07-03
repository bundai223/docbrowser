import { ReactNode } from "react"

export type MouseEventHandler = React.MouseEventHandler<HTMLButtonElement>
export type PropsWithChildren<P> = P & { children?: ReactNode }
