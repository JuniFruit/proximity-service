use serde_json::{json, Value};
use std::{
    error::Error,
    fmt::{self, format},
};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n";
const BAD_REQUEST: &str = "HTTP/1.1 400 BAD_REQUEST\r\nContent-Type: application/json\r\n";
const UNAUTHORIZED: &str = "HTTP/1.1 401 UNAUTHORIZED\r\nContent-Type: application/json\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n";
const INTERNAL_SERVER_ERROR: &str =
    "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Type: application/json\r\n";

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Response {
    pub status: u16,
    pub body: Value,
}

#[allow(dead_code)]
impl Response {
    pub fn default() -> Response {
        Response {
            status: 200,
            body: json!({"message": "Request successfull!"}),
        }
    }
    pub fn bad_request(message: Option<&str>) -> Response {
        Response {
            status: 400,
            body: json!({"message": message.unwrap_or("Invalid request")}),
        }
    }
    pub fn unauthorized(message: Option<&str>) -> Response {
        Response {
            status: 401,
            body: json!({"message": message.unwrap_or("Not authorized")}),
        }
    }
    pub fn success(data: Value, status: Option<u16>) -> Response {
        Response {
            status: status.unwrap_or(200),
            body: data,
        }
    }
    pub fn internal(data: Option<Value>) -> Response {
        Response {
            status: 500,
            body: data.unwrap_or(json!({"message": "Something went wrong!"})),
        }
    }
    pub fn not_found(message: Option<&str>) -> Response {
        Response {
            status: 404,
            body: json!({"message": message.unwrap_or("Resource not found!")}),
        }
    }
    pub fn to_response_string(&self) -> String {
        if self.status == 200 {
            return self.construct_response_string(OK_RESPONSE);
        }
        if self.status == 400 {
            return self.construct_response_string(BAD_REQUEST);
        }
        if self.status == 401 {
            return self.construct_response_string(UNAUTHORIZED);
        }
        if self.status == 404 {
            return self.construct_response_string(NOT_FOUND);
        }
        self.construct_response_string(INTERNAL_SERVER_ERROR)
    }
    fn construct_response_string(&self, response_type: &str) -> String {
        let allow_origin = "Access-Control-Allow-Origin: *\r\n".to_string();
        let allow_methods = "Access-Control-Allow-Methods: GET, POST, PUT\r\n".to_string();
        let allow_headers = "Access-Control-Allow-Headers: DNT,User-Agent,X-Requested-With,If-Modified-Since,Cache-Control,Content-Type,Range\r\n".to_string();
        let content_length = format!(
            "Content-Length: {}\r\n\r\n",
            self.body.to_string().as_bytes().len()
        );
        let server = format!("Server: {}\r\n", "Rust");
        let response = format!(
            "{}{}{}{}{}{}{}\r\n\r\n",
            response_type,
            server,
            allow_origin,
            allow_methods,
            allow_headers,
            content_length,
            self.body
        );
        response
    }
}

#[derive(Debug, Clone)]
pub struct InternalError;

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal server error")
    }
}
