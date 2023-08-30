import { ReactNode } from "react"

export type MouseEventHandler = React.MouseEventHandler<HTMLElement>
export type PropsWithChildren<P> = P & { children?: ReactNode }
