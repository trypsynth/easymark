#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{
	env,
	error::Error,
	fs,
	io::{BufRead, BufReader, Write},
	net::{TcpListener, TcpStream},
	path::PathBuf,
	result, thread,
};

use native_dialog::DialogBuilder;
use pulldown_cmark::{Options, Parser, html};

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
	let path = get_file_path()?;
	let addr = serve_markdown(PathBuf::from(path))?;
	let url = format!("http://{addr}/");
	println!("Running on {url}");
	webbrowser::open(&url)?;
	loop {
		thread::park();
	}
}

fn get_file_path() -> Result<String> {
	if let Some(path) = env::args().nth(1) {
		Ok(path)
	} else {
		let file = DialogBuilder::file()
			.add_filter("Markdown Files", ["md", "markdown", "mdx", "mdown", "mdwn", "mkd", "mkdn", "mkdown", "ronn"])
			.add_filter("All Files", ["*"])
			.open_single_file()
			.show()?
			.ok_or("No file selected")?;
		file.to_str().map(str::to_owned).ok_or_else(|| "Invalid file path".into())
	}
}

fn serve_markdown(path: PathBuf) -> Result<String> {
	let listener = TcpListener::bind("127.0.0.1:0")?;
	let addr = listener.local_addr()?;
	let addr_string = addr.to_string();
	thread::spawn(move || {
		for stream in listener.incoming() {
			match stream {
				Ok(s) => {
					if let Err(e) = handle_connection(&s, &path) {
						eprintln!("Connection error: {e}");
					}
				}
				Err(e) => eprintln!("Incoming connection failed: {e}"),
			}
		}
	});
	Ok(addr_string)
}

fn handle_connection(stream: &TcpStream, md_path: &PathBuf) -> Result<()> {
	stream.set_nodelay(true).ok();
	let mut reader = BufReader::new(stream.try_clone()?);
	let mut request_line = String::new();
	reader.read_line(&mut request_line)?;
	let mut line = String::new();
	loop {
		line.clear();
		let n = reader.read_line(&mut line)?;
		if n == 0 || line == "\r\n" || line.is_empty() {
			break;
		}
	}
	let (method, path) = parse_request_line(&request_line)?;
	match (method.as_str(), path.as_str()) {
		("GET", "/") => respond_ok_html(stream, &render_markdown_page(md_path)?)?,
		_ => respond_not_found(stream)?,
	}
	Ok(())
}

fn parse_request_line(line: &str) -> Result<(String, String)> {
	let mut parts = line.split_whitespace();
	let method = parts.next().ok_or("Malformed request")?.to_string();
	let path = parts.next().ok_or("Malformed request")?.to_string();
	Ok((method, path))
}

fn render_markdown_page(md_path: &PathBuf) -> Result<String> {
	let md = fs::read_to_string(md_path)?;
	let parser = Parser::new_ext(&md, Options::all());
	let mut body_html = String::new();
	html::push_html(&mut body_html, parser);
	let page = format!(
		"<!doctype html>\n<html lang=\"en\">\n<head>\n\t<meta charset=\"utf-8\" />\n\t<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n\t<title>easymark preview</title>\n</head>\n<body>\n{body_html}\n</body>\n</html>\n"
	);
	Ok(page)
}

fn respond_ok_html(mut stream: &TcpStream, body: &str) -> Result<()> {
	let headers = format!(
		"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
		body.len()
	);
	stream.write_all(headers.as_bytes())?;
	stream.write_all(body.as_bytes())?;
	stream.flush()?;
	Ok(())
}

fn respond_not_found(mut stream: &TcpStream) -> Result<()> {
	let body = "<h1>404 Not Found</h1>";
	let headers = format!(
		"HTTP/1.1 404 Not Found\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
		body.len()
	);
	stream.write_all(headers.as_bytes())?;
	stream.write_all(body.as_bytes())?;
	stream.flush()?;
	Ok(())
}
