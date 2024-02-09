use crate::response::Result;
use crate::response::{INTERNAL_SERVER_ERROR, NOT_FOUND, OK_RESPONSE};
use serde_json::{Map, Value};
use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::net::TcpStream;
// Very basic request struct. We're not going implement entire HTTP protocol
pub struct Request {
    pub host: Option<String>,
    pub method: Option<String>,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<String>,
    pub path: Option<String>,
    pub http_version: Option<String>,
    pub body: Option<Map<String, Value>>,
    pub params: HashMap<String, String>,
}
impl Request {
    pub fn default() -> Request {
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

    while rows.len() > 0 {
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
        if splitted[0].contains("{") {
            let parsed_body: Option<Value> = serde_json::from_str(line.unwrap()).unwrap_or(None);
            if let Some(value) = parsed_body {
                request_struct.body = Some(value.as_object().unwrap().clone());
            }
        }

        ind += 1;
    }
}

pub struct Router<'a> {
    pub handlers: HashMap<String, fn(&Request) -> Result<Value>>,
    pub paths: Vec<&'a str>,
}

fn construct_params(
    split_existing: &mut VecDeque<&str>,
    split_requested: &mut VecDeque<&str>,
    params_map: &mut HashMap<String, String>,
) {
    while split_existing.len() > 0 {
        let current_existing = split_existing.pop_front().unwrap_or("");
        let current_requested = split_requested.pop_front().unwrap_or("");
        if current_existing != current_requested {
            params_map.insert(
                current_existing[1..].to_string(),
                current_requested.to_string(),
            );
        }
    }
}

fn path_parser<'a>(
    router: &'a Router,
    requested_path: &str,
    params_map: &mut HashMap<String, String>,
) -> Option<&'a str> {
    let mut matched_path: Option<&str> = None;
    for path in &router.paths {
        let mut splitted_existing = path.split("/").collect::<VecDeque<&str>>();
        let mut splitted_requested = requested_path.split("/").collect::<VecDeque<&str>>();

        // delete empty string from start
        splitted_requested.pop_front();
        splitted_existing.pop_front();

        // start constructing params if existing path is the same length as path from client and if
        // their first values match
        if splitted_requested.len() == splitted_existing.len()
            && splitted_requested[0] == splitted_existing[0]
        {
            matched_path = Some(path);
            construct_params(&mut splitted_existing, &mut splitted_requested, params_map);
        }
    }
    matched_path
}

pub fn handle_request(req: &mut Request, router: &Router) -> String {
    if req.method.is_none() || req.path.is_none() {
        return String::from(NOT_FOUND);
    }

    let matched_path = path_parser(router, req.path.as_ref().unwrap(), &mut req.params);
    if matched_path.is_none() {
        return String::from(NOT_FOUND);
    }

    let req_key = format!("{} {}", req.method.as_ref().unwrap(), matched_path.unwrap());
    let handler = router.handlers.get(&req_key);
    if handler.is_none() {
        return String::from(NOT_FOUND);
    }
    match handler.unwrap()(req) {
        Ok(res) => {
            let content_length = format!(
                "Content-Length: {}\r\n\r\n",
                res.to_string().as_bytes().len()
            );
            let response = format!("{} {} {}", OK_RESPONSE, content_length, res);
            response
        }
        Err(e) => {
            println!("{:?}", e);
            String::from(INTERNAL_SERVER_ERROR)
        }
    }
}
