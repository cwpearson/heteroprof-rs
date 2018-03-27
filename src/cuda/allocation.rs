extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;

#[derive(PartialEq, Eq)]
pub enum AddressSpace {
    UVA,
}

#[derive(PartialEq, Eq)]
pub struct Allocation {
    pub id: u64,
    pub pos: u64,
    pub size: u64,
    pub address_space: AddressSpace,
}

impl Allocation {
    pub fn contains(&self, item: u64) -> bool {
        return (item >= self.pos) && (item < self.pos + self.size);
    }
}

impl Ord for Allocation {
    fn cmp(&self, other: &Allocation) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Allocation {
    fn partial_cmp(&self, other: &Allocation) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
