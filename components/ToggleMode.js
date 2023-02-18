import IconButton from "@mui/material/IconButton";
import DarkModeIcon from "@mui/icons-material/DarkMode";
import LightModeIcon from "@mui/icons-material/LightMode";
import { useModeContext } from "./ColorModeContext";

import { useTheme } from "@mui/material/styles";

export default function ToggleColorMode() {
  const colorMode = useModeContext()
  const theme = useTheme()

  return (
    <IconButton onClick={colorMode.toggleColorMode}>
      {theme.palette.mode === 'dark' ? <DarkModeIcon /> : <LightModeIcon />}
    </IconButton>
  );
}
