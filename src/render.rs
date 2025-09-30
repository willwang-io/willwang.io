use crate::ast::{AstKind, AstNode};

pub fn convert_ast_to_html(node: &AstNode, src: &[u8]) -> String {
    match &node.kind {
        AstKind::Document => node
            .children
            .iter()
            .map(|child| convert_ast_to_html(child, src))
            .collect(),
        AstKind::Paragraph => wrap_with_tag("p", node, src),
        // AstKind::Heading { level } => format!("<h{lvl}>{body}</h{lvl}>",
        //     lvl = level, body = render_children(node, src)),
        AstKind::PlainText => node.span.as_str(src).to_owned(),
        // AstKind::Strong => wrap_inline("strong", node, src),
        // AstKind::Emph => wrap_inline("em", node, src),
        // AstKind::List { kind } => match kind {
        //     ListKind::Bullet => wrap("ul", node, src),
        //     ListKind::Ordered => wrap("ol", node, src),
        // },
        // AstKind::ListItem => wrap("li", node, src),
        // …handle the rest of AstKind variants here…
        _ => render_children(node, src),
    }
}

fn render_children(node: &AstNode, src: &[u8]) -> String {
    node.children
        .iter()
        .map(|child| convert_ast_to_html(child, src))
        .collect()
}

fn wrap_with_tag(tag: &str, node: &AstNode, src: &[u8]) -> String {
    format!("<{tag}>{}</{tag}>", render_children(node, src))
}
