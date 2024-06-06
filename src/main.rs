// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
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
    let headers = header.lines().skip(1).collect::<Vec<&str>>();
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
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        _stream.write(response.as_bytes()).unwrap();
    }
}
