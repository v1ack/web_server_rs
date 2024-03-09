use crate::protocol::{HttpCodes, Response};
use log::{log, Level};
use std::fs;
use std::path::Path;

fn get_404() -> Response {
    return match fs::read_to_string("404.html") {
        Ok(file) => Response {
            code: HttpCodes::NotFound,
            body: file,
        },
        Err(_) => Response {
            code: HttpCodes::ServerError,
            body: "Server error".to_string(),
        },
    };
}

pub fn get_file(filename: &str) -> Response {
    let mut filepath = filename;
    if filepath.contains("../") {
        return Response {
            code: HttpCodes::BadRequest,
            body: "Relative path not allowed".to_string(),
        };
    }
    if filepath.starts_with("/") {
        filepath = filepath.trim_start_matches("/");
    }
    let path = Path::new(&filepath);
    log!(Level::Info, "- File: {}", path.display());
    if !path.exists() {
        return get_404();
    }
    let file = fs::read_to_string(path).unwrap();
    Response {
        code: HttpCodes::Ok,
        body: file,
    }
}
