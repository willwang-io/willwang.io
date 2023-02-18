import Container from "@mui/material/Container";
import Header from "../components/Header";
import Footer from "../components/Footer";
import "../styles/globals.css";
import ThemeContext from "../components/ColorModeContext";

const App = ({ Component, pageProps }) => {
  return (
    <ThemeContext>
      <Container
        maxWidth="md"
        sx={{
          display: "flex",
          minHeight: "100vh",
          flexDirection: "column",
          justifyContent: "flex-start",
        }}
      >
        <Header />
        <Component {...pageProps} />
        <Footer />
      </Container>
    </ThemeContext>
  );
};

export default App;