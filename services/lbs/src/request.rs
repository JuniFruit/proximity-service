use std::collections::{HashMap, VecDeque};
use std::io::Read;
use std::net::TcpStream;

use crate::ROUTES;

#[derive(Clone)]
pub struct Request<'a> {
    pub host: Option<String>,
    pub method: Option<String>,
    pub user_agent: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<String>,
    pub path: Option<String>,
    pub http_version: Option<String>,
    pub query: HashMap<String, String>,
    pub matched_path: Option<&'a str>,
    pub params: HashMap<String, String>,
}
impl<'a> Request<'a> {
    pub fn default<'b>() -> Request<'b> {
        Request {
            host: Some(String::from("unknown")),
            method: Some(String::from("unknown")),
            user_agent: Some(String::from("unknown")),
            path: Some(String::from("/")),
            content_type: Some(String::from("application/json")),
            content_length: Some(String::from("0")),
            http_version: Some(String::from("HTTP/1.1")),
            query: HashMap::default(),
            matched_path: None,
            params: HashMap::default(),
        }
    }
}
pub fn parse_tcp_stream(stream: &mut TcpStream, request_struct: &mut Request) {
    let mut buffer = [0; 1024];

    let request_raw;
    if let Ok(size) = stream.read(&mut buffer) {
        request_raw = String::from_utf8_lossy(&buffer[..size])
    } else {
        println!("Error reading incoming stream");
        return;
    }

    println!("Raw: {:?}", request_raw);

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
            if request_struct.path.is_some() && request_struct.method.is_some() {
                let matched_path = path_parser(
                    request_struct.path.as_ref().unwrap(),
                    &mut request_struct.params,
                    request_struct.method.as_ref().unwrap(),
                );
                request_struct.matched_path = matched_path;
                parse_query(
                    request_struct.path.as_ref().unwrap(),
                    &mut request_struct.query,
                )
            }
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

        ind += 1;
    }
}

fn parse_query(path: &str, query_map: &mut HashMap<String, String>) {
    let query_raw = path.split_once('?');
    if query_raw.is_none() {
        return;
    }

    let pairs: Vec<&str> = query_raw.unwrap().1.split('&').collect();

    for pair in pairs {
        let splitted = pair.split_once('=').unwrap_or(("", ""));
        query_map.insert(splitted.0.to_string(), splitted.1.to_string());
    }
}
fn construct_params<'b>(
    split_existing: &mut VecDeque<&'b str>,
    split_requested: &mut VecDeque<&'b str>,
    params_map: &mut HashMap<String, String>,
) {
    while !split_existing.is_empty() {
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
    req_path: &str,
    params_map: &mut HashMap<String, String>,
    requested_method: &str,
) -> Option<&'a str> {
    let mut matched_path: Option<&str> = None;
    for path in ROUTES {
        let mut requested_path = req_path;
        if req_path.contains('?') {
            requested_path = req_path.split_once('?').unwrap().0;
        }

        let mut splitted_existing = path.split('/').collect::<VecDeque<&str>>();
        let mut splitted_requested = requested_path.split('/').collect::<VecDeque<&str>>();

        // delete empty string from start
        splitted_requested.pop_front();
        let existing_method = splitted_existing.pop_front(); // first item is method
        if requested_method.trim() != existing_method.unwrap().trim() {
            continue;
        }

        // start constructing params if existing path is the same length as path from client and if
        // their first values match
        if splitted_requested.len() == splitted_existing.len()
            && splitted_requested[0].trim() == splitted_existing[0].trim()
        {
            matched_path = Some(path);
            construct_params(&mut splitted_existing, &mut splitted_requested, params_map);
        }
    }
    matched_path
}
