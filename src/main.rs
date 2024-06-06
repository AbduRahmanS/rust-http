// Uncomment this block to pass the first stage
use std::{
    fs::{self, File},
    io::{Read, Write},
    net::TcpListener,
    path::Path,
    thread,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                thread::spawn(|| handle_conn(_stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_conn(mut _stream: std::net::TcpStream) {
    let mut buffer = [0; 1024];
    _stream.read(&mut buffer).unwrap();

    let header = std::str::from_utf8(&buffer).unwrap();
    let request_target = header
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("");
    println!("Target : {}", request_target);
    if request_target == '/'.to_string() {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        _stream.write(response.as_bytes()).unwrap();
    } else if request_target.starts_with("/echo") {
        let echo = request_target.strip_prefix("/echo/").unwrap_or("");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo.len(),
            echo
        );
        _stream.write(response.as_bytes()).unwrap();
    } else if request_target == "/user-agent".to_string() {
        let headers = header.lines().skip(1).collect::<Vec<&str>>();
        let user_agent = headers
            .iter()
            .find(|&x| x.starts_with("User-Agent"))
            .unwrap_or(&"User-Agent: Unknown")
            .strip_prefix("User-Agent: ")
            .unwrap();
        let res = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            user_agent.len(),
            user_agent
        );
        _stream.write(res.as_bytes()).unwrap();
    } else if request_target.starts_with("/files") {
        let file_name = request_target.replace("/files/", "");
        let dir_path = Path::new("/tmp");
        let content = read_file(&file_name, dir_path);

        match content {
            Some(c) => {
                let res = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", c.len(), c);
                _stream.write(res.as_bytes()).unwrap();
            }
            None => {
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                _stream.write(response.as_bytes()).unwrap();
            }
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        _stream.write(response.as_bytes()).unwrap();
    }
}

fn read_file(file_name: &str, dir_path: &Path) -> Option<String> {
    let entries = fs::read_dir(dir_path).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if let Some(content) = read_file(file_name, &path) {
                return Some(content);
            }
        } else if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .contains(file_name)
        {
            let mut file = File::open(path).unwrap();
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            return Some(buf);
        }
    }
    None
}
