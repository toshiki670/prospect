import { JSX, useState } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { type PaletteMode } from "@mui/material";
import { CssBaseline } from "@mui/material";
import { PaletteModeCtx } from "@/ctx/PaletteModeCtx";
import { Outlet } from "react-router";


export default function CustomTheme(): JSX.Element {
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
        <Outlet />
      </ThemeProvider>
    </PaletteModeCtx.Provider>
  );
};
