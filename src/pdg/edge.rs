extern crate petgraph;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct TransferS {
    id: usize,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ComputeS {
    id: usize,
}

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
        TransferS { id: 0 }
    }
}

impl ComputeS {
    pub fn new() -> ComputeS {
        ComputeS { id: 0 }
    }
}

pub enum Edge<'a> {
    Compute(&'a ComputeS),
    Transfer(&'a TransferS),
}
