use crate::ast::{AstKind, AstNode, Span};
use crate::parser::context::{Context, is_blank};
use crate::parser::inline::parse_inline;

pub fn parse_block(ctx: &mut Context) -> AstNode {
    let mut children: Vec<AstNode> = Vec::new();
    let start = ctx.line_range[ctx.cur_line_position].start;
    let mut end = start;

    while !ctx.is_eof() && !is_blank(ctx.current_line()) {
        let nodes = parse_inline(ctx);
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
}
