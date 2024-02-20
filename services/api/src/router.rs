use crate::mongo::BusinessData;
use crate::response::{Response, Result};
use crate::Request;
use serde_json::json;
use std::{collections::HashMap, future::Future, pin::Pin};

pub type RouteHandler = Box<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>>>>>;

pub struct Router {
    pub handlers: HashMap<String, RouteHandler>,
}

impl Router {
    pub fn init() -> Router {
        let mut handlers_map: HashMap<String, RouteHandler> = HashMap::default();
        handlers_map.insert(
            String::from("GET /business/:id"),
            Box::new(|req: Request| Box::pin(handle_get_business(req))),
        );

        handlers_map.insert(
            String::from("POST /business"),
            Box::new(|req: Request| Box::pin(handle_create_business(req))),
        );

        handlers_map.insert(
            String::from("PUT /business/:id"),
            Box::new(|req: Request| Box::pin(handle_update_business(req))),
        );

        Router {
            handlers: handlers_map,
        }
    }
}

async fn handle_get_business(req: Request) -> Result<Response> {
    let id = req.params.get("id").unwrap().parse()?;

    let data = BusinessData::get_business_by_id(&req.mongo.unwrap(), id).await;
    if data.is_err() {
        println!("{:?}", data.err());
        return Ok(Response::internal(None));
    }
    let body = json!({
        "data": data.unwrap()
    });
    Ok(Response::success(body, None))
}

async fn handle_create_business(req: Request) -> Result<Response> {
    if req.body.is_none() {
        return Ok(Response::bad_request(Some("Missing data for new item")));
    }
    let data = req.body.unwrap();
    let serialized = BusinessData::from_value(data);
    if serialized.is_err() {
        return Ok(Response::bad_request(Some("Invalid data for new item")));
    }
    match BusinessData::create_business(&req.mongo.unwrap(), serialized.unwrap()).await {
        Ok(res) => Ok(Response::success(json!({"id": res}), None)),
        Err(_) => Ok(Response::internal(None)),
    }
}

async fn handle_update_business(req: Request) -> Result<Response> {
    let id = req.params.get("id").unwrap().parse()?;
    if req.body.is_none() {
        return Ok(Response::bad_request(Some("Missing data for update")));
    }
    let data = BusinessData::from_value(req.body.unwrap());
    if data.is_err() {
        return Ok(Response::bad_request(Some("Invalid data for update")));
    }
    match BusinessData::update_business_by_id(&req.mongo.unwrap(), id, data.unwrap()).await {
        Ok(_) => Ok(Response::default()),
        Err(_) => Ok(Response::internal(None)),
    }
}
