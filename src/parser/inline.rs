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
        if let Some(delim) = delim_of(bytes[idx]) {
            if matches!(delim_stack.last(), Some(top) if top.kind == delim) {
                if last_emit < idx {
                    push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
                }

                let top = delim_stack.pop().expect("delimiter stack corrupted");
                let kind = match delim {
                    DelimKind::Underscore => AstKind::Emph,
                    DelimKind::Star => AstKind::Strong,
                    DelimKind::Tilde => AstKind::Sub,
                    DelimKind::Caret => AstKind::Sup,
                    _ => AstKind::Attributes,
                };
                let node = AstNode {
                    kind,
                    span: Span {
                        start: top.content_start + offset,
                        end: idx + offset,
                    },
                    attrs: None,
                    children: top.children,
                };
                push_node(&mut delim_stack, &mut nodes, node);

                last_emit = idx + 1;
                idx += 1;
            } else {
                if last_emit < idx {
                    push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
                }
                delim_stack.push(Delimiter {
                    kind: delim,
                    open_pos: idx,
                    content_start: idx + 1,
                    children: Vec::new(),
                });
                last_emit = idx + 1;
                idx += 1;
            }
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
    kind: DelimKind,
    open_pos: usize,
    content_start: usize,
    children: Vec<AstNode>,
}

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

fn delim_of(c: u8) -> Option<DelimKind> {
    match c {
        b'_' => Some(DelimKind::Underscore),
        b'*' => Some(DelimKind::Star),
        b'~' => Some(DelimKind::Tilde),
        b'^' => Some(DelimKind::Caret),
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
