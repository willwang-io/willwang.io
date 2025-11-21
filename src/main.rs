mod content;
mod djot;
mod render;

use anyhow;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize)]
struct SiteConfig {
    content_dir: PathBuf,
    public_dir: PathBuf,
    main_title: String,
    thread: Option<HashMap<String, ThreadEntry>>,
    page: Option<HashMap<String, PageEntry>>,
}

#[derive(Deserialize)]
struct ThreadEntry {
    sub_title: Option<String>,
    path: PathBuf,
}

#[derive(Deserialize)]
struct PageEntry {
    sub_title: Option<String>,
    filename: PathBuf,
}

fn run() -> anyhow::Result<()> {
    let site_config: SiteConfig = toml::from_str(
        &std::fs::read_to_string("site.toml").expect("Should be able to read site.toml"),
    )?;

    let threads = site_config.thread.unwrap_or_default();
    for (_, config) in threads {
        let from = site_config.content_dir.join(&config.path);
        let to = site_config.public_dir.join(&config.path);
        if let Err(e) = content::render_dir(&from, &to) {
            eprintln!("Error while render {:?} to {:?}: {e}", from, to);
        }
    }

    let page = site_config.page.unwrap_or_default();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
