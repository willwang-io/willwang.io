/// Parse raw Djot to Abstract Syntax tree
use crate::ast::{AstKind, AstNode, Span};
use crate::parser::block::parse_block;
use crate::parser::context::{Context, is_blank};

mod block;
mod context;
mod inline;

pub fn parse_document(content: &str) -> AstNode {
    let mut ctx: Context = Context::new(content);

    let mut children = vec![];

    while !ctx.is_eof() {
        if is_blank(ctx.current_line()) {
            ctx.advance();
            continue;
        }
        children.push(parse_block(&mut ctx));
    }

    println!("{:?}", children);

    AstNode {
        kind: AstKind::Document,
        span: Span {
            start: 0,
            end: content.len(),
        },
        attrs: None,
        children,
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{AstKind, AstNode, Span};
    use crate::parser::parse_document;
    use indoc::indoc;

    #[test]
    fn multiple_blocks() {
        let line = indoc! {"
        first

        second
        "};

        let actual = parse_document(line);
        let expect = AstNode {
            kind: AstKind::Document,
            span: Span { start: 0, end: 14 },
            attrs: None,
            children: vec![
                AstNode {
                    kind: AstKind::Paragraph,
                    span: Span { start: 0, end: 5 },
                    attrs: None,
                    children: vec![
                        AstNode {
                            kind: AstKind::PlainText,
                            span: Span { start: 0, end: 5 },
                            attrs: None,
                            children: vec![],
                        }
                    ]
                },
                AstNode {
                    kind: AstKind::Paragraph,
                    span: Span { start: 7, end: 13 },
                    attrs: None,
                    children: vec![
                        AstNode {
                            kind: AstKind::PlainText,
                            span: Span { start: 7, end: 13 },
                            attrs: None,
                            children: vec![],
                        }
                    ]
                }
            ],
        };
        assert_eq!(expect, actual)
    }
}
