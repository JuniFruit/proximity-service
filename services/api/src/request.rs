use crate::dbs::{MongoDb, RedisBusiness};
use crate::response::Response;
use crate::router::Router;
use serde_json::Value;
use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::net::TcpStream;

// Very basic request struct. We're not going implement entire HTTP protocol
#[derive(Clone)]
pub struct Request<'a> {
    pub host: Option<String>,
    pub method: Option<String>,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<String>,
    pub path: Option<String>,
    pub http_version: Option<String>,
    pub body: Option<Value>,
    pub params: HashMap<String, String>,
    pub mongo: Option<&'a MongoDb>,
    pub redis_business: Option<&'a RedisBusiness>,
}
impl<'a> Request<'a> {
    pub fn default() -> Request<'a> {
        Request {
            host: Some(String::from("unknown")),
            method: Some(String::from("unknown")),
            user_agent: Some(String::from("unknown")),
            path: Some(String::from("/")),
            content_type: Some(String::from("application/json")),
            content_length: Some(String::from("0")),
            http_version: Some(String::from("HTTP/1.1")),
            body: None,
            params: HashMap::new(),
            mongo: None,
            redis_business: None,
        }
    }
}
pub fn parse_tcp_stream(stream: &mut TcpStream, request_struct: &mut Request) {
    let mut buffer = [0; 1024];

    let request_raw;

    match stream.read(&mut buffer) {
        Ok(size) => request_raw = String::from_utf8_lossy(&buffer[..size]),
        Err(e) => {
            println!("Error reading incoming stream: {:?}", e);
            return;
        }
    };
    let mut rows = request_raw.split("\r\n").collect::<VecDeque<&str>>();
    let mut ind = 0;

    while !rows.is_empty() {
        let line = rows.pop_front();
        if line.is_none() {
            continue;
        };
        if line.unwrap() == "" {
            continue;
        }
        let splitted: Vec<&str> = line.unwrap().split_whitespace().collect();
        if ind == 0 {
            request_struct.method = Some(splitted[0].to_string());
            request_struct.path = Some(splitted[1].to_string());
            request_struct.http_version = Some(splitted[2].to_string());
        }
        if splitted[0] == "Host:" {
            request_struct.host = Some(splitted[1].to_string());
        }
        if splitted[0] == "Content-Type:" {
            request_struct.content_type = Some(splitted[1].to_string());
        }
        if splitted[0] == "User-Agent:" {
            request_struct.user_agent = Some(splitted[1].to_string());
        }
        if splitted[0] == "Content-Length:" {
            request_struct.content_length = Some(splitted[1].to_string());
        }
        if splitted[0].contains('{') {
            let parsed_body: Option<Value> = serde_json::from_str(line.unwrap()).unwrap_or(None);
            if let Some(value) = parsed_body {
                request_struct.body = Some(value);
            }
        }

        ind += 1;
    }
}

pub async fn handle_request<'a>(req: &mut Request<'a>, router: &Router<'a>) -> String {
    if req.method.is_none() || req.path.is_none() {
        return Response::not_found(None).to_response_string();
    }

    let response = router.handle_route(req).await;

    match response {
        Ok(res) => res.to_response_string(),
        Err(e) => {
            println!("{:?}", e);
            Response::internal(None).to_response_string()
        }
    }
}
