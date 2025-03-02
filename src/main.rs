#![windows_subsystem = "windows"]
use native_dialog::FileDialog;
use std::{
    fs::File,
    io::{Read, Write},
    process,
    thread,
    time::Duration,
};
use tempfile::Builder;

fn main() {
    let path = FileDialog::new()
        .add_filter("Markdown Files", &["md", "markdown"])
        .add_filter("All Files", &["*"])
        .show_open_single_file()
        .unwrap();
    let path = path
        .ok_or_else(|| {
            eprintln!("No file selected.");
            process::exit(1);
        })
        .unwrap();
    let mut md_contents = String::new();
    File::open(&path).unwrap().read_to_string(&mut md_contents).unwrap();
    let mut html_file = Builder::new().suffix(".html").tempfile().unwrap();
    writeln!(html_file, "{}", markdown::to_html(&md_contents)).unwrap();
    if let Some(html_path) = html_file.path().to_str() {
        if webbrowser::open(html_path).is_ok() {
            thread::sleep(Duration::from_secs(10)); // Should be enough time for the browser to render the page without the temperary file getting claened up.
        }
    } else {
        eprintln!("Error opening HTML file.");
    }
}
