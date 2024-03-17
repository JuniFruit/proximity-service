use serde_json::{json, Value};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n";
const BAD_REQUEST: &str = "HTTP/1.1 400 BAD_REQUEST\r\nContent-Type: application/json\r\n";
const UNAUTHORIZED: &str = "HTTP/1.1 401 UNAUTHORIZED\r\nContent-Type: application/json\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n";
const INTERNAL_SERVER_ERROR: &str =
    "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Type: application/json\r\n";

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
        let content_length = format!(
            "Content-Length: {}\r\n\r\n",
            self.body.to_string().as_bytes().len()
        );
        let server = format!("Server: {}\r\n", "Rust");
        let response = format!(
            "{}{}{}{}\r\n\r\n",
            response_type, server, content_length, self.body
        );
        response
    }
}
