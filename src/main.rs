#![windows_subsystem = "windows"]
use native_dialog::FileDialog;
use std::fs::File;
use std::io::{Read, Write};
use std::process;
use std::thread;
use std::time::Duration;
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
            // Give the browser enough time to open and render the HTML file. This value is probably super excessive, but we'll hardly use any RAM and I very highly doubt someone's going to run 500 easymarks in 2 minutes so whatever.
            thread::sleep(Duration::from_secs(120));
        }
    } else {
        eprintln!("Error opening HTML file.");
    }
}
