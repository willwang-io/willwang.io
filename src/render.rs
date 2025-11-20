use lazy_static::lazy_static;
use tera::{Context, Tera};

pub fn insert_content_to_template(content: &str) -> Result<String, tera::Error> {
    let mut tera_ctx = Context::new();
    let html = djot_to_html(content);
    tera_ctx.insert("content", &html);
    TEMPLATES.render("base.html", &tera_ctx)
}

fn djot_to_html(content: &str) -> String {
    let events = jotdown::Parser::new(&content);
    jotdown::html::render_to_string(events)
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}
