use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::time::SystemTime;

const FILE: &str = "table.html";

fn mtime_secs(path: &Path) -> u64 {
    fs::metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn respond(stream: &mut impl Write, status: &str, content_type: &str, body: &[u8]) {
    let _ = write!(
        stream,
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(body);
}

fn handle(mut stream: impl Read + Write) {
    let mut buf = [0u8; 1024];
    let n = stream.read(&mut buf).unwrap_or(0);
    let req = String::from_utf8_lossy(&buf[..n]);
    let path = req.lines().next().unwrap_or("").split_whitespace().nth(1).unwrap_or("/");

    match path {
        "/mtime" => {
            let body = mtime_secs(Path::new(FILE)).to_string();
            respond(&mut stream, "200 OK", "text/plain", body.as_bytes());
        }
        "/" | "/index.html" => {
            let content = fs::read_to_string(FILE).unwrap_or_else(|_| {
                "<p>table.html not found — run debug.sh or the html example first.</p>".into()
            });
            let body = include_str!("index.html").replace("{content}", &content);
            respond(&mut stream, "200 OK", "text/html; charset=utf-8", body.as_bytes());
        }
        _ => respond(&mut stream, "404 Not Found", "text/plain", b"not found"),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("http://127.0.0.1:8080/  (reloads when {FILE} changes)");
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handle(stream);
        }
    }
    Ok(())
}
