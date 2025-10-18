use crate::ast::{AstKind, AstNode, DelimKind, Span};
use crate::parser::context::Context;

pub fn parse_inline(ctx: &mut Context) -> Vec<AstNode> {
    let bytes = ctx.current_line().as_bytes();
    let n = bytes.len();
    let mut nodes: Vec<AstNode> = Vec::new();
    let mut delim_stack: Vec<Delimiter> = Vec::new();

    let offset = ctx.line_range[ctx.cur_line_position].start;

    let mut last_emit = 0;
    let mut idx = 0;

    while idx < n {
        if let Some((def_index, close_len)) = match_closing(bytes, idx, &delim_stack) {
            if last_emit < idx {
                push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
            }

            let top = delim_stack.pop().expect("delimiter stack corrupted");
            let def = &DELIMITERS[def_index];
            let node = AstNode {
                kind: def.ast_kind(),
                span: Span {
                    start: top.content_start + offset,
                    end: idx + offset,
                },
                attrs: None,
                children: top.children,
            };
            push_node(&mut delim_stack, &mut nodes, node);

            last_emit = idx + close_len;
            idx += close_len;
            continue;
        }

        if let Some((def_index, open_len)) = match_opening(bytes, idx) {
            if last_emit < idx {
                push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
            }
            delim_stack.push(Delimiter {
                def_index,
                open_pos: idx,
                content_start: idx + open_len,
                children: Vec::new(),
            });
            last_emit = idx + open_len;
            idx += open_len;
        } else {
            idx += 1;
        }
    }

    if !delim_stack.is_empty() {
        if let Some(min_open) = delim_stack.iter().map(|d| d.open_pos).min() {
            last_emit = last_emit.min(min_open);
        }
        delim_stack.clear();
    }

    if last_emit < n {
        push_plain(&mut delim_stack, &mut nodes, last_emit, n, offset);
    }

    nodes
}

#[derive(Debug)]
struct Delimiter {
    def_index: usize,
    open_pos: usize,
    content_start: usize,
    children: Vec<AstNode>,
}

#[derive(Debug)]
struct DelimiterDef {
    kind: DelimKind,
    open: &'static [u8],
    close: &'static [u8],
}

impl DelimiterDef {
    const fn new(kind: DelimKind, open: &'static [u8], close: &'static [u8]) -> Self {
        Self { kind, open, close }
    }

    fn ast_kind(&self) -> AstKind {
        match self.kind {
            DelimKind::Underscore => AstKind::Emph,
            DelimKind::Star => AstKind::Strong,
            DelimKind::Tilde => AstKind::Sub,
            DelimKind::Caret => AstKind::Sup,
            DelimKind::Mark => AstKind::Mark,
            DelimKind::Insert => AstKind::Insert,
            DelimKind::Delete => AstKind::Delete,
            _ => AstKind::PlainText,
        }
    }
}

const DELIMITERS: &[DelimiterDef] = &[
    DelimiterDef::new(DelimKind::Mark, b"{=", b"=}"),
    DelimiterDef::new(DelimKind::Insert, b"{+", b"+}"),
    DelimiterDef::new(DelimKind::Delete, b"{-", b"-}"),
    DelimiterDef::new(DelimKind::Underscore, b"_", b"_"),
    DelimiterDef::new(DelimKind::Star, b"*", b"*"),
    DelimiterDef::new(DelimKind::Tilde, b"~", b"~"),
    DelimiterDef::new(DelimKind::Caret, b"^", b"^"),
];

fn push_plain(
    delim_stack: &mut Vec<Delimiter>,
    nodes: &mut Vec<AstNode>,
    start: usize,
    end: usize,
    offset: usize,
) {
    if start >= end {
        return;
    }

    let node = AstNode {
        kind: AstKind::PlainText,
        span: Span {
            start: start + offset,
            end: end + offset,
        },
        attrs: None,
        children: vec![],
    };

    push_node(delim_stack, nodes, node);
}

