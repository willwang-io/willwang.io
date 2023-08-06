import ReactMarkdown from "react-markdown";

import rehypeStringify from 'rehype-stringify'
import rehypeSanitize, {defaultSchema} from 'rehype-sanitize'
import remarkMath from 'remark-math'
import rehypeKatex from "rehype-katex";
import rehypeHighlight from "rehype-highlight/lib";
import Typography from "@mui/material/Typography";
import Link from "@mui/material/Link";
import "katex/dist/katex.min.css"; // `rehype-katex` does not import the CSS for you


const myComponents = {
  a: (props) => <Link>{props.children}</Link>,
  p: (props) => <Typography>{props.children}</Typography>,
  h1: (props) => <Typography variant="h1">{props.children}</Typography>,
  h2: (props) => <Typography variant="h2">{props.children}</Typography>,
  h3: (props) => <Typography variant="h3">{props.children}</Typography>,
  h4: (props) => <Typography variant="h4">{props.children}</Typography>,
  h5: (props) => <Typography variant="h5">{props.children}</Typography>,
  h6: (props) => <Typography variant="h6">{props.children}</Typography>,
};

const remarkPlugins = [remarkMath];

const rehypePlugins = [
  [rehypeSanitize, {
  ...defaultSchema,
  attributes: {
  ...defaultSchema.attributes,
  div: [
    ...(defaultSchema.attributes.div || []),
    ['className', 'math', 'math-display']
  ],
  span: [
    ...(defaultSchema.attributes.span || []),
    ['className', 'math', 'math-inline']
  ],
  code: [
    ...(defaultSchema.attributes.code || []),
    ['className', 'hljs', 'language-cpp']
  ]}}],
  rehypeKatex, 
  [rehypeHighlight, {subset: false}],
  rehypeStringify,
]

export default function MyMarkdown({ children }) {
  return (
    <ReactMarkdown     
      components={myComponents}
      remarkPlugins={remarkPlugins}
      rehypePlugins={rehypePlugins}
    >
      {children}
    </ReactMarkdown>
  );
}
