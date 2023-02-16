import { Card, Typography } from "@mui/material";
import { Container } from "@mui/system";
// import Link from "next/link";
import { Link } from "@mui/material";
import ChevronRightIcon from "@mui/icons-material/ChevronRight";

const OTCard = ({ tag, name, example }) => {
  return (
    <Card sx={{ boxShadow: 0 }}>
      <Typography display={"inline-block"}>{tag} </Typography>
      <Typography display={"inline-block"} variant="caption">
        {": "}
        {name}
      </Typography>
      <Container sx={{ textAlign: "center" }}>
        <Typography
          sx={{
            color: "#808080",
            fontFeatureSettings: `"${tag}" 0`,
            fontSize: "35px",
            display: "inline-block",
            marginBottom: 1,
          }}
        >
          {example}
        </Typography>
        <ChevronRightIcon sx={{ mr: 2, ml: 2 }} />
        <Typography
          sx={{
            display: "inline-block",
            fontSize: "35px",
            fontFeatureSettings: `"${tag}" 1`,
            marginBottom: 1,
          }}
        >
          {example}
        </Typography>
      </Container>
    </Card>
  );
};

export default function SiteNotes() {
  return (
    <Container>
      <Typography variant="h1">Site Notes</Typography>
      <Typography sx={{mt: 0, fontSize: '0.8em'}}>Last edited: Feb 12, 2023</Typography>
      <Typography variant="h2">Introduction</Typography>
      <Typography>
        Here I describe the typography and show demos of my website. In this
        context, also UI/CSS/themes. I am not an experts to any of those, but
        just want to make my page looks ok. I designed along the way I wrote
        content, but then I realized a major issue like any other software
        development, I miss the consistent and reusability. Thus, I create this
        page as an reference of the design of my website.
      </Typography>
      <Typography variant="h2">Sources</Typography>
      <Typography variant="h3">Typography</Typography>
      <Typography>
        While I am searching website typography, I stumbled into{" "}
        <Link href={"https://practicaltypography.com/"}>
          Butterick’s Practical Typography
        </Link>
        . The book not only introduce what typography to use, but also the
        reason behind it. Thus, this website is basically a practical
        application of the book. The author also designed a families of elegant
        fonts, this website is using{" "}
        <Link href="https://mbtype.com/fonts/triplicate/">Triplicate</Link> font
        for inline and block code; and{" "}
        <Link href="https://mbtype.com/fonts/equity/">Equity</Link> for
        everything else. Check the book for more information.
      </Typography>
      <Typography variant="h3">Tools</Typography>
      <Typography>
        This website is created with Next.js and hosted on Vercel. I am using{" "}
        <Link href={"https://mui.com/"}>Material UI</Link> for UI/theming, I try
        to take full advantage and utilize the feature of the Material UI to
        achieve the design of this website.
      </Typography>
      <Typography variant="h2">Body text</Typography>
      <Typography>
        Body text/paragraph has 20px font size, 0.05em letter spacing, 1em
        bottom margin, 0.5em top margin, and 1.35em line height.
      </Typography>
      <Typography variant="h2">Headings</Typography>
      <Typography>
        h1 to h6 have sizes 2em, 1.5em, 1.25em, 1.12em, 1.06em, and 1.03em,
        respectively.
      </Typography>

      <Typography variant="h2">OpenType</Typography>
      <Typography>
        The following Open Type features are enabled for body text.
      </Typography>

      <OTCard tag="liga" name="Standard Ligatures" example={"ff fi fj ffl"} />
      <OTCard
        tag="onum"
        name="Oldstyle Figures"
        example={"0 1 2 3 4 5 6 7 8 9"}
      />

      {/* <Typography variant="h2">Links</Typography>
      <Typography>hello world <Link href="/">hello, world</Link></Typography>
      <Link href={'#'}>hollo</Link> */}
    </Container>
  );
}
