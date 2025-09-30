use std::path::{Path, PathBuf};
use std::{fs, io};

pub const IN_DIR: &str = "content";
pub const OUT_DIR: &str = "public";

pub fn read_file(file_path: &PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}

pub fn read_dir(dir: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    entries.sort();
    Ok(entries)
}

pub fn write_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?
    }
    fs::write(path, content)
}
