// Uncomment this block to pass the first stage
use std::{
    env::args,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
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

fn handle_conn(mut _stream: TcpStream) {
    let mut buffer = [0; 1024];
    _stream.read(&mut buffer).unwrap();

    let mut directory: Option<String> = None;
    if args().len() > 1 {
        if std::env::args().nth(1).expect("no pattern given") == "--directory" {
            directory = Some(args().nth(2).expect("no pattern given"));
        } else {
            panic!()
        }
    }

    let req = std::str::from_utf8(&buffer).unwrap();
    let headers = req.lines().skip(1).collect::<Vec<&str>>();

    let method = req
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap();
    let request_target = req
        .lines()
        .next()
        .unwrap_or("")
        .split_whitespace()
        .nth(1)
        .unwrap_or("");

    let body = req.split("\r\n\r\n").nth(1).unwrap_or("");
    println!("Body: {}", body);

    match method {
        "GET" => handle_get(&mut _stream, request_target, headers, directory),
        "POST" => handle_post(&mut _stream, request_target, body, directory),
        _ => {
            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
            _stream.write(response.as_bytes()).unwrap();
        }
    }
}

fn handle_get(
    stream: &mut TcpStream,
    request_target: &str,
    headers: Vec<&str>,
    directory: Option<String>,
) {
    if request_target == '/'.to_string() {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    } else if request_target.starts_with("/echo") {
        let echo = request_target.strip_prefix("/echo/").unwrap_or("");
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            echo.len(),
            echo
        );
        stream.write(response.as_bytes()).unwrap();
    } else if request_target == "/user-agent".to_string() {
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
        stream.write(res.as_bytes()).unwrap();
    } else if request_target.starts_with("/files") {
        let file_name = request_target.replace("/files/", "");
        let content = read_file(&file_name, &directory.unwrap());

        match content {
            Some(c) => {
                let res = format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", c.len(), c);
                stream.write(res.as_bytes()).unwrap();
            }
            None => {
                let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
        }
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }
}

fn handle_post(
    stream: &mut TcpStream,
    request_target: &str,
    body: &str,
    directory: Option<String>,
) {
    if request_target.starts_with("/files") {
        let file_name = request_target.replace("/files/", "");
        write_file(&file_name, body, &directory.unwrap());
        let response = "HTTP/1.1 201 Created\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }
}
fn read_file(file_name: &str, dir_path: &String) -> Option<String> {
    let path = Path::new(dir_path).join(file_name);
    let mut file = File::open(path).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    match buf {
        s if s.is_empty() => None,
        _ => Some(buf),
    }
}

fn write_file(file_name: &str, content: &str, directory: &String) {
    let path = Path::new(directory).join(file_name);
    File::create(path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
}
