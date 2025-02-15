import { JSX, useState } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { type PaletteMode } from "@mui/material";
import { CssBaseline } from "@mui/material";
import { PaletteModeCtx } from "@/ctx/PaletteModeCtx";

// import { getSettings } from "@/tauri/command";

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

  return (
    <PaletteModeCtx.Provider value={{ mode, setMode }}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </PaletteModeCtx.Provider>
  );
};
