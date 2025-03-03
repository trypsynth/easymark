#![windows_subsystem = "windows"]
use native_dialog::FileDialog;
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
    writeln!(html_file, "{}", markdown::to_html(md_contents))?;
    Ok(html_file)
}

fn open_in_browser(html_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    webbrowser::open(html_path)?;
    Ok(())
}
