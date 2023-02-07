import { createTheme } from "@mui/material/styles";
import { red } from '@mui/material/colors';


const theme = createTheme({
  palette: {
    primary: {
      main: red[600],
    },
    text: {
    },
    background: {
      default: '#ff0000'
    }
  },
  typography: {
    fontFamily: 'equity_a',
    fontSize: 12.6,
  },
});

export default theme;
