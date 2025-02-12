import { HelmetProvider } from "react-helmet-async";
import { BrowserRouter } from "react-router";

import { CustomThemeProvider } from "./CustomThemeProvider";
import { JSX } from "react";

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers = ({ children }: ProvidersProps): JSX.Element => {
  return (
    <CustomThemeProvider>
      <HelmetProvider>
        <BrowserRouter>{children}</BrowserRouter>
      </HelmetProvider>
    </CustomThemeProvider>
  );
};
