import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Link from "@mui/material/Link";

export default function Navbar() {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <AppBar
        position="static"
        sx={{ background: "transparent", boxShadow: "none" }}
      >
        <Toolbar>
          <Typography variant="h4" component="div" sx={{ flexGrow: 1 }}>
            <Link href="/" sx={{ color: "black" }}>
              Will&rsquo;s Log
            </Link>
          </Typography>
          <Typography variant="h6">
            <Link href="/about" sx={{ color: "black" }}>
              About
            </Link>
          </Typography>
        </Toolbar>
      </AppBar>
    </Box>
  );
}
