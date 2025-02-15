
import { Outlet } from "react-router";
import { CustomThemeProvider } from "./CustomThemeProvider";
import { JSX } from "react";


export default function Providers(): JSX.Element {
  return (
    <CustomThemeProvider>
      <Outlet />
    </CustomThemeProvider>
  );
};
