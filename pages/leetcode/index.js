import fs from "fs";
import path from "path";
import matter from "gray-matter";
import LCProblemTable from "../../components/LCProblemTable";


export default function LeetCode({ problems }) {
  return <LCProblemTable problems={problems}/>
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
      frontmatter
    };
  });
  return {
    props: {
      problems: posts
    },
  };
}
