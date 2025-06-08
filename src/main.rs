#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![windows_subsystem = "windows"]

use anyhow::Result;
use native_dialog::FileDialog;
use pulldown_cmark::{html, Options, Parser};
use std::{env, fs, thread, time::Duration};
use tempfile::{Builder, NamedTempFile};

fn main() -> Result<()> {
    let path = get_file_path()?;
    let md_contents = fs::read_to_string(&path)?;
    let html_file = create_temp_html_file(&md_contents)?;
    let html_path = html_file
        .path()
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to convert temp file path to string"))?;
    open_in_browser(html_path)?;
    // Give the browser time to render the page
    thread::sleep(Duration::from_secs(10));
    Ok(())
}

fn get_file_path() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if let Some(path) = args.get(1) {
        return Ok(path.clone());
    }
    let file = FileDialog::new()
        .add_filter("Markdown Files", &["md", "markdown"])
        .add_filter("All Files", &["*"])
        .show_open_single_file()?
        .ok_or_else(|| anyhow::anyhow!("No file selected"))?;
    Ok(file
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?
        .to_string())
}

fn create_temp_html_file(md_contents: &str) -> Result<NamedTempFile> {
    let mut html_file = Builder::new().suffix(".html").tempfile()?;
    let options = Options::all();
    let parser = Parser::new_ext(md_contents, options);
    html::write_html_io(&mut html_file, parser)?;
    Ok(html_file)
}

fn open_in_browser(html_path: &str) -> Result<()> {
    webbrowser::open(html_path)?;
    Ok(())
}
