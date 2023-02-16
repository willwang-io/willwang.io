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
import { ListItem, List } from "@mui/material";

SyntaxHighlighter.registerLanguage("cpp", cpp);
SyntaxHighlighter.registerLanguage("python", python);

function mdParagraph({ node, ...props }) {
  return (
    <Typography sx={{ mb: 3, mt: 0, textRendering: "optimizeLegibility" }}>
      {props.children}
    </Typography>
  );
}

function mdCodeBlock({ node, inline, className, children, ...props }) {
  const match = /language-(\w+)/.exec(className || "");
  return !inline && match ? (
    <>
      <CopyToClipboard content={children} />
      <SyntaxHighlighter
        customStyle={{ background: "#F9F6EE" }}
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
    <Typography variant="h1" sx={{ my: 1 }}>
      {props.children}
    </Typography>
  ),
  h2: (props) => (
    <Typography variant="h2" sx={{ mt: 1, mb: 2 }}>
      {props.children}
    </Typography>
  ),
  h3: (props) => (
    <Typography variant="h3" sx={{ my: 1 }}>
      {props.children}
    </Typography>
  ),
  // ul: (props) => <List disablePadding dense>{props.children}</List>,
  li: (props) => <li>{props.children}</li>,
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
