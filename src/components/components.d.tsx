import { PropsWithChildren as ReactPropsWithChildren } from "react"

export type MouseEventHandler = React.MouseEventHandler<HTMLElement>
// export type PropsWithChildren<P> = P & { children?: ReactNode } // これreactにあります
export type PropsWithChildren = ReactPropsWithChildren;
