use rust_http::*;
use std::{
    env::args,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    str::FromStr,
    thread,
};

fn main() {
    println!("Server starting on port 4221");
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
    let mut buffer = [0; 1024]; // A buffer with a fixed size
    let bytes_read = _stream.read(&mut buffer).unwrap_or(0); // Read into the buffer, get the number of bytes read

    // Convert only the part of the buffer that contains data to a String
    let req_string = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
    let request = HttpRequest::from_str(&req_string).unwrap();

    let mut directory: Option<String> = None;
    if args().len() > 1 {
        if std::env::args().nth(1).expect("no pattern given") == "--directory" {
            directory = Some(args().nth(2).expect("no pattern given"));
        } else {
            panic!()
        }
    }

    match request.method {
        Method::GET => handle_get(
            &mut _stream,
            &request.request_target,
            &request.headers,
            directory,
        ),
        Method::POST => handle_post(
            &mut _stream,
            &request.request_target,
            request.body.unwrap(),
            directory,
        ),
        Method::UNKOWN => {
            let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
            _stream.write(response.as_bytes()).unwrap();
        }
    }
}

fn handle_get(
    stream: &mut TcpStream,
    request_target: &str,
    headers: &Vec<String>,
    directory: Option<String>,
) {
    if request_target == '/'.to_string() {
        let response = "HTTP/1.1 200 OK\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    } else if request_target.starts_with("/echo") {
        let accept_encoding = headers
            .iter()
            .find(|&x| x.to_lowercase().starts_with("accept-encoding"))
            .unwrap_or(&"accept-encoding:  ".to_string())
            .to_lowercase();
        let encoding = accept_encoding
            .strip_prefix("accept-encoding: ")
            .unwrap_or("");
        let echo = request_target.strip_prefix("/echo/").unwrap_or("");
        let encoded = gzip_encode(echo).unwrap();
        println!("-----------Encoded : {:?}", encoded);
        if encoding.contains("gzip") {
            let res = format!(
                "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
                encoded.len(),
            );
            stream.write_all(res.as_bytes()).unwrap();
            stream.write_all(&encoded).unwrap();
        } else {
            let res = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                echo.len(),
                echo
            );
            stream.write_all(res.as_bytes()).unwrap();
        };
    } else if request_target == "/user-agent".to_string() {
        let user_agent = headers
            .iter()
            .find(|&x| x.to_lowercase().starts_with("user-agent"))
            .unwrap()
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
    body: String,
    directory: Option<String>,
) {
    if request_target.starts_with("/files") {
        let file_name = request_target.replace("/files/", "");
        write_file(&file_name, body, &directory.expect("Directory not set"));
        let response = "HTTP/1.1 201 Created\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
        stream.write(response.as_bytes()).unwrap();
    }
}
