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
          Hello there. I am a software developer. I love typing, competitive
          programming, and speed cubing. Other than that, I am also passionate 
          about distance running. 
        </Typography>
        <Typography>
          This is my personal website, where I randomly blog what I learned and 
          experienced. 
        </Typography>
      </Container>
    </>
  );
}
