use serde::{Deserialize, Serialize};
use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    error::Error,
    f64::consts::PI,
    rc::Rc,
};

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

#[derive(Clone, Debug)]
pub struct GraphNode {
    pub data: Node,
    pub visited: bool,
    pub distance_from_start: f64,
    pub parent_id: Option<u64>,
    pub edges: Vec<u64>,
    pub f_score: f64,
}

impl GraphNode {
    pub fn init(node: Node) -> Rc<RefCell<GraphNode>> {
        Rc::new(RefCell::new(GraphNode {
            f_score: 0.0,
            data: node,
            visited: false,
            parent_id: None,
            distance_from_start: f64::INFINITY,
            edges: vec![],
        }))
    }

    pub fn add_edge_to(&mut self, node_id: u64) {
        self.edges.push(node_id)
    }
}

pub struct Graph {
    start_node: Option<u64>,
    nodes: HashMap<u64, Rc<RefCell<GraphNode>>>,
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
    pub fn get_node(&self, id: &u64) -> Option<&Rc<RefCell<GraphNode>>> {
        self.nodes.get(id)
    }
    pub fn add_node(&mut self, node: Rc<RefCell<GraphNode>>) {
        let id = node.borrow().data.id;
        self.nodes.insert(id, node);
    }

    pub fn interconnect(&mut self, start_node_id: &u64, target_node_id: &u64) {
        let start_node = self.get_node(start_node_id);
        if start_node.is_none() {
            return;
        }
        start_node
            .unwrap()
            .borrow_mut()
            .add_edge_to(target_node_id.to_owned());
        let target_node = self.get_node(target_node_id);
        if target_node.is_none() {
            return;
        }
        target_node
            .unwrap()
            .borrow_mut()
            .add_edge_to(start_node_id.to_owned())
    }

    pub fn find_closest_start(&mut self, start: LatLonPos) {
        for (id, node) in self.nodes.iter() {
            if is_within_radius(
                start,
                (node.borrow().data.lat, node.borrow().data.lon),
                1000,
            ) {
                self.start_node = Some(id.to_owned());
                return;
            }
        }
    }

    pub fn find_closest_target(&mut self, target: LatLonPos) {
        for (id, node) in self.nodes.iter() {
            if is_within_radius(
                target,
                (node.borrow().data.lat, node.borrow().data.lon),
                1000,
            ) {
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
fn find_path(graph: &Graph) -> Result<Option<u64>, Box<dyn Error>> {
    let id = graph.start_node.unwrap();
    let start_node = graph.get_node(&id).unwrap();
    let mut open_set = vec![start_node.clone()];
    let mut closed_set: Vec<Rc<RefCell<GraphNode>>> = vec![];

    while !open_set.is_empty() {
        let lowest_ind = find_lowest_f_score_node_ind(&open_set);
        let removed_node = open_set.remove(lowest_ind);
        let current_node = removed_node.try_borrow()?;

        if current_node.data.id == graph.target_node.unwrap() {
            return Ok(Some(current_node.data.id));
        }
        closed_set.push(removed_node.clone());

        for neighbor_id in current_node.edges.iter() {
            let neighbor_node = graph.get_node(neighbor_id).unwrap().clone();
            let neighbor = neighbor_node.try_borrow()?;
            let mut neigbor_mut = neighbor_node.try_borrow_mut()?;

            if !set_includes(&closed_set, neighbor_id) {
                let current_distance =
                    current_node.distance_from_start + count_hypot(&current_node, &neighbor);
                if set_includes(&open_set, neighbor_id)
                    && neighbor.distance_from_start > current_distance
                {
                    neigbor_mut.distance_from_start = current_distance;
                } else {
                    neigbor_mut.distance_from_start = current_distance;
                    neigbor_mut.parent_id = Some(current_node.data.id);
                    open_set.push(neighbor_node.clone());
                }
                let end_node = graph
                    .get_node(&graph.target_node.unwrap())
                    .unwrap()
                    .borrow();
                let heuristics = count_heuristics(&current_node, &end_node);
                let f_score = neighbor.distance_from_start + heuristics;
                neigbor_mut.f_score = f_score;
            }
        }
    }
    Ok(None)
}

fn count_heuristics(node: &Ref<GraphNode>, end_node: &Ref<GraphNode>) -> f64 {
    (node.data.lat - end_node.data.lat).abs() + (node.data.lon - end_node.data.lon).abs()
}

fn count_hypot(node1: &Ref<GraphNode>, node2: &Ref<GraphNode>) -> f64 {
    ((node2.data.lat - node1.data.lat).powi(2) + (node2.data.lon - node1.data.lon).powi(2)).sqrt()
}

fn set_includes(set: &[Rc<RefCell<GraphNode>>], node_id: &u64) -> bool {
    let res = set.iter().find(|item| &item.borrow().data.id == node_id);
    res.is_some()
}

fn find_lowest_f_score_node_ind(set: &[Rc<RefCell<GraphNode>>]) -> usize {
    let mut lowest_ind: usize = 0;
    for (ind, node) in set.iter().enumerate() {
        if node.borrow().f_score < set[lowest_ind].borrow().f_score {
            lowest_ind = ind;
        }
    }
    lowest_ind
}

fn construct_path(end_node_id: u64, graph: Graph) -> Vec<Node> {
    let mut current: Option<u64> = Some(end_node_id);
    let mut path: Vec<Node> = vec![];

    while current.is_some() {
        let current_node = graph.get_node(&current.unwrap()).unwrap().borrow();
        path.push(Node {
            r#type: current_node.data.r#type.clone(),
            id: current_node.data.id,
            lat: current_node.data.lat,
            lon: current_node.data.lon,
        });
        if let Some(parent) = current_node.parent_id {
            current = Some(parent);
        } else {
            current = None;
        }
    }
    path.reverse();
    path
}

pub fn create_path(
    elements: Vec<ApiElements>,
    start_pos: LatLonPos,
    target_pos: LatLonPos,
) -> Result<Vec<Node>, String> {
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
                let graph_node = GraphNode::init(node);

                if start_pos.0 == graph_node.borrow().data.lat
                    && start_pos.1 == graph_node.borrow().data.lon
                {
                    graph.start_node = Some(graph_node.borrow().data.id)
                }
                if target_pos.0 == graph_node.borrow().data.lat
                    && target_pos.1 == graph_node.borrow().data.lon
                {
                    graph.target_node = Some(graph_node.borrow().data.id)
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
    let result = find_path(&graph);

    match result {
        Ok(val) => {
            if let Some(res) = val {
                let path = construct_path(res, graph);
                Ok(path)
            } else {
                Err(String::from("Couldn't find a path to a requested point"))
            }
        }
        _ => Err(String::from(
            "Something went wrong when calculating the path",
        )),
    }
}
