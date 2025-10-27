use crate::ast::{AstKind, AstNode, Span};
use crate::parser::context::{Context, is_blank};
use crate::parser::inline::parse_inline;

pub fn parse_block(ctx: &mut Context) -> AstNode {
    if is_block_quote_line(ctx.current_line()) {
        parse_block_quote(ctx)
    } else {
        parse_paragraph(ctx)
    }
}

fn parse_paragraph(ctx: &mut Context) -> AstNode {
    let mut children: Vec<AstNode> = Vec::new();
    let start = ctx.line_range[ctx.cur_line_position].start;
    let mut end = start;

    while !ctx.is_eof() && !is_blank(ctx.current_line()) {
        let offset = ctx.line_range[ctx.cur_line_position].start;
        let line = ctx.current_line();
        let nodes = parse_inline(line, offset);
        end += ctx.current_line().len();
        children.extend(nodes);
        ctx.advance();
    }
    AstNode {
        kind: AstKind::Paragraph,
        span: Span { start, end },
        attrs: None,
        children,
    }
}

fn parse_block_quote(ctx: &mut Context) -> AstNode {
    let start = ctx.line_range[ctx.cur_line_position].start;
    let mut end = start;
    let mut children: Vec<AstNode> = Vec::new();
    let mut current_para: Vec<AstNode> = Vec::new();

    while !ctx.is_eof() {
        let line_idx = ctx.cur_line_position;
        let line_start = ctx.line_range[line_idx].start;
        let line_end = ctx.line_range[line_idx].end;

        {
            let line = ctx.current_line();
            if !is_block_quote_line(line) {
                break;
            }

            let (content, marker_len) = strip_block_quote_marker(line);
            let content_offset = line_start + marker_len;

            if is_blank(content) {
                flush_paragraph(&mut children, &mut current_para);
            } else {
                let nodes = parse_inline(content, content_offset);
                current_para.extend(nodes);
            }
        }

        end = line_end;
        ctx.advance();
    }

    flush_paragraph(&mut children, &mut current_para);

    AstNode {
        kind: AstKind::BlockQuote,
        span: Span { start, end },
        attrs: None,
        children,
    }
}

fn is_block_quote_line(line: &str) -> bool {
    line.starts_with('>')
}

fn strip_block_quote_marker(line: &str) -> (&str, usize) {
    if let Some(rest) = line.strip_prefix("> ") {
        (rest, 2)
    } else if let Some(rest) = line.strip_prefix('>') {
        (rest, 1)
    } else {
        (line, 0)
    }
}

fn flush_paragraph(children: &mut Vec<AstNode>, current_para: &mut Vec<AstNode>) {
    if current_para.is_empty() {
        return;
    }

    let start = current_para.first().unwrap().span.start;
    let end = current_para.last().unwrap().span.end;

    let paragraph = AstNode {
        kind: AstKind::Paragraph,
        span: Span { start, end },
        attrs: None,
        children: std::mem::take(current_para),
    };

    children.push(paragraph);
}

#[cfg(test)]
mod tests {
    use crate::ast::{AstKind, AstNode, Span};
    use crate::parser::block::parse_block;
    use crate::parser::context::Context;
    use indoc::indoc;

    #[test]
    fn simple_paragraph() {
        let line = indoc! {
            "This is a simple line."
        };
        let mut ctx = Context::new(line);

        let actual = parse_block(&mut ctx);
        let expect = AstNode {
            kind: AstKind::Paragraph,
            span: Span { start: 0, end: 22 },
            attrs: None,
            children: vec![AstNode {
                kind: AstKind::PlainText,
                span: Span { start: 0, end: 22 },
                attrs: None,
                children: vec![],
            }],
        };
        assert_eq!(expect, actual);
    }

    #[test]
    fn block_quote_with_multiple_paragraphs() {
        let src = indoc! {"
        > This is a line
        > This is another line
        > This is the third line
        >
        > This is another paragraph.
        "};

        let mut ctx = Context::new(src);
        let actual = parse_block(&mut ctx);

        assert_eq!(actual.kind, AstKind::BlockQuote);
        assert_eq!(actual.children.len(), 2);

        let first_para = &actual.children[0];
        assert_eq!(first_para.kind, AstKind::Paragraph);
        assert_eq!(first_para.children.len(), 3);
        let bytes = src.as_bytes();
        assert_eq!(
            first_para.children[0].span.as_str(bytes),
            "This is a line"
        );
        assert_eq!(
            first_para.children[1].span.as_str(bytes),
            "This is another line"
        );
        assert_eq!(
            first_para.children[2].span.as_str(bytes),
            "This is the third line"
        );

        let second_para = &actual.children[1];
        assert_eq!(second_para.kind, AstKind::Paragraph);
        assert_eq!(second_para.children.len(), 1);
        assert_eq!(
            second_para.children[0].span.as_str(bytes),
            "This is another paragraph."
        );
    }
}
