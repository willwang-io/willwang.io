import Typography from "@mui/material/Typography";
import Container from "@mui/system/Container";

import MyMarkdown from "./MyMarkdown";
import LCMetaInfo from "./LCMetaInfo";
import Breadcrumbs from "@mui/material/Breadcrumbs";
import { Link } from "@mui/material";
import Code from "./Code";

export default function LCSolution({ frontmatter, content }) {
  return (
    <Container sx={{ margin: 2 }}>
      <Breadcrumbs arial-label="breadcrumb" separator="/">
        <Link href="/">Home</Link>
        <Link href="/leetcode">LeetCode</Link>
        <Typography>{frontmatter.id}</Typography>
      </Breadcrumbs>
      <Typography variant="h2">
        {frontmatter.id}. {frontmatter.title}
      </Typography>

      <LCMetaInfo metaInfo={frontmatter} />
      <Typography>Some notices: </Typography>
      <ul>
        <li>
          <Typography>
            The robot can start at any empty cell, so the coordinate
            <Code>(i, j)</Code>
            is for reference only, because we need to track which cells we
            already visited
          </Typography>
        </li>
        <li>
          <Typography>
            You know for DFS, we sometimes need to restore to the previous state
          </Typography>
        </li>
        <li>
          <Typography>Turn right three times is same as turn left</Typography>
        </li>
      </ul>
      <Typography>
        At current position, can we move to <Code>(ci, cj)</Code>? If no, we
        turn right and check again, otherwise, we move to <Code>(ci, cj)</Code>.
        After we finish, we need to return to the previous state. Imagine we landed 
        on <Code>(ci, cj)</Code> from the right tile, so turn 180 degrees, move 
        to that left tile, and turn 180 degree again. 
      </Typography>
      <Code is_block={true} lang={"cpp"}>
        {`int di[4] = {-1, 0, 1, 0}, dj[4] = {0, 1, 0, -1};

void cleanRoom(Robot& robot) {
    unordered_set<string> seen;
    solve(robot, 0, 0, 0, seen);
}

void solve(Robot &robot, int i, int j, int cur_dir, 
           unordered_set<string> &seen) {
    robot.clean();
    seen.insert(to_string(i) + ':' + to_string(j));
    for (int k = 0; k < 4; k++) {
        int ci = i + di[cur_dir], cj = j + dj[cur_dir];
        string token = to_string(ci) + ':' + to_string(cj);
        if (seen.find(token) == seen.end() && robot.move()) {
            solve(robot, ci, cj, cur_dir, seen);
            robot.turnRight();
            robot.turnRight();
            robot.move();
            robot.turnRight();
            robot.turnRight();
        }
        cur_dir = (cur_dir + 1) % 4;
        robot.turnRight();
    }
}`}
      </Code>
      {/* <MyMarkdown>{content}</MyMarkdown> */}
    </Container>
  );
}