fn push_node(delim_stack: &mut Vec<Delimiter>, nodes: &mut Vec<AstNode>, node: AstNode) {
    if let Some(top) = delim_stack.last_mut() {
        top.children.push(node);
    } else {
        nodes.push(node);
    }
}

fn match_closing(bytes: &[u8], idx: usize, stack: &[Delimiter]) -> Option<(usize, usize)> {
    let top = stack.last()?;
    let def = &DELIMITERS[top.def_index];
    if starts_with(bytes, idx, def.close) {
        Some((top.def_index, def.close.len()))
    } else {
        None
    }
}

fn match_opening(bytes: &[u8], idx: usize) -> Option<(usize, usize)> {
    let mut best: Option<(usize, usize)> = None;
    for (i, def) in DELIMITERS.iter().enumerate() {
        if starts_with(bytes, idx, def.open) {
            let len = def.open.len();
            if best.map_or(true, |(_, best_len)| len > best_len) {
                best = Some((i, len));
            }
        }
    }
    best
}

fn starts_with(bytes: &[u8], idx: usize, pat: &[u8]) -> bool {
    let end = idx + pat.len();
    end <= bytes.len() && &bytes[idx..end] == pat
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use rstest::rstest;

    use crate::ast::{AstKind, AstNode, Span};
    use crate::parser::context::Context;
    use crate::parser::inline::parse_inline;

    #[test]
    fn simple_text() {
        let line = indoc! { "This is a simple line." };
        let mut ctx = Context::new(line);

        let actual = parse_inline(&mut ctx);
        let expect = vec![plain(0, 22)];
        assert_eq!(expect, actual);
    }

    #[rstest]
    #[case::emphasis("_", AstKind::Emph)]
    #[case::strong("*", AstKind::Strong)]
    #[case::strong("~", AstKind::Sub)]
    #[case::strong("^", AstKind::Sup)]
    fn inline_syntax_with_single_char_delimiter(#[case] delim: &str, #[case] kind: AstKind) {
        let line = format!("{delim}some text{delim}");
        let mut ctx = Context::new(line.as_str());

        let actual = parse_inline(&mut ctx);
        let expect = vec![with_children(kind, 1, 10, vec![plain(1, 10)])];

        assert_eq!(expect, actual);
    }

    #[test]
    fn multiple_nodes() {
        let line = "Text with _emphasized_.";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![
            plain(0, 10),
            with_children(AstKind::Emph, 11, 21, vec![plain(11, 21)]),
            plain(22, 23),
        ];
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("{=highlight=}", AstKind::Mark, 2, 11)]
    #[case("{+insert+}", AstKind::Insert, 2, 8)]
    #[case("{-remove-}", AstKind::Delete, 2, 8)]
    fn inline_multi_char_delimiters(
        #[case] line: &str,
        #[case] kind: AstKind,
        #[case] start: usize,
        #[case] end: usize,
    ) {
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![with_children(kind, start, end, vec![plain(start, end)])];
        assert_eq!(expected, actual);
    }

    #[test]
    fn multi_syntaxes_in_one_line() {
        let line = "_emphasized_ and *strong*";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![
            with_children(AstKind::Emph, 1, 11, vec![plain(1, 11)]),
            plain(12, 17),
            with_children(AstKind::Strong, 18, 24, vec![plain(18, 24)]),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn nested_inline_nodes() {
        let line = "_*they can be nested*_";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![with_children(
            AstKind::Emph,
            1,
            21,
            vec![with_children(AstKind::Strong, 2, 20, vec![plain(2, 20)])],
        )];
        assert_eq!(expected, actual);
    }

    fn plain(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::PlainText,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }

    fn with_children(kind: AstKind, start: usize, end: usize, children: Vec<AstNode>) -> AstNode {
        AstNode {
            kind,
            span: Span { start, end },
            attrs: None,
            children,
        }
    }
}
