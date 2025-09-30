#[derive(Debug, PartialEq, Clone)]
pub struct AstNode {
    pub kind: AstKind,
    pub span: Span,
    pub attrs: Option<Attrs>,
    pub children: Vec<AstNode>,
}

impl AstNode {
    pub fn text_view<'a>(&self, src: &'a [u8]) -> Option<&'a str> {
        match self.kind {
            _ => Some(self.span.as_str(src)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstKind {
    // Block
    Document,
    Paragraph,
    Heading {
        level: u8,
    },
    BlockQuote,
    List {
        kind: ListKind,
    },
    ListItem,
    /// Fence code or raw block. Fence length and language/raw format are stored
    /// so you can render accurately.
    CodeBlock {
        fence: usize,
        lang: Option<String>,
        raw_format: Option<String>,
    },
    Div,
    Table {
        aligns: Vec<Align>,
    },
    ThematicBreak,
    /// Standalone attribute lines that apply to the next block.
    Attributes,

    // Inline
    PlainText,
    Emph,
    Strong,
    Code,
    Verbatim {
        format: Option<String>,
    },
    Link {
        dest_span: Option<Span>,
        title_span: Option<Span>,
    },
    Image {
        dest_span: Option<Span>,
        title_span: Option<Span>,
    },
    Sub,
    Sup,
    Insert,
    Delete,
    Mark,
    MathInline,
    MathDisplay,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn as_str<'a>(&self, src: &'a [u8]) -> &'a str {
        std::str::from_utf8(&src[self.start..self.end]).unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attrs {
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub kv: Vec<(String, String)>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ListKind {
    Bullet,
    Ordered,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Align {
    Left,
    Center,
    Right,
    None,
}

#[derive(Debug, PartialEq)]
pub enum DelimKind {
    Star,
    Underscore,
    Backtick { run: usize },
    LBracket,
    BangLBracket,
    Paren,
    Tilde,
    Caret,
    Insert,
    Delete,
    Mark,
    QuoteSingle,
    QuoteDouble,
}
