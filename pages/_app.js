import { ThemeProvider } from "@mui/material/styles";
import Navbar from "../components/Navbar";
import Container from "@mui/system/Container";
import theme from "../assets/theme";
import GlobalStyles from "@mui/material/GlobalStyles";

import '../styles/globals.css';


export default function App({Component, pageProps}) {
  return (
    <ThemeProvider theme={theme}>
      <GlobalStyles styles={{
        p: {
          textRendering: 'optimizeLegibility',
        }
      }}/>
      <Container maxWidth="md">
          <Navbar/>
          <Component {...pageProps}/>
        </Container>
    </ThemeProvider>
  );
}