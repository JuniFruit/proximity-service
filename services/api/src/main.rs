mod config;
mod dbs;
mod request;
mod response;
mod router;

use config::ServerConfig;
use dbs::DBConnections;
use request::Request;
use request::{handle_request, parse_tcp_stream};
use response::Result;
use router::Router;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let config: ServerConfig = ServerConfig::get();
    let connections = DBConnections::init(&config).await?;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .expect("Server failed to start at {config.port}");
    println!("Server is listening at {}", config.port);
    let router = Router::init();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_tcp_stream(stream, &router, &connections).await;
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

async fn handle_tcp_stream<'a>(
    mut stream: TcpStream,
    router: &Router<'a>,
    db_clients: &DBConnections,
) {
    println!("New incoming request...");
    let mut req: Request = Request::default();
    req.mongo = Some(&db_clients.mongo);
    req.redis_business = Some(&db_clients.redis_business);
    parse_tcp_stream(&mut stream, &mut req);
    let response = handle_request(&mut req, router).await;
    println!(
        "{} {} was requested",
        req.method.unwrap(),
        req.path.unwrap()
    );
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write back")
}
