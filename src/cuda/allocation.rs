extern crate gcollections;
extern crate interval;
extern crate serde;
extern crate serde_json;

use std::cmp::Ordering;
use self::interval::interval_set::{IntervalSet, ToIntervalSet};
use self::gcollections::ops::set::{Intersection, Union};
use self::gcollections::ops::cardinality::IsEmpty;

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
    pub space_occupied: IntervalSet<u64>,
}

impl Allocation {
    pub fn contains(&self, item: u64) -> bool {
        return (item >= self.pos) && (item < self.pos + self.size);
    }

    pub fn value_occupied(&self, ptr: u64, item_size: u64) {
        //This may need to be divided by four to be correct, I can't remember how pointers work
        let temp_set = vec![(ptr, ptr + item_size)].to_interval_set();
        let intersection = self.space_occupied.intersection(&temp_set);
        if intersection.is_empty() {
            //All good, create away
            self.space_occupied.union(&temp_set);
        } else {
            //Handle the intersection gracefully
        }
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
