import Typography from "@mui/material/Typography";
import Container from "@mui/system/Container";

import MyMarkdown from "./MyMarkdown";
import LCMetaInfo from "./LCMetaInfo";
import Breadcrumbs from "@mui/material/Breadcrumbs";
import Link from "@mui/material/Link";


export default function LCSolution({ frontmatter, content }) {
  return (
    <Container sx={{margin: 2}}>
      <Breadcrumbs arial-label="breadcrumb" separator="/">
        <Link href="/">
          Home
        </Link>
        <Link href="/leetcode">
          LeetCode
        </Link>
        <Typography>{frontmatter.id}</Typography>
      </Breadcrumbs>
      <Typography variant="h4">
        {frontmatter.id}. {frontmatter.title}
      </Typography>

      <LCMetaInfo metaInfo={frontmatter}/>
      <MyMarkdown>{content}</MyMarkdown>
    </Container>
  );
}
