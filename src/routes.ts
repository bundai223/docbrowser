// Routes
export const Search = Symbol();
export const Config = Symbol();

export type Route = typeof Search | typeof Config;
export type RouteSetter = (Route: Route) => void;
