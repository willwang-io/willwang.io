import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";
import { darkTheme, lightTheme } from "../assets/theme";
import { ThemeProvider } from "@mui/material/styles";
import CssBaseline from "@mui/material/CssBaseline";
import { useEventCallback } from "@mui/material";

const ColorModeContext = createContext({ toggleColorMode: () => {} });

export function useModeContext() {
  return useContext(ColorModeContext);
}

function useLocalStorage(key, fallbackValue) {
  const [value, setValue] = useState(fallbackValue);
  useEffect(() => {
    const stored = localStorage.getItem(key);
    setValue(stored ? JSON.parse(stored) : fallbackValue);
  }, [fallbackValue, key]);

  useEffect(() => {
    localStorage.setItem(key, JSON.stringify(value));
  }, [key, value]);

  return [value, setValue];
}

export default function ThemeContext({ children }) {
  const [isDarkMode, setDarkMode] = useLocalStorage("theme", false);

  const colorMode = {
    toggleColorMode: () => {
      setDarkMode((prevMode) => !prevMode);
    },
  };

  return (
    <ColorModeContext.Provider value={colorMode}>
      <ThemeProvider theme={isDarkMode ? darkTheme : lightTheme}>
        <CssBaseline />
        {children}
      </ThemeProvider>
    </ColorModeContext.Provider>
  );
}
