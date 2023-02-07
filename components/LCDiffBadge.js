import { styled } from "@mui/material/styles";
import { green, orange, red } from "@mui/material/colors";
import { Typography } from "@mui/material";

const Badge = styled("div")(({ theme }) => ({
  color: "white",
  paddingLeft: 5,
  paddingRight: 5,
  borderRadius: 15,
}));

export default function LCDiffBadge({ diff }) {
  const color =
    diff == "Hard" ? red[400] : diff == "Medium" ? orange[400] : green[400];
  return <Badge sx={{background: color}}><Typography>{diff}</Typography></Badge> 
}
