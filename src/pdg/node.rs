extern crate serde;
extern crate serde_json;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct TransferS {
    id: usize,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ComputeS {
    id: usize,
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ValueS {
    id: usize,
}

impl TransferS {
    pub fn new(id: usize) -> TransferS {
        TransferS { id: id }
    }
}

impl ComputeS {
    pub fn new(id: usize) -> ComputeS {
        ComputeS { id: id }
    }
}

impl ValueS {
    pub fn new(id: usize) -> ValueS {
        ValueS { id: id }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Node {
    Compute(ComputeS),
    Transfer(TransferS),
    Value(ValueS),
}
