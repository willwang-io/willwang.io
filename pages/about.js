import { Container, Typography } from "@mui/material";
import Head from "next/head";

export default function About() {
  return (
    <>
    <Head>
      <title>About</title>
    </Head>
      <Container>
        <Typography variant="h2">Hello, you are here...</Typography>
        <Typography>
          My name is Will. I am a software developer in Canada.{" "}
        </Typography>
      </Container>
    </>
  );
}
