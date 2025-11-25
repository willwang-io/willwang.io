mod djot;

use core::fmt;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use tera::{Context, Tera};

use crate::djot::{Metadata, parse_djot};

#[derive(Debug)]
pub enum AppError {
    Io(io::Error),
    Template(tera::Error),
    MetadataParse(toml::de::Error),
    MissingMetadata,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "io error: {e}"),
            AppError::Template(e) => write!(f, "template error: {e}"),
            AppError::MetadataParse(e) => write!(f, "metadata parse error: {e}"),
            AppError::MissingMetadata => write!(f, "missing metadata section"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<tera::Error> for AppError {
    fn from(e: tera::Error) -> Self {
        AppError::Template(e)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(e: toml::de::Error) -> Self {
        AppError::MetadataParse(e)
    }
}

type Result<T> = std::result::Result<T, AppError>;

fn process_threads_post(content_root: &Path, templates: &Tera) -> Result<()> {
    for entry_result in fs::read_dir(content_root)? {
        let entry = entry_result?;
        let path = entry.path();
        if path.is_dir() {
            process_threads_post(&path, templates)?;
            continue;
        }
        let raw_djot = fs::read_to_string(&path)?;
        let (metadata, html) = parse_djot(&raw_djot)?;
        let public_path = to_public_path(&path)?;

        let content = insert_content_to_template(&html, &metadata, templates)?;
        write_with_dirs(&public_path, &content)?;
    }
    Ok(())
}

pub fn insert_content_to_template(
    content: &str,
    meta: &Metadata,
    templates: &Tera,
) -> Result<String> {
    let mut tera_ctx = Context::new();
    tera_ctx.insert("content", content);
    if let Some(keywords) = meta.keywords.as_ref() {
        tera_ctx.insert("keywords", &keywords.join(", "));
    }
    tera_ctx.insert("title", &meta.title);
    if let Some(description) = meta.description.as_ref() {
        tera_ctx.insert("description", description);
    }
    let rendered = templates.render("base.html", &tera_ctx)?;
    Ok(rendered)
}

fn to_public_path(path: &Path) -> Result<PathBuf> {
    let rel = path.strip_prefix("content").map_err(|e| {
        AppError::Io(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path not under content root: {e}"),
        ))
    })?;
    let mut out = PathBuf::from("public");
    out.push(rel);
    out.set_extension("html");
    Ok(out)
}

fn write_with_dirs(path: impl AsRef<Path>, contents: &str) -> Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn run() -> Result<()> {
    let templates = Tera::new("templates/**/*.html")?;
    process_threads_post(Path::new("content"), &templates)
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
