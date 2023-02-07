import fs from "fs";
import matter from "gray-matter";
import path from "path";
import LCSolution from "../../components/LCSolution";

export default function PostPage({ post }) {
  const { _, frontmatter, content} = post;
  return <LCSolution content={content} frontmatter={frontmatter}/>
}

export async function getStaticPaths() {
  const files = fs.readdirSync("posts/leetcode");
  const paths = files.map((filename) => ({
    params: {
      problemId: filename.replace(".md", ""),
    },
  }));
  return {
    paths,
    fallback: false,
  };
}

export async function getStaticProps({ params: { problemId } }) {
  const metaInfo = fs.readFileSync(
    path.join("posts/leetcode", problemId + ".md"),
    "utf-8"
  );
  const { data: frontmatter, content } = matter(metaInfo);

  return {
    props: {
      post: {
        problemId,
        frontmatter,
        content,
      },
    },
  };
}
