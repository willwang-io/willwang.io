import fs from "fs";
import path from "path";
import matter from "gray-matter";
import LCMainTable from "../../components/LCMainTable";
import Container from "@mui/system/Container";
import Typography from "@mui/material/Typography";

export default function LeetCode({ problems }) {
  return (
    <Container>
      <Typography variant="h5">Give up LeetCode</Typography>
      <Typography>
        Just a mediocre Knight struggling to achieve Guardian some day...
      </Typography>
      <LCMainTable problems={problems} />
    </Container>
  );
}

export async function getStaticProps() {
  const files = fs.readdirSync("posts/leetcode");

  const posts = files.map((filename) => {
    const slug = filename.replace(".md", "");
    const metaInfo = fs.readFileSync(
      path.join("posts/leetcode", filename),
      "utf-8"
    );
    const { data: frontmatter } = matter(metaInfo);

    return {
      frontmatter,
    };
  });
  return {
    props: {
      problems: posts,
    },
  };
}
