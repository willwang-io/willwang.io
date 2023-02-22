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
        <Link href="/site-notes">Site Notes</Link>
        <Typography>I don&rsquo;t like this dark mode flickering :( Fix it. </Typography>
      </Container>
    </>
  );
}
