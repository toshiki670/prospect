import { JSX, useEffect, useState } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { type PaletteMode } from "@mui/material";
import CssBaseline from "@mui/material/CssBaseline";
import { PaletteModeCtx } from "@/ctx/PaletteModeCtx";

interface CustomThemeProviderProps {
  children: React.ReactNode;
}

export const CustomThemeProvider = ({
  children,
}: CustomThemeProviderProps): JSX.Element => {
  const [mode, setMode] = useState<PaletteMode>("dark");

  const theme = createTheme({
    palette: {
      mode,
    },
  });

  useEffect(() => {
    void (async () => {
      try {
        const theme = "dark";
        if (theme !== undefined) {
          setMode(theme);
        }
      } catch (error) {
        console.error(error);
      }
    })();
  }, []);

  return (
    <PaletteModeCtx.Provider value={{ mode, setMode }}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </PaletteModeCtx.Provider>
  );
};
