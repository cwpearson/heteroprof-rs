use std::hash::{Hash, Hasher};

use std::mem;

#[derive(PartialOrd, PartialEq, Clone, Copy)]
pub struct Node {
    start: f64,
}

struct Float {
    f: f64,
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits: u64 = unsafe { mem::transmute(&self) };
        bits.hash(state)
    }
}
