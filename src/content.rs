use crate::render::insert_content_to_template;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn render_dir(content_root: &Path, public_root: &Path) -> io::Result<()> {
    let entries: Vec<_> = fs::read_dir(content_root)?.collect();

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            render_dir(&path, public_root)?;
            continue;
        }
        if !is_dj(&path) {
            continue;
        }
        output_to_public(&path, content_root, public_root)?;
    }
    Ok(())
}

fn is_dj(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map(|ext| ext.eq_ignore_ascii_case("dj"))
        .unwrap_or(false)
}

fn output_to_public(
    djot_filepath: &Path,
    content_root: &Path,
    public_root: &Path,
) -> io::Result<()> {
    let djot_content = fs::read_to_string(djot_filepath)?;
    let html_out = insert_content_to_template(&djot_content)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let public_path = to_public_path(djot_filepath, content_root, public_root)?;
    write_with_dirs(&public_path, &html_out)
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

fn to_public_path(path: &Path, content_root: &Path, public_root: &Path) -> io::Result<PathBuf> {
    let rel = path.strip_prefix(content_root).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path not under content root: {e}"),
        )
    })?;
    let mut out = PathBuf::from(public_root);
    out.push(rel);
    out.set_extension("html");
    Ok(out)
}
