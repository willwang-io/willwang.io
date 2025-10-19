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
        if bytes[idx] == b'$' {
            let run = if idx + 1 < n && bytes[idx + 1] == b'$' {
                2
            } else {
                1
            };
            if let Some((span_start, span_end, next_idx)) = extract_backtick_span(bytes, idx + run)
            {
                if last_emit < idx {
                    push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
                }

                let kind = if run == 2 {
                    AstKind::MathDisplay
                } else {
                    AstKind::MathInline
                };

                let node = AstNode {
                    kind,
                    span: Span {
                        start: span_start + offset,
                        end: span_end + offset,
                    },
                    attrs: None,
                    children: vec![],
                };
                push_node(&mut delim_stack, &mut nodes, node);

                idx = next_idx;
                last_emit = idx;
                continue;
            }
        }

        if let Some((span_start, span_end, next_idx)) = extract_backtick_span(bytes, idx) {
            if last_emit < idx {
                push_plain(&mut delim_stack, &mut nodes, last_emit, idx, offset);
            }

            let node = AstNode {
                kind: AstKind::Code,
                span: Span {
                    start: span_start + offset,
                    end: span_end + offset,
                },
                attrs: None,
                children: vec![],
            };
            push_node(&mut delim_stack, &mut nodes, node);

            idx = next_idx;
            last_emit = idx;
            continue;
        }

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

fn count_backticks(bytes: &[u8], idx: usize) -> usize {
    let mut count = 0;
    while idx + count < bytes.len() && bytes[idx + count] == b'`' {
        count += 1;
    }
    count
}

fn find_backtick_closer(bytes: &[u8], mut idx: usize, run: usize) -> (usize, usize) {
    let n = bytes.len();
    while idx < n {
        if bytes[idx] == b'`' {
            let mut count = 0;
            while idx + count < n && bytes[idx + count] == b'`' {
                count += 1;
            }
            if count == run {
                return (idx, run);
            }
            idx += count.max(1);
        } else {
            idx += 1;
        }
    }
    (n, 0)
}

fn extract_backtick_span(bytes: &[u8], idx: usize) -> Option<(usize, usize, usize)> {
    let run = count_backticks(bytes, idx);
    if run == 0 {
        return None;
    }

    let content_start_raw = idx + run;
    let (content_end_raw, close_len) = find_backtick_closer(bytes, content_start_raw, run);

    let mut span_start = content_start_raw;
    let mut span_end = if close_len == run {
        content_end_raw
    } else {
        bytes.len()
    };

    if span_start + 1 < bytes.len()
        && span_start < span_end
        && bytes.get(span_start) == Some(&b' ')
        && bytes.get(span_start + 1) == Some(&b'`')
    {
        span_start += 1;
    }

    if close_len == run
        && span_start < span_end
        && span_end > 0
        && bytes.get(span_end - 1) == Some(&b' ')
        && span_end >= span_start + 2
        && bytes.get(span_end - 2) == Some(&b'`')
    {
        span_end -= 1;
    }

    let next_idx = if close_len == run {
        content_end_raw + run
    } else {
        bytes.len()
    };

    Some((span_start, span_end, next_idx))
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

    #[test]
    fn inline_code_simple() {
        let line = "`code`";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![code(1, 5)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn inline_code_with_backtick_inside() {
        let line = "``Verbatim with a backtick` character``";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![code(2, line.len() - 2)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn inline_code_with_padding_for_backtick() {
        let line = "`` `foo` ``";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![code(3, 8)];
        assert_eq!(expected, actual);
    }

    #[test]
    fn inline_code_unterminated_extends() {
        let line = "`foo bar";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);
        let expected = vec![code(1, line.len())];
        assert_eq!(expected, actual);
    }

    #[test]
    fn inline_math() {
        let line = "Einstein derived $`e=mc^2`.";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);

        let dollar_pos = line.find('$').unwrap();
        let open_tick = line.find('`').unwrap();
        let close_tick = line.rfind('`').unwrap();
        let expected = vec![
            plain(0, dollar_pos),
            math_inline(open_tick + 1, close_tick),
            plain(close_tick + 1, line.len()),
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn display_math() {
        let line = "Pythagoras proved $$` x^n + y^n = z^n `";
        let mut ctx = Context::new(line);
        let actual = parse_inline(&mut ctx);

        let dollar_pos = line.find('$').unwrap();
        let open_tick = line.find('`').unwrap();
        let close_tick = line.rfind('`').unwrap();
        let expected = vec![
            plain(0, dollar_pos),
            math_display(open_tick + 1, close_tick),
        ];

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

    fn code(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::Code,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }

    fn math_inline(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::MathInline,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }

    fn math_display(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::MathDisplay,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }
}
