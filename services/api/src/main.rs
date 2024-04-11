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
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let config: ServerConfig = ServerConfig::get();
    let mut connections = DBConnections::init(&config).await?;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .expect("Server failed to start at {config.port}");
    println!("Server is listening at {}", config.port);
    let router = Router::init();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_tcp_stream(stream, &router, &mut connections).await;
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
    db_clients: &mut DBConnections,
) {
    println!("New incoming request...");
    let mut req: Request = Request::default();
    let start_time = Instant::now();
    parse_tcp_stream(&mut stream, &mut req);
    let response = handle_request(&mut req, router, db_clients).await;
    println!("Time took {:?}", start_time.elapsed());
    stream
        .write_all(response.as_bytes())
        .expect("Failed to write back")
}
