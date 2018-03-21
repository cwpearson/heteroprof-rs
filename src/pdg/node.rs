use std::hash::{Hash, Hasher};

use std::mem;

#[derive(Clone, Copy)]
struct Float {
    f: f64,
}

#[derive(Hash, PartialOrd, PartialEq, Clone, Copy)]
pub struct Node {
    start: Float,
}

impl Node {
    pub fn as_f64(&self) -> f64 {
        return self.start;
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits: u64 = unsafe { mem::transmute(&self) };
        bits.hash(state)
    }
}
