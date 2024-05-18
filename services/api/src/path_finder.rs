use serde::{Deserialize, Serialize};
use std::{collections::HashMap, f64::consts::PI};

pub type LatLonPos = (f64, f64);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApiElements {
    ApiWay {
        id: u64,
        nodes: Vec<u64>,
        r#type: String,
    },
    Node {
        id: u64,
        lat: f64,
        lon: f64,
        r#type: String,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OverpassApiResponse {
    pub version: f32,
    pub generator: String,
    pub elements: Vec<ApiElements>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiWay {
    pub r#type: String,
    pub id: u64,
    pub nodes: Vec<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub r#type: String,
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
}

pub struct GraphNode {
    pub data: Node,
    pub visited: bool,
    pub edges: Vec<u64>,
}

impl GraphNode {
    pub fn init(node: Node) -> Self {
        GraphNode {
            data: node,
            visited: false,
            edges: vec![],
        }
    }

    pub fn add_edge_to(&mut self, node_id: u64) {
        self.edges.push(node_id)
    }
}

pub struct Graph {
    start_node: Option<u64>,
    nodes: HashMap<u64, GraphNode>,
    target_node: Option<u64>,
}

impl Graph {
    pub fn init() -> Self {
        Graph {
            start_node: None,
            nodes: HashMap::default(),
            target_node: None,
        }
    }
    pub fn get_node(&mut self, id: &u64) -> Option<&mut GraphNode> {
        self.nodes.get_mut(id)
    }
    pub fn add_node(&mut self, node: GraphNode) {
        self.nodes.insert(node.data.id, node);
    }

    pub fn interconnect(&mut self, start_node_id: &u64, target_node_id: &u64) {
        let start_node = self.get_node(start_node_id).take();
        if start_node.is_none() {
            return;
        }
        start_node.unwrap().add_edge_to(target_node_id.to_owned());
        let target_node = self.get_node(target_node_id).take();
        if target_node.is_none() {
            return;
        }
        target_node.unwrap().add_edge_to(start_node_id.to_owned())
    }

    pub fn find_closest_start(&mut self, start: LatLonPos) {
        for (id, node) in self.nodes.iter() {
            if is_within_radius(start, (node.data.lat, node.data.lon), 1000) {
                self.start_node = Some(id.to_owned());
                return;
            }
        }
    }

    pub fn find_closest_target(&mut self, target: LatLonPos) {
        for (id, node) in self.nodes.iter() {
            if is_within_radius(target, (node.data.lat, node.data.lon), 1000) {
                self.target_node = Some(id.to_owned());
                return;
            }
        }
    }
}

fn is_within_radius(pos1: LatLonPos, pos2: LatLonPos, radius: u16) -> bool {
    let k_equatorial_radius_in_metres: u32 = 6378137;
    let kpi_double = PI;
    let k_degrees_to_radians_double = kpi_double / 180.0;

    let lon1 = pos1.1 * k_degrees_to_radians_double;
    let lat1 = pos1.0 * k_degrees_to_radians_double;
    let lon2 = pos2.1 * k_degrees_to_radians_double;
    let lat2 = pos2.0 * k_degrees_to_radians_double;
    let cos_angle = lat1.sin() * lat2.sin() + lat1.cos() * lat2.cos() * (lon2 - lon1).cos();

    /*
    Inaccurate trig functions can cause cos_angle to be a tiny amount
    greater than 1 if the two positions are very close. That in turn causes
    acos to give a domain error and return the special floating point value
    -1.#IND000000000000, meaning 'indefinite'. Observed on VS2008 on 64-bit Windows.
    */
    if cos_angle >= 0.0 {
        return true;
    }

    let angle = cos_angle.acos();
    let distance = angle * k_equatorial_radius_in_metres as f64;

    radius as f64 <= distance
}

fn connect_nodes(way: &ApiWay, graph: &mut Graph) {
    let mut prev_node: Option<&u64> = None;
    for node_id in way.nodes.iter() {
        if prev_node.is_some() {
            graph.interconnect(prev_node.unwrap(), node_id);
        }
        prev_node = Some(node_id);
    }
}

// A* path finding algorithm
fn find_path(mut graph: Graph) {
    let open_set: Vec<u64> = vec![graph.start_node.unwrap()];
}

pub fn create_path(elements: Vec<ApiElements>, start_pos: LatLonPos, target_pos: LatLonPos) {
    let mut graph = Graph::init();
    for el in elements.iter() {
        match el {
            ApiElements::Node {
                id,
                lat,
                lon,
                r#type,
            } => {
                let node = Node {
                    id: id.to_owned(),
                    lat: lat.to_owned(),
                    lon: lon.to_owned(),
                    r#type: r#type.to_owned(),
                };
                let graph_node = GraphNode::init(node.clone());
                if start_pos.0 == graph_node.data.lat && start_pos.1 == graph_node.data.lon {
                    graph.start_node = Some(graph_node.data.id)
                }
                if target_pos.0 == graph_node.data.lat && target_pos.1 == graph_node.data.lon {
                    graph.target_node = Some(graph_node.data.id)
                }

                graph.add_node(graph_node);
            }
            ApiElements::ApiWay { id, nodes, r#type } => connect_nodes(
                &ApiWay {
                    r#type: r#type.to_owned(),
                    id: id.to_owned(),
                    nodes: nodes.to_owned(),
                },
                &mut graph,
            ),
        }
    }

    if graph.start_node.is_none() {
        graph.find_closest_start(start_pos);
    }

    if graph.target_node.is_none() {
        graph.find_closest_target(target_pos);
    }
    println!("{:?}", graph.start_node);
    let result = find_path(graph);
}
