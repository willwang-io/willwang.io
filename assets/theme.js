import { createTheme } from "@mui/material/styles";
import { blue } from '@mui/material/colors';


export const lightTheme = createTheme({
  palette: {
    mode: 'light'
  },
  typography: {
    fontFamily: "equity_a",
    htmlFontSize: 16,
    fontSize: 14,
    body1: {
      letterSpacing: "0.05em",
      marginBottom: '1em',
      marginTop: '0.5em',
      lineHeight: 1.35,
      fontFeatureSettings: `"liga" 1, "onum" 1`,
      // color: '#4C4E52',
    },
    caption: {
      fontFamily: "equity_a_caps",
      fontSize: '1em',
    },
    h1: {
      fontSize: '2em',
      marginBottom: '5px'
    },
    h2: {
      fontSize: '1.5em',
      marginBottom: '5px'
    },
    h3: {
      fontSize: '1.25em',
      marginBottom: '5px'
    },
    h4: {
      fontSize: '1.12em',
      marginBottom: '5px'
    },
    h5: {
      fontSize: '1.06em',
      marginBottom: '5px'
    },
    h6: {
      fontSize: '1.03em',
      marginBottom: '5px'
    },
  },
  components: {
    MuiLink: {
      styleOverrides: {
        root: {
          fontFamily: 'equity_a_caps',
          fontFeatureSettings: `"ss10" 1`,
          textDecoration: 'none',
          transition: '0.3s',
          color: '#000000',
          '&:hover': {
            background: blue[50],
            textDecoration: 'none',
            paddingBottom: 1,
            borderRadius: '5px'
          },
          '&:active': {
            color: '#808080'
          }
        },
      }
    }
  }
});

export const darkTheme = createTheme({
  palette: {
    mode: 'dark'
  },
  typography: {
    fontFamily: "equity_a",
    htmlFontSize: 16,
    fontSize: 14,
    body1: {
      letterSpacing: "0.05em",
      marginBottom: '1em',
      marginTop: '0.5em',
      lineHeight: 1.35,
      fontFeatureSettings: `"liga" 1, "onum" 1`,
      // color: '#4C4E52',
    },
    caption: {
      fontFamily: "equity_a_caps",
      fontSize: '1em',
    },
    h1: {
      fontSize: '2em',
      marginBottom: '5px'
    },
    h2: {
      fontSize: '1.5em',
      marginBottom: '5px'
    },
    h3: {
      fontSize: '1.25em',
      marginBottom: '5px'
    },
    h4: {
      fontSize: '1.12em',
      marginBottom: '5px'
    },
    h5: {
      fontSize: '1.06em',
      marginBottom: '5px'
    },
    h6: {
      fontSize: '1.03em',
      marginBottom: '5px'
    },
  },
  components: {
    MuiLink: {
      styleOverrides: {
        root: {
          fontFamily: 'equity_a_caps',
          fontFeatureSettings: `"ss10" 1`,
          textDecoration: 'none',
          transition: '0.3s',
          color: '#ffffff',
          '&:hover': {
            background: blue[800],
            textDecoration: 'none',
            paddingBottom: 1,
            borderRadius: '5px'
          },
          '&:active': {
            color: '#ffffff'
          }
        },
      }
    }
  }
});