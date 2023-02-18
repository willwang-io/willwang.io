import { createContext, useContext, useMemo, useState } from "react";
import { darkTheme, lightTheme } from "../assets/theme";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";

const ColorModeContext = createContext({ toggleColorMode: () => {} });

export function useModeContext() {
  return useContext(ColorModeContext);
}

export default function ThemeContext({ children }) {
  const [isDarkMode, setIsDarkMode] = useState(false);
  const colorMode = useMemo(
    () => ({
      toggleColorMode: () => {
        setIsDarkMode((prevMode) => !prevMode);
      },
    }),
    []
  );
  const theme = isDarkMode ? darkTheme : lightTheme;

  return (
    <ColorModeContext.Provider value={colorMode}>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </ColorModeContext.Provider>
  );
}
