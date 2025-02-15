import {
  index,
  route,
  layout,
  type RouteConfig,
} from "@react-router/dev/routes";

export default [
  layout("layouts/app.tsx", [
    index("routes/home.tsx"),
    route("overview", "routes/overview.tsx"),
  ]),
] as RouteConfig;
