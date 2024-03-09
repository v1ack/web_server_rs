mod files;
mod protocol;

use crate::protocol::*;
use log::{log, Level};
use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    env_logger::init();
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("7878".to_string());
    let address = format!("{host}:{port}");
    log!(Level::Warn, "Starting on {address}");
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
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
            code: HttpCodes::BadRequest,
            body: "BAD REQUEST".to_string(),
        },
        Some(filepath) => files::get_file(&filepath),
    };

    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Length: {length}\r\n\r\n{body}",
        status = text_by_code(resp.code),
        length = resp.body.len(),
        body = resp.body,
    );

    stream.write_all(response.as_bytes()).unwrap();
}
