import { autocompleteClasses, Typography } from "@mui/material";
import { Container } from "@mui/system";

export default function Footer() {
  return (
    // <footer>
    <Container sx={{
      marginTop: 'auto',
      textAlign: 'center'
    }}>
      <Typography> &copy; 2023 Will Wang</Typography>
    </Container>
    // </footer>
  );
}
