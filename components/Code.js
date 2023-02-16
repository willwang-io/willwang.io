
import { Light as SyntaxHighlighter } from "react-syntax-highlighter";
import cpp from "react-syntax-highlighter/dist/cjs/languages/hljs/cpp";
import python from "react-syntax-highlighter/dist/cjs/languages/hljs/python";
import githubGist from "react-syntax-highlighter/dist/cjs/styles/hljs/github-gist";
import CopyToClipboard from "../assets/copyToClipboard";

SyntaxHighlighter.registerLanguage("cpp", cpp);
SyntaxHighlighter.registerLanguage("python", python);

export default function Code({ lang='text', is_block=false, children, ...props }) {
  return is_block ? (
    <>
    <CopyToClipboard content={children}/>
    <SyntaxHighlighter
      style={githubGist}
      language={lang}
      PreTag='div'
      showLineNumbers
    >
      {String(children).replace(/\n$/, '')}
    </SyntaxHighlighter>
    </>
  ) : (<code>{children}</code>)
}