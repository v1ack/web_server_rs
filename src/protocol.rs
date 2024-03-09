pub enum HttpCodes {
    Ok,
    NotFound,
    ServerError,
    BadRequest,
}

pub struct Response {
    pub(crate) code: HttpCodes,
    pub(crate) body: String,
}

pub fn text_by_code(code: HttpCodes) -> String {
    return match code {
        HttpCodes::Ok => "200 OK".to_string(),
        HttpCodes::NotFound => "404 NOT FOUND".to_string(),
        HttpCodes::ServerError => "500 SERVER ERROR".to_string(),
        HttpCodes::BadRequest => "400 BAD REQUEST".to_string(),
    };
}
