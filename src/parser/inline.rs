use crate::ast::{AstKind, AstNode, DelimKind, Span};
use crate::parser::context::Context;

pub fn parse_inline(ctx: &mut Context) -> Vec<AstNode> {
    let bytes = ctx.current_line().as_bytes();
    let n = bytes.len();
    let mut nodes: Vec<AstNode> = vec![];
    let mut delim_stack: Vec<(DelimKind, usize)> = Vec::new();

    let offset = ctx.line_range[ctx.cur_line_position].start;

    let mut last_emit = 0;
    let mut idx = 0;

    while idx < n {
        if let Some(delim) = delim_of(bytes[idx]) {
            // Close current
            if matches!(delim_stack.last(), Some((top, _)) if *top == delim) {
                let (_, start) = delim_stack.pop().unwrap();
                let kind = match delim {
                    DelimKind::Underscore => AstKind::Emph,
                    _ => AstKind::Attributes,
                };
                nodes.push(AstNode {
                    kind,
                    span: Span { start: start + offset, end: idx + offset },
                    attrs: None,
                    children: vec![],
                });

                if delim_stack.is_empty() {
                    last_emit = idx + 1;
                }
                idx += 1;
            } else {
                if last_emit < idx {
                    nodes.push(AstNode {
                        kind: AstKind::PlainText,
                        span: Span {
                            start: last_emit + offset,
                            end: idx + offset,
                        },
                        attrs: None,
                        children: vec![],
                    })
                }
                last_emit = idx;
                delim_stack.push((delim, idx + 1));
                idx += 1;
            }
        } else {
            idx += 1;
        }
    }
    if last_emit < n {
        nodes.push(AstNode {
            kind: AstKind::PlainText,
            span: Span {
                start: last_emit + offset,
                end: n + offset,
            },
            attrs: None,
            children: vec![],
        });
    }

    nodes
}

fn delim_of(c: u8) -> Option<DelimKind> {
    match c {
        b'_' => Some(DelimKind::Underscore),
        b'*' => Some(DelimKind::Star),
        _ => None,
    }
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
        let expect = vec![AstNode {
            kind: AstKind::PlainText,
            span: Span { start: 0, end: 22 },
            attrs: None,
            children: vec![],
        }];
        assert_eq!(expect, actual);
    }

    #[rstest]
    #[case::emphasis("_", AstKind::Emph)]
    // #[case::strong("*", AstKind::Strong)]
    // #[case::strong("~", AstKind::Sup)]
    // #[case::strong("^", AstKind::Sub)]
    fn inline_syntax_with_single_char_delimiter(#[case] delim: &str, #[case] kind: AstKind) {
        let line = format!("{delim}some text{delim}");
        let mut ctx = Context::new(line.as_str());

        let actual = parse_inline(&mut ctx);
        let expect = vec![AstNode {
            kind: kind,
            span: Span { start: 1, end: 10 },
            attrs: None,
            children: vec![],
        }];

        assert_eq!(expect, actual);
    }

    #[test]
    fn multiple_nodes() {
        let line = "Text with _emphasized_.";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![
            AstNode {
                kind: AstKind::PlainText,
                span: Span { start: 0, end: 10 },
                attrs: None,
                children: vec![],
            },
            AstNode {
                kind: AstKind::Emph,
                span: Span { start: 11, end: 21 },
                attrs: None,
                children: vec![],
            },
            AstNode {
                kind: AstKind::PlainText,
                span: Span { start: 22, end: 23 },
                attrs: None,
                children: vec![],
            },
        ];

        assert_eq!(expected, actual);
    }
}
