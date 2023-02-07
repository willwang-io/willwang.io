import Typography from "@mui/material/Typography";
import SellIcon from "@mui/icons-material/Sell";
import LaunchIcon from "@mui/icons-material/Launch";
import Link from "@mui/material/Link";
import Stack from "@mui/material/Stack";
import { styled } from "@mui/material/styles";
import LCDiffBadge from "./LCDiffBadge";

const Item = styled("div")(({ theme }) => ({
  boxShadow: "0",
  padding: theme.spacing(1),
  textAlign: "center",
}));

export default function LCMetaInfo({ metaInfo }) {
  const problemLink = metaInfo.title.toLowerCase().replace(/\s+/g, "-");
  return (
    <Stack direction={"row"} spacing={0}>
      <Item>
          <LCDiffBadge diff={metaInfo.diff}/>
      </Item>
      <Item>
        <Typography>
          <SellIcon sx={{ fontSize: "medium", marginRight: 1 }} />
          {metaInfo.tags.join(", ")}
        </Typography>
      </Item>
      <Item>
        <Typography>
        <LaunchIcon sx={{ fontSize: "medium", marginRight: 1 }} />
          <Link href={`https://leetcode.com/problems/${problemLink}`}>
            Problem Link
          </Link>
        </Typography>
      </Item>
    </Stack>
  );
}
