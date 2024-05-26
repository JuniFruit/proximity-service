use crate::dbs::{BusinessData, DBConnections};
use crate::path_finder::{create_path, OverpassApiResponse};
use crate::response::{Response, Result};
use crate::Request;
use serde_json::json;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::future::IntoFuture;
use tokio::task::spawn_blocking;

pub struct Router<'a> {
    pub routes: Vec<&'a str>,
}

impl<'a> Router<'a> {
    pub fn init() -> Router<'a> {
        let routes = vec![
            "GET /api/business/:id",
            "PUT /api/business/:id",
            "POST /api/business",
            "POST /api/createRoute",
        ];

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

        match matched_path {
            "GET /api/business/:id" => self.handle_get_business(req, connections).await,
            "PUT /api/business/:id" => self.handle_update_business(req, connections).await,
            "POST /api/business" => self.handle_create_business(req, connections).await,
            "POST /api/createRoute" => self.handle_calculate_route(req).await,
            _ => Ok(Response::not_found(None)),
        }
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

    async fn handle_calculate_route(&self, req: &Request) -> Result<Response> {
        let data = req.body.as_ref();
        if data.is_none() {
            return Ok(Response::bad_request(Some(
                "Data is not present in the request",
            )));
        }
        let mut data = data.unwrap().clone();
        let area = data["area"].take();
        let target = data["target"].take();
        let origin = data["origin"].take();

        if !target.is_array() || !origin.is_array() {
            return Ok(Response::bad_request(Some(
                "Target or origin were not correctly specified",
            )));
        }

        if !area.is_array() {
            return Ok(Response::bad_request(Some(
                "Bounding box area coordinates are missing in the request",
            )));
        }
        let coords = area.as_array().unwrap();
        let mut is_valid = coords.len() == 4;

        coords.iter().for_each(|item| {
            if !item.is_f64() {
                is_valid = false;
            }
        });
        if !is_valid {
            return Ok(Response::bad_request(Some(
                "Invalid format for coordinates",
            )));
        }

        let query = format!(
            "[out:json];(
        way[highway][highway!='footway'][highway!='street_lamp'][highway!='steps'][highway!='pedestrian'][highway!='track'][highway!='path'][footway!='*']
        ({},{},{},{});
        node(w);

    );
    out skel;",
            coords[0], coords[1], coords[2], coords[3]
        );

        let req_client = reqwest::Client::new();
        let overpass_api = "https://overpass-api.de/api/interpreter";

        let map_response = req_client
            .post(overpass_api)
            .header("Content-Type", "application/json")
            .body(query)
            .send()
            .await?;
        let res_status = map_response.status();
        if !res_status.is_success() {
            return Ok(Response::bad_request(Some(&format!(
                "{}: Failed to get map data from server",
                res_status.as_str()
            ))));
        }
        let res: OverpassApiResponse = map_response.json().await?;
        if res.elements.is_empty() {
            return Ok(Response::not_found(Some("Could not construct the path")));
        }

        let path_finder_thread = spawn_blocking(move || {
            let path = create_path(
                res.elements,
                (
                    origin.as_array().unwrap()[0].as_f64().unwrap(),
                    origin.as_array().unwrap()[1].as_f64().unwrap(),
                ),
                (
                    target.as_array().unwrap()[0].as_f64().unwrap(),
                    target.as_array().unwrap()[1].as_f64().unwrap(),
                ),
            );

            if path.is_err() {
                return Response::internal(Some(json!({"message": path.err()})));
            }

            Response::success(json!({"path": path.unwrap()}), None)
        });
        let result = path_finder_thread.into_future().await?;
        Ok(result)
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
