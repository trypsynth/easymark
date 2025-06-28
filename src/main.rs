#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![windows_subsystem = "windows"]

use anyhow::{Result, anyhow};
use native_dialog::FileDialog;
use pulldown_cmark::{Options, Parser, html};
use std::{env, fs, thread, time::Duration};
use tempfile::{Builder, NamedTempFile};

fn main() -> Result<()> {
	let path = get_file_path()?;
	let md_contents = fs::read_to_string(&path)?;
	let html_file = render_markdown_to_tempfile(&md_contents)?;
	let html_path = html_file.path().to_str().ok_or_else(|| anyhow!("Failed to convert temp file path to string"))?;
	webbrowser::open(html_path)?;
	thread::sleep(Duration::from_secs(10));
	Ok(())
}

fn get_file_path() -> Result<String> {
	if let Some(path) = env::args().nth(1) {
		Ok(path)
	} else {
		let file = FileDialog::new()
			.add_filter("Markdown Files", &["md", "markdown"])
			.add_filter("All Files", &["*"])
			.show_open_single_file()?
			.ok_or_else(|| anyhow!("No file selected"))?;
		file.to_str().map(str::to_owned).ok_or_else(|| anyhow!("Invalid file path"))
	}
}

fn render_markdown_to_tempfile(md: &str) -> Result<NamedTempFile> {
	let mut file = Builder::new().suffix(".html").tempfile()?;
	let parser = Parser::new_ext(md, Options::all());
	html::write_html_io(&mut file, parser)?;
	Ok(file)
}
