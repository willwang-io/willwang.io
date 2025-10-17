use crate::ast::{AstKind, AstNode};

pub fn convert_ast_to_html(node: &AstNode, src: &[u8]) -> String {
    match &node.kind {
        AstKind::Document => node
            .children
            .iter()
            .map(|child| convert_ast_to_html(child, src))
            .collect::<Vec<_>>()
            .join(""),
        AstKind::Paragraph => wrap_with_tag("p", node, src),
        AstKind::PlainText => node.span.as_str(src).to_owned(),
        // For now Emph nodes carry their text in span, not children.
        AstKind::Emph => format!("<em>{}</em>", node.span.as_str(src)),
        // …handle other kinds later…
        _ => render_children(node, src),
    }
}

fn render_children(node: &AstNode, src: &[u8]) -> String {
    node
        .children
        .iter()
        .map(|child| convert_ast_to_html(child, src))
        .collect::<Vec<_>>()
        .join("")
}

fn wrap_with_tag(tag: &str, node: &AstNode, src: &[u8]) -> String {
    format!("<{tag}>{}</{tag}>", render_children(node, src))
}

#[cfg(test)]
mod tests {
    use super::convert_ast_to_html;
    use crate::ast::{AstKind, AstNode, Span};

    fn plain(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::PlainText,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }

    fn emph(start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::Emph,
            span: Span { start, end },
            attrs: None,
            children: vec![],
        }
    }

    fn paragraph(children: Vec<AstNode>, start: usize, end: usize) -> AstNode {
        AstNode {
            kind: AstKind::Paragraph,
            span: Span { start, end },
            attrs: None,
            children,
        }
    }

    fn document(children: Vec<AstNode>, len: usize) -> AstNode {
        AstNode {
            kind: AstKind::Document,
            span: Span { start: 0, end: len },
            attrs: None,
            children,
        }
    }

    #[test]
    fn renders_paragraph_with_plain_text() {
        let src = "This is a simple line.";
        let doc = document(
            vec![paragraph(vec![plain(0, src.len())], 0, src.len())],
            src.len(),
        );

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(html, "<p>This is a simple line.</p>");
    }

    #[test]
    fn renders_emphasis_inside_paragraph() {
        let src = "Text with _emphasized_.";
        // Spans mirror inline parser tests: 0..10 text, 11..21 emph, 22..23 period
        let para = paragraph(
            vec![plain(0, 10), emph(11, 21), plain(22, 23)],
            0,
            23,
        );
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(html, "<p>Text with <em>emphasized</em>.</p>");
    }
}
