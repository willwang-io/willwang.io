import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import { IconButton, Link } from "@mui/material";
import LightModeIcon from '@mui/icons-material/LightMode';
import ToggleMode from "./ToggleMode";

export default function Navbar() {
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
          {/* <ToggleMode/> */}
          {/* <IconButton sx={{marginLeft: 1}}>
            <LightModeIcon/>
          </IconButton> */}
        </Toolbar>
      </AppBar>
    </Box>
  );
}
