extern crate serde;
extern crate serde_json;

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub struct ValueS {
    id: usize,
}

impl ValueS {
    pub fn new(id: usize) -> ValueS {
        ValueS { id: id }
    }
}

#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum Node {
    Value(ValueS),
}
