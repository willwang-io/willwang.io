import ReactMarkdown from "react-markdown";
import Accordion from "@mui/material/Accordion";
import AccordionDetails from "@mui/material/AccordionDetails";
import AccordionSummary from "@mui/material/AccordionSummary";
import Typography from "@mui/material/Typography";
import { Light as SyntaxHighlighter } from "react-syntax-highlighter";
import cpp from "react-syntax-highlighter/dist/cjs/languages/hljs/cpp";
import python from "react-syntax-highlighter/dist/cjs/languages/hljs/python";
import githubGist from "react-syntax-highlighter/dist/cjs/styles/hljs/github-gist";

import ExpandMoreIcon from "@mui/icons-material/ExpandMore";

import remarkMath from "remark-math";
import rehypeKatex from "rehype-katex";
import rehypeRaw from "rehype-raw";

import "katex/dist/katex.min.css"; // `rehype-katex` does not import the CSS for you
import CopyToClipboard from "../assets/copyToClipboard";

SyntaxHighlighter.registerLanguage("cpp", cpp);
SyntaxHighlighter.registerLanguage("python", python);

function mdParagraph({ node, ...props }) {
  return <Typography lineHeight={1.5} letterSpacing={0.8} sx={{ mb: 3, mt:0, textRendering: 'optimizeLegibility'}}>{props.children}</Typography>;
}

function mdCodeBlock({ node, inline, className, children, ...props }) {
  const match = /language-(\w+)/.exec(className || "");
  return !inline && match ? (
    <>
      <CopyToClipboard content={children} />
      <SyntaxHighlighter
        style={githubGist}
        language={match[1]}
        PreTag="div"
        showLineNumbers
        {...props}
      >
        {String(children).replace(/\n$/, "")}
      </SyntaxHighlighter>
    </>
  ) : (
    <code className={className} {...props}>
      {children}
    </code>
  );
}

const components = {
  p: mdParagraph,
  code: mdCodeBlock,
  h1: (props) => (
    <Typography variant="h4" sx={{ my: 1 }}>
      {props.children}
    </Typography>
  ),
  h2: (props) => (
    <Typography variant="h5" sx={{ mt: 1, mb: 2 }}>
      {props.children}
    </Typography>
  ),
  h3: (props) => (
    <Typography variant="h6" sx={{ my: 1 }}>
      {props.children}
    </Typography>
  ),
  details: (props) => {
    return (
      <Accordion
        disableGutters
        square
        sx={{
          boxShadow: "none",
          border: 1,
          borderLeft: 0,
          borderRight: 0,
          my: 2,
        }}
      >
        <AccordionSummary
          expandIcon={<ExpandMoreIcon />}
          aria-controls="problem-statement-content"
          id="problem-statement-header"
        >
          <Typography variant="h6">{props.children[1]}</Typography>
        </AccordionSummary>
        <AccordionDetails>{props.children.slice(3)}</AccordionDetails>
      </Accordion>
    );
  },
};

export default function MyMarkdown(props) {
  return (
    <ReactMarkdown
      components={components}
      remarkPlugins={[remarkMath]}
      rehypePlugins={[rehypeKatex, rehypeRaw]}
      {...props}
    />
  );
}
