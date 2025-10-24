mod ast;
mod file_util;
mod parser;
mod render;

use std::path::Path;
use tera;

use file_util::{IN_DIR, OUT_DIR, read_dir, read_file, write_file};
use parser::parse_document;
use render::convert_ast_to_html;

fn main() {
    let tera = tera::Tera::new("templates/**/*.html").unwrap();
    let mut tera_ctx = tera::Context::new();

    let files = read_dir(IN_DIR).unwrap();

    for path in files {
        if !path.is_file() {
            continue;
        }
        let content = read_file(&path).unwrap();
        let ast_root = parse_document(&content);

        let rendered_html = convert_ast_to_html(&ast_root, &content.as_bytes());
        tera_ctx.insert("post", &rendered_html);
        let out_content = tera.render("base.html", &tera_ctx).unwrap();

        let mut output_path = path;
        output_path.set_extension("html");
        let path = Path::new(OUT_DIR).join(output_path.strip_prefix("content").unwrap());
        write_file(&path, &out_content).unwrap();
    }
}
