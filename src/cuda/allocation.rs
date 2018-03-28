use std::cmp::Ordering;
use cuda::value::{Value, Values};

#[derive(PartialEq, Eq)]
pub enum AddressSpace {
    UVA,
}

#[derive(PartialEq, Eq)]
pub struct Allocation {
    pub pos: u64,
    pub size: u64,
    pub address_space: AddressSpace,
    values: Values,
}

impl Allocation {
    pub fn new(pos: u64, size: u64, address_space: AddressSpace) -> Allocation {
        let a = Allocation {
            pos: pos,
            size: size,
            address_space: address_space,
            values: Values::new(pos, size),
        };
        a
    }

    pub fn contains(&self, item: u64) -> bool {
        return (item >= self.pos) && (item < self.pos + self.size);
    }

    pub fn get_value(&self, pos: u64, size: u64) -> &Value {
        return self.values.get(pos, size);
    }

    pub fn new_value(&mut self, pos: u64, size: u64) {
        self.values.push(Value::new(pos, size))
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
