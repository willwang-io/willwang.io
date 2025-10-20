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
        AstKind::Emph => wrap_inline_tag("em", node, src),
        AstKind::Strong => wrap_inline_tag("strong", node, src),
        AstKind::Mark => wrap_inline_tag("mark", node, src),
        AstKind::Insert => wrap_inline_tag("ins", node, src),
        AstKind::Delete => wrap_inline_tag("del", node, src),
        AstKind::Sub => wrap_inline_tag("sub", node, src),
        AstKind::Sup => wrap_inline_tag("sup", node, src),
        AstKind::Code => wrap_inline_tag("code", node, src),
        AstKind::Link { dest_span, .. } => {
            let href = dest_span
                .as_ref()
                .map(|span| span.as_str(src))
                .unwrap_or("");
            format!("<a href=\"{}\">{}</a>", href, render_children(node, src))
        }
        AstKind::Image { dest_span, .. } => {
            let src_attr = dest_span
                .as_ref()
                .map(|span| span.as_str(src))
                .unwrap_or("");
            let alt = node
                .children
                .iter()
                .map(|child| child.span.as_str(src))
                .collect::<String>();
            format!("<img alt=\"{}\" src=\"{}\">", alt, src_attr)
        }
        AstKind::MathInline => format!(
            "<span class=\"math inline\">\\({}\\)</span>",
            inline_inner(node, src)
        ),
        AstKind::MathDisplay => format!(
            "<span class=\"math display\">\\[{}\\]</span>",
            inline_inner(node, src)
        ),
        // …handle other kinds later…
        _ => render_children(node, src),
    }
}

fn render_children(node: &AstNode, src: &[u8]) -> String {
    node.children
        .iter()
        .map(|child| convert_ast_to_html(child, src))
        .collect::<Vec<_>>()
        .join("")
}

fn wrap_with_tag(tag: &str, node: &AstNode, src: &[u8]) -> String {
    format!("<{tag}>{}</{tag}>", render_children(node, src))
}

fn wrap_inline_tag(tag: &str, node: &AstNode, src: &[u8]) -> String {
    let inner = inline_inner(node, src);
    format!("<{tag}>{inner}</{tag}>")
}

fn inline_inner(node: &AstNode, src: &[u8]) -> String {
    if node.children.is_empty() {
        node.span.as_str(src).to_owned()
    } else {
        render_children(node, src)
    }
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
        inline_node(AstKind::Emph, start, end)
    }

    fn mark(start: usize, end: usize) -> AstNode {
        inline_node(AstKind::Mark, start, end)
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
        let para = paragraph(vec![plain(0, 10), emph(11, 21), plain(22, 23)], 0, 23);
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(html, "<p>Text with <em>emphasized</em>.</p>");
    }

    #[test]
    fn renders_mark_inline() {
        let src = "{=highlight=}";
        let para = paragraph(vec![mark(2, 11)], 0, src.len());
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(html, "<p><mark>highlight</mark></p>");
    }

    #[test]
    fn renders_inline_code() {
        let src = "`code`";
        let para = paragraph(vec![code(1, 5)], 0, src.len());
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(html, "<p><code>code</code></p>");
    }

    #[test]
    fn renders_inline_math() {
        let src = "Einstein derived $`e=mc^2`.";
        let span_start = src.find('`').unwrap() + 1;
        let span_end = src.rfind('`').unwrap();
        let para = paragraph(
            vec![
                plain(0, src.find('$').unwrap()),
                math_inline(span_start, span_end),
                plain(span_end + 1, src.len()),
            ],
            0,
            src.len(),
        );
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(
            html,
            "<p>Einstein derived <span class=\"math inline\">\\(e=mc^2\\)</span>.</p>"
        );
    }

    #[test]
    fn renders_display_math() {
        let src = "Pythagoras proved $$` x^n + y^n = z^n `";
        let span_start = src.find('`').unwrap() + 1;
        let span_end = src.rfind('`').unwrap();
        let para = paragraph(
            vec![
                plain(0, src.find('$').unwrap()),
                math_display(span_start, span_end),
            ],
            0,
            src.len(),
        );
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(
            html,
            "<p>Pythagoras proved <span class=\"math display\">\\[ x^n + y^n = z^n \\]</span></p>"
        );
    }

    #[test]
    fn renders_link() {
        let src = "[My link text](http://example.com)";
        let label_start = src.find('[').unwrap() + 1;
        let label_end = src.find(']').unwrap();
        let dest_start = src.find('(').unwrap() + 1;
        let dest_end = src.rfind(')').unwrap();
        let para = paragraph(
            vec![link(
                label_start,
                label_end,
                dest_start,
                dest_end,
                vec![plain(label_start, label_end)],
            )],
            0,
            src.len(),
        );
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(
            html,
            "<p><a href=\"http://example.com\">My link text</a></p>"
        );
    }

    #[test]
    fn renders_image() {
        let src = "![picture of a cat](cat.jpg)";
        let label_start = src.find('[').unwrap() + 1;
        let label_end = src.find(']').unwrap();
        let dest_start = src.find('(').unwrap() + 1;
        let dest_end = src.rfind(')').unwrap();
        let para = paragraph(
            vec![image(
                label_start,
                label_end,
                dest_start,
                dest_end,
                vec![plain(label_start, label_end)],
            )],
            0,
            src.len(),
        );
        let doc = document(vec![para], src.len());

        let html = convert_ast_to_html(&doc, src.as_bytes());
        assert_eq!(
            html,
            "<p><img alt=\"picture of a cat\" src=\"cat.jpg\"></p>"
        );
    }

    fn inline_node(kind: AstKind, start: usize, end: usize) -> AstNode {
        AstNode {
            kind,
            span: Span { start, end },
            attrs: None,
            children: vec![plain(start, end)],
        }
    }

    fn link(
        label_start: usize,
        label_end: usize,
        dest_start: usize,
        dest_end: usize,
        children: Vec<AstNode>,
    ) -> AstNode {
        AstNode {
            kind: AstKind::Link {
                dest_span: Some(Span {
                    start: dest_start,
                    end: dest_end,
                }),
                title_span: None,
            },
            span: Span {
                start: label_start,
                end: label_end,
            },
            attrs: None,
            children,
        }
    }

    fn image(
        label_start: usize,
        label_end: usize,
        dest_start: usize,
        dest_end: usize,
        children: Vec<AstNode>,
    ) -> AstNode {
        AstNode {
            kind: AstKind::Image {
                dest_span: Some(Span {
                    start: dest_start,
                    end: dest_end,
                }),
                title_span: None,
            },
            span: Span {
                start: label_start,
                end: label_end,
            },
            attrs: None,
            children,
        }
    }
}
