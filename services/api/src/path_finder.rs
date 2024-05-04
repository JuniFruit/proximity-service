use std::collections;

pub struct Node {
    pub r#type: String,
    pub id: u64,
    pub lat: f64,
    pub lon: f64,
}

pub struct GraphNode {
    pub data: Node,
    pub edges: Vec<GraphNode>,
}

pub struct Graph {
    nodes: collections::HashMap<u64, GraphNode>,
}
