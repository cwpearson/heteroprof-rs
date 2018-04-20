extern crate gcollections;
extern crate interval;
extern crate serde;
extern crate serde_json;

//Traits that we must implement
use std::cmp::{Eq, Ordering, PartialEq};
use std::fmt::Debug;

use self::interval::interval_set::{IntervalSet, ToIntervalSet};
use self::gcollections::ops::set::{Intersection, Union};
use self::gcollections::ops::cardinality::IsEmpty;
use std::collections::HashMap;
use cuda::value::Value;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AddressSpace {
    UVA,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Allocation {
    pub id: u64,
    pub pos: u64,
    pub size: u64,
    pub address_space: AddressSpace,
    pub space_occupied: IntervalSet<u64>,
    pub values: HashMap<(u64, u64), Rc<Value>>,
}

impl Allocation {
    pub fn contains(&self, item: u64) -> bool {
        return (item >= self.pos) && (item < self.pos + self.size);
    }

    pub fn value_occupied(&mut self, id: u64, ptr: u64, item_size: u64) {
        //This may need to be divided by four to be correct, I can't remember how pointers work
        let temp_set = vec![(ptr, ptr + item_size)].to_interval_set();
        let intersection = self.space_occupied.intersection(&temp_set);
        if intersection.is_empty() {
            //All good, create away
            self.space_occupied.union(&temp_set);
            let temp_val = Value {
                id: id,
                ptr: ptr,
                size: item_size,
                times_modified: 0,
            };
            self.values.insert((ptr, item_size), Rc::from(temp_val));
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

// impl Borrow for Allocation {}

// impl Eq for Rc<Value> {}
//
