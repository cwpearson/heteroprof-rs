extern crate petgraph;

pub struct Edge {}

impl petgraph::EdgeType for Edge {
    fn is_directed() -> bool {
        true
    }
}

impl Edge {
    pub fn new() -> Edge {
        Edge {}
    }
}
