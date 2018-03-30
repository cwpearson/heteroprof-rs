extern crate petgraph;

pub struct Edge {}

pub struct ComputeS {}

pub struct TransferS {}

impl petgraph::EdgeType for ComputeS {
    fn is_directed() -> bool {
        true
    }
}

impl petgraph::EdgeType for TransferS {
    fn is_directed() -> bool {
        true
    }
}

impl TransferS {
    pub fn new() -> TransferS {
        TransferS {}
    }
}

pub enum Edge2<'a> {
    Compute(&'a ComputeS),
    Transfer(&'a TransferS),
}

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
