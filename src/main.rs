use log::{log, Level};
use std::path::Path;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

struct Response {
    code: u16,
    body: String,
}

fn main() {
    env_logger::init();
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn get_404() -> Response {
    return match fs::read_to_string("404.html") {
        Ok(file) => Response {
            code: 404,
            body: file,
        },
        Err(_) => Response {
            code: 500,
            body: "Server error".to_string(),
        },
    };
}

fn get_file(filename: &str) -> Response {
    let mut filepath = filename;
    if filepath.starts_with("/") {
        filepath = filepath.trim_start_matches("/");
    }
    let path = Path::new(&filepath);
    log!(Level::Info, "- File: {}", path.display());
    if path.exists() {
        let file = fs::read_to_string(path).unwrap();
        return Response {
            code: 200,
            body: file,
        };
    }
    get_404()
}

fn text_by_code(code: u16) -> String {
    return match code {
        200 => "OK".to_string(),
        404 => "NOT FOUND".to_string(),
        500 => "SERVER ERROR".to_string(),
        _ => "".parse().unwrap(),
    };
}

fn process_request(request: String) -> Option<String> {
    log!(Level::Info, "Request {request}");
    let parts: Vec<&str> = request.split(" ").collect();
    if parts.len() != 3 {
        return None;
    };
    let mut path = String::from(parts[1]);
    if path.ends_with("/") {
        path += "index.html"
    }
    return Some(path);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();

    let resp = match process_request(request) {
        None => Response {
            code: 400,
            body: "BAD REQUEST".to_string(),
        },
        Some(filepath) => get_file(&filepath),
    };

    let response = format!(
        "HTTP/1.1 {code} {status}\r\nContent-Length: {length}\r\n\r\n{body}",
        code = resp.code,
        status = text_by_code(resp.code),
        length = resp.body.len(),
        body = resp.body,
    );

    stream.write_all(response.as_bytes()).unwrap();
}
