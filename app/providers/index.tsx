import { CustomThemeProvider } from "./CustomThemeProvider";
import { JSX } from "react";

interface ProvidersProps {
  children: React.ReactNode;
}

export const Providers = ({ children }: ProvidersProps): JSX.Element => {
  return (
    <CustomThemeProvider>
      {children}
    </CustomThemeProvider>
  );
};
