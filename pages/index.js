import { Container } from "@mui/system";
import { Link, Typography } from "@mui/material";
import Head from "next/head";

export default function Home() {
  return (
    <>
      <Head>
        <title>Will&rsquo;s log</title>
      </Head>
      <Container>
        <Typography>
          Hello there, I&rsquo;m a software developer who, after work, indulges 
          in coding, typing, and cubing. Additionally, I&rsquo;m passionate about long-distance running.
        </Typography>
        <Typography>
          This is my personal website, where I randomly blog what I learned and 
          experienced. 
        </Typography>
      </Container>
    </>
  );
}
