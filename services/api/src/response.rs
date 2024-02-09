use serde_json::{Map, Value};
use std::fmt;

pub const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n";
pub const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
pub const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

pub struct Response {
    pub status: u16,
    pub body: Map<String, Value>,
}
pub type Result<T> = std::result::Result<T, InternalError>;

#[derive(Debug, Clone)]
pub struct InternalError;

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal server error")
    }
}
