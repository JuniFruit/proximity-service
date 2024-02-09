mod config;
mod request;
mod response;

use config::ServerConfig;
use request::{handle_request, parse_tcp_stream};
use request::{Request, Router};
use response::Result;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let config: ServerConfig = ServerConfig::get();
    let router = create_router();
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .expect("Server failed to start at {config.port}");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_tcp_stream(stream, &router),

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_tcp_stream(mut stream: TcpStream, router: &Router) {
    let mut req: Request = Request::default();
    parse_tcp_stream(&mut stream, &mut req);
    let response = handle_request(&mut req, router);
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write back")
}

fn create_router<'a>() -> Router<'a> {
    let mut handlers: HashMap<String, fn(&Request) -> Result<Value>> = HashMap::new();
    let paths = vec!["/test", "/business/:id"];
    handlers.insert(String::from("GET /test"), handle_get_test);
    handlers.insert(String::from("GET /business/:id"), handle_get_business);

    Router { handlers, paths }
}

fn handle_get_test(req: &Request) -> Result<Value> {
    let t = req.body.as_ref();
    println!("{:?}", t);
    let body = json!({
        "message": "Request successfull",
        "data": t
    });
    Ok(body)
}

fn handle_get_business(req: &Request) -> Result<Value> {
    let id = req.params.get("id").unwrap();

    let body = json!({
        "data": id
    });
    Ok(body)
}
