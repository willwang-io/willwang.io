import { Typography } from "@mui/material";
import { Container } from "@mui/system";

export default function Footer() {
  return (
    <footer>
      <Container
        maxWidth={"lg"}
        sx={{
          clear: "both",
          position: "relative",
          width: "100%",
          overflow: "hidden",
          left: 0,
          bottom: 0,
          height: '200px',
          mb: 1,
          mt: 2,
        }}
      >
        <Typography textAlign={"center"}> &copy; 2023 Will Wang</Typography>
      </Container>
    </footer>
  );
}
