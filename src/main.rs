// Uncomment this block to pass the first stage
use std::{
    io::{Read, Write},
    net::TcpListener,
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
                if !request_target.starts_with("/echo") {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    _stream.write(response.as_bytes()).unwrap();
                } else {
                    let echo = request_target.strip_prefix("/echo/").unwrap_or("");
                    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", echo);
                    _stream.write(response.as_bytes()).unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
