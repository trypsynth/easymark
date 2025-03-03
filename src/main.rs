#![windows_subsystem = "windows"]
use native_dialog::FileDialog;
use pulldown_cmark::{html, Options, Parser};
use std::{
    env,
    error::Error,
    fs::File,
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use tempfile::Builder;

fn main() -> Result<(), Box<dyn Error>> {
    let path = get_file_path()?;
    let md_contents = read_file_contents(&path)?;
    let html_file = create_temp_html_file(&md_contents)?;
    if let Some(html_path) = html_file.path().to_str() {
        open_in_browser(html_path)?;
        thread::sleep(Duration::from_secs(10)); // Give the browser time to render the page
    } else {
        return Err("Failed to convert temp file path to string.".into());
    }
    Ok(())
}

fn get_file_path() -> Result<String, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        Ok(args[1].clone())
    } else {
        let file = FileDialog::new()
            .add_filter("Markdown Files", &["md", "markdown"])
            .add_filter("All Files", &["*"])
            .show_open_single_file()?
            .ok_or("No file selected.")?;
        Ok(file.to_str().ok_or("Invalid file path")?.to_string())
    }
}

fn read_file_contents(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_temp_html_file(md_contents: &str) -> Result<tempfile::NamedTempFile, io::Error> {
    let mut html_file = Builder::new().suffix(".html").tempfile()?;
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION
        | Options::ENABLE_HEADING_ATTRIBUTES
        | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
        | Options::ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS
        | Options::ENABLE_OLD_FOOTNOTES
        | Options::ENABLE_MATH
        | Options::ENABLE_GFM
        | Options::ENABLE_DEFINITION_LIST
        | Options::ENABLE_SUPERSCRIPT
        | Options::ENABLE_SUBSCRIPT
        | Options::ENABLE_WIKILINKS;
    let parser = Parser::new_ext(md_contents, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    writeln!(html_file, "{}", html_output)?;
    Ok(html_file)
}

fn open_in_browser(html_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    webbrowser::open(html_path)?;
    Ok(())
}
