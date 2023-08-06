import fs from "fs";
import matter from "gray-matter";

import Container from "@mui/system/Container";
import MetaInfo from "../components/MetaInfo";
import MyMarkdown from "../components/MyMarkdown";

export default function Post({ postData }) {
  const {frontmatter, content} = postData;
  return (
    <Container>
      <MetaInfo frontmatter={frontmatter}></MetaInfo>
      <MyMarkdown>{content}</MyMarkdown>
      <div>Last edited: {frontmatter.date}</div>
    </Container>
  );
}

export async function getPostData() {
  const fileContents = fs.readFileSync('posts/delaunay.md', 'utf8');
  const {data: frontmatter, content} = matter(fileContents);
  return {
    frontmatter,
    content,
  }
}

export async function getStaticProps() {
  const postData = await getPostData();
  return {
    props: {
      postData,
    }
  }
}
