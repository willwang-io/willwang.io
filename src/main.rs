mod djot;

use anyhow;
use lazy_static::lazy_static;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::io;
use tera::{Context, Tera};

use crate::djot::{Metadata, parse_djot};

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

fn process_threads_post(content_root: &Path) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(content_root)?.collect();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            process_threads_post(&path)?;
            continue;
        }
        let raw_djot = fs::read_to_string(&path)?;
        let (metadata, html) = parse_djot(&raw_djot);
        let public_path = to_public_path(&path)?;

        let content = match insert_content_to_template(&html, &metadata) {
            Ok(res) => res,
            Err(e) => {
                panic!("Failed to insert to template: {e}");
            }
        };
        let _ = write_with_dirs(&public_path, &content);
    }
    Ok(())
}

pub fn insert_content_to_template(content: &str, meta: &Metadata) -> Result<String, tera::Error> {
    let mut tera_ctx = Context::new();
    tera_ctx.insert("content", content);
    if let Some(keywords) = meta.keywords.as_ref() {
        tera_ctx.insert("keywords",&keywords.join(", "));
    }

    TEMPLATES.render("base.html", &tera_ctx)
}

fn to_public_path(path: &Path) -> io::Result<PathBuf> {
    let rel = path.strip_prefix("content").map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path not under content root: {e}"),
        )
    })?;
    let mut out = PathBuf::from("public");
    out.push(rel);
    out.set_extension("html");
    Ok(out)
}

fn write_with_dirs(path: impl AsRef<Path>, contents: &str) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(contents.as_bytes())
}

fn run() -> anyhow::Result<()> {
    let _ = process_threads_post(&PathBuf::from("content"));
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
