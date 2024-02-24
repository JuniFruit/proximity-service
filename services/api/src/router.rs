use crate::dbs::{BusinessData, DBConnections};
use crate::response::{Response, Result};
use crate::Request;
use serde_json::json;
use std::collections::HashMap;
use std::collections::VecDeque;

pub struct Router<'a> {
    pub routes: Vec<&'a str>,
}

impl<'a> Router<'a> {
    pub fn init() -> Router<'a> {
        let routes = vec!["GET /business/:id", "PUT /business/:id", "POST /business"];

        Router { routes }
    }
    pub async fn handle_route(
        &self,
        req: &mut Request,
        connections: &mut DBConnections,
    ) -> Result<Response> {
        let matched_path = self.path_parser(
            self.routes.clone(),
            req.path.as_ref().unwrap(),
            &mut req.params,
            req.method.as_ref().unwrap(),
        );
        if matched_path.is_none() {
            return Ok(Response::not_found(None));
        }
        let matched_path = matched_path.unwrap();

        let mut result: Result<Response> = Ok(Response::not_found(None));

        if matched_path == "GET /business/:id" {
            result = self.handle_get_business(req, connections).await;
        }

        if matched_path == "PUT /business/:id" {
            // result = s(req).await;
            result = self.handle_update_business(req, connections).await;
        }

        if matched_path == "POST /business" {
            result = self.handle_create_business(req, connections).await;
        }

        result
    }

    async fn handle_get_business(
        &self,
        req: &Request,
        connections: &mut DBConnections,
    ) -> Result<Response> {
        let id = req.params.get("id").unwrap().parse()?;
        let data = BusinessData::get_business_by_id(connections, id).await?;

        if data.is_none() {
            return Ok(Response::success(json!({"data": ""}), None));
        }
        let body = json!({
            "data": data.unwrap()
        });
        Ok(Response::success(body, None))
    }

    async fn handle_create_business(
        &self,
        req: &Request,
        connections: &mut DBConnections,
    ) -> Result<Response> {
        if req.body.is_none() {
            return Ok(Response::bad_request(Some("Missing data for new item")));
        }
        let data = req.body.as_ref().unwrap();
        let serialized = BusinessData::from_value(data);
        if serialized.is_err() {
            return Ok(Response::bad_request(Some("Invalid data for new item")));
        }
        match BusinessData::create_business(connections, serialized.unwrap()).await {
            Ok(res) => Ok(Response::success(json!({"id": res}), None)),
            Err(e) => Err(e),
        }
    }

    async fn handle_update_business(
        &self,
        req: &Request,
        connections: &mut DBConnections,
    ) -> Result<Response> {
        let id = req.params.get("id").unwrap().parse()?;
        if req.body.is_none() {
            return Ok(Response::bad_request(Some("Missing data for update")));
        }
        let data = req.body.as_ref().unwrap();
        let serialized = BusinessData::from_value(data);

        if serialized.is_err() {
            return Ok(Response::bad_request(Some("Invalid data for update")));
        }
        match BusinessData::update_business_by_id(connections, id, serialized.unwrap()).await {
            Ok(_) => Ok(Response::default()),
            Err(e) => Err(e),
        }
    }

    fn construct_params<'b>(
        &self,
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

    fn path_parser<'b>(
        &self,
        paths: Vec<&'b str>,
        requested_path: &str,
        params_map: &mut HashMap<String, String>,
        requested_method: &str,
    ) -> Option<&'b str> {
        let mut matched_path: Option<&str> = None;
        for path in paths {
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
                self.construct_params(&mut splitted_existing, &mut splitted_requested, params_map);
            }
        }
        matched_path
    }
}
