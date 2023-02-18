import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import { Link } from "@mui/material";
import ToggleMode from "./ToggleMode";

const Header = () => {
  return (
    <Box>
      <AppBar
        position="static"
        sx={{ background: "transparent", boxShadow: "none" }}
      >
        <Toolbar>
          <Typography variant="h3" sx={{ flexGrow: 1 }}>
            <Link href="/">Will&rsquo;s Log</Link>
          </Typography>
          <Typography variant="h3">
            <Link href="/about">About</Link>
          </Typography>
          <ToggleMode />
        </Toolbar>
      </AppBar>
    </Box>
  );
};

export default Header;
