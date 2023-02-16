import { ThemeProvider } from "@mui/material/styles";
import Navbar from "../components/Navbar";
import Container from "@mui/system/Container";
import theme from "../assets/theme";

import "../styles/globals.css";
import Footer from "../components/Footer";

export default function App({ Component, pageProps }) {
  return (
    <ThemeProvider theme={theme}>
      <Container maxWidth="md" sx={{display: 'flex', minHeight: '100vh', flexDirection: 'column', justifyContent: 'flex-start'}}>
        <Navbar />
        <Component {...pageProps} />
        <Footer/>
      </Container>
    </ThemeProvider>
  );
}
