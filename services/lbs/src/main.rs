mod config;
mod dbs;
mod request;
mod response;

use config::ServerConfig;
use dbs::{DBConnections, Result};
use redis::{
    geo::{RadiusOptions, RadiusOrder, Unit},
    AsyncCommands, Pipeline,
};
use request::{parse_tcp_stream, Request};
use response::Response;
use serde_json::json;
use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use crate::dbs::BusinessData;

pub const ROUTES: [&str; 1] = ["GET /search"];

#[tokio::main]
async fn main() -> Result<()> {
    let config: ServerConfig = ServerConfig::get();
    let mut connections = DBConnections::init(&config).await?;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .expect("Server failed to start at {config.port}");
    println!("Server is listening at {}", config.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_tcp_stream(stream, &mut connections).await;
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}

async fn handle_tcp_stream(mut stream: TcpStream, connections: &mut DBConnections) -> usize {
    let mut req = Request::default();
    parse_tcp_stream(&mut stream, &mut req);
    let response = handle_request(&req, connections).await;

    match response {
        Ok(res) => {
            println!("{:?}", res.to_response_string());

            stream
                .write(res.to_response_string().as_bytes())
                .unwrap_or_default()
        }
        Err(e) => {
            println!("{:?}", e);
            stream
                .write(Response::internal(None).to_response_string().as_bytes())
                .unwrap_or_default()
        }
    }
}

async fn handle_request<'a>(
    req: &Request<'a>,
    connections: &mut DBConnections,
) -> Result<Response> {
    if req.matched_path.is_none() {
        return Ok(Response::not_found(None));
    }
    println!(
        "{} {} was requested",
        req.method.as_ref().unwrap(),
        req.path.as_ref().unwrap()
    );

    match req.matched_path.unwrap() {
        "GET /search" => handle_get_area_businesses(req, connections).await,
        _ => Ok(Response::not_found(None)),
    }
}

async fn handle_get_area_businesses<'a>(
    req: &Request<'a>,
    conns: &mut DBConnections,
) -> Result<Response> {
    if !req.query.contains_key("lon") || !req.query.contains_key("lat") {
        return Ok(Response::bad_request(Some("Lon or lat was not specified")));
    }
    let lat = req.query.get("lat").unwrap().parse::<f64>()?;
    let lon = req.query.get("lon").unwrap().parse::<f64>()?;
    let radius = req
        .query
        .get("radius")
        .unwrap_or(&"500".to_string())
        .parse::<f64>()?;

    let ids: Vec<String> = conns
        .redis_geo
        .connection
        .geo_radius(
            "world",
            lon,
            lat,
            radius,
            Unit::Meters,
            RadiusOptions::default().order(RadiusOrder::Asc),
        )
        .await?;
    if ids.is_empty() {
        return Ok(Response::success(json!({"businesses": []}), None));
    }
    if ids.len() == 1 {
        let mut single_item: BusinessData =
            conns.redis_business.connection.hgetall(&ids[0]).await?;
        single_item.id = Some(ids[0].parse::<u64>()?);
        return Ok(Response::success(
            json!({"businesses": vec![single_item]}),
            None,
        ));
    }

    let mut pipe = Pipeline::new();
    for id in &ids {
        pipe.hgetall(id);
    }
    let mut businesses: Vec<BusinessData> = pipe
        .query_async(&mut conns.redis_business.connection)
        .await?;

    for (i, business) in businesses.iter_mut().enumerate() {
        business.id = Some(ids[i].parse::<u64>()?);
    }

    let body = json!({
        "businesses": businesses
    });

    Ok(Response::success(body, None))
}
