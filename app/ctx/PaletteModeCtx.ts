import type { PaletteMode } from "@mui/material";
import { type Dispatch, type SetStateAction, createContext } from "react";

export interface PaletteModeCtxType {
  mode: PaletteMode;
  setMode: Dispatch<SetStateAction<PaletteMode>>;
}

export const PaletteModeCtx = createContext<PaletteModeCtxType | undefined>(
  undefined,
);
