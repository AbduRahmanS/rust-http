use flate2::{write::GzEncoder, Compression};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    str::FromStr,
};
pub enum Method {
    GET,
    POST,
    UNKOWN,
}
#[allow(dead_code)]
pub struct HttpRequest {
    pub method: Method,
    pub request_target: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
}

impl FromStr for HttpRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let request_line = lines.next().unwrap();
        let mut parts = request_line.split_whitespace();
        let method = parts.next().unwrap().to_string();
        let request_target = parts.next().unwrap().to_string();
        let mut headers = vec![];
        let mut body = None;
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            headers.push(line.to_string());
        }
        if let Some(body_line) = lines.next() {
            body = Some(body_line.to_string());
        }
        Ok(HttpRequest {
            method: match method.as_str() {
                "GET" => Method::GET,
                "POST" => Method::POST,
                _ => Method::UNKOWN,
            },
            request_target,
            headers,
            body,
        })
    }
}

pub fn read_file(file_name: &str, dir_path: &String) -> Option<String> {
    let path = Path::new(dir_path).join(file_name);
    let file = File::open(path);
    match file {
        Err(_) => return None,
        Ok(mut f) => {
            let mut buf = String::new();
            f.read_to_string(&mut buf).unwrap();
            Some(buf)
        }
    }
}

pub fn write_file(file_name: &str, content: String, directory: &String) {
    println!("Content: {:?}", content);
    let path = Path::new(directory).join(file_name);
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    file.flush().unwrap();
}

pub fn gzip_encode(input: &str) -> Result<Vec<u8>, std::io::Error> {
    // let mut body_buf = vec![];
    // body_buf.extend_from_slice(input.as_bytes());
    // let mut encoder = GzEncoder::new(vec![], Compression::default());
    // encoder.write_all(&body_buf)?;
    // let compressed_buf = encoder.finish()?;
    // Ok(compressed_buf)
    let mut compbody = Vec::new();
    GzEncoder::new(&mut compbody, Compression::default()).write_all(input.as_bytes())?;
    Ok(compbody)
}
