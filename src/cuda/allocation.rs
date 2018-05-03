extern crate gcollections;
extern crate interval;
extern crate serde;
extern crate serde_json;

//Traits that we must implement or that we need
use cuda::allocation::gcollections::ops::Bounded;
use std::cmp::Ordering;

use self::gcollections::ops::cardinality::IsEmpty;
use self::gcollections::ops::set::{Intersection, Union};
use self::interval::interval_set::{IntervalSet, ToIntervalSet};

use cuda::value::Value;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::vec::Vec;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AddressSpace {
    UVA,
    HOST,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Allocation {
    pub id: u64,
    pub pos: u64,
    pub size: u64,
    pub address_space: AddressSpace,
    pub space_occupied: IntervalSet<u64>,
    pub values: HashMap<u64, Rc<Value>>,
    pub old_values: Vec<Rc<Value>>,
}

impl Allocation {
    pub fn new(temp_id: u64, temp_pos: u64, temp_size: u64, temp_addr: AddressSpace) -> Allocation {
        Allocation {
            id: temp_id,
            pos: temp_pos,
            size: temp_size,
            address_space: temp_addr,
            space_occupied: vec![(0, 0)].to_interval_set(),
            values: HashMap::new(),
            old_values: vec![Rc::new(Value {
                id: 0,
                ptr: 0,
                size: 0,
                times_modified: 0,
            })],
        }
    }

    pub fn contains(&self, item: u64) -> bool {
        return (item >= self.pos) && (item < self.pos + self.size);
    }

    pub fn value_occupied(&mut self, id: u64, ptr: u64, item_size: u64) -> Weak<Value> {
        //This may need to be divided by four to be correct, I can't remember how pointers work
        let temp_set = vec![(ptr, ptr + item_size)].to_interval_set();
        let intersection = self.space_occupied.intersection(&temp_set);
        if intersection.is_empty() {
            println!("No intersection, {}", ptr);

            //All good, create away
            self.space_occupied.union(&temp_set);
            let temp_val = Value {
                id: id,
                ptr: ptr,
                size: item_size,
                times_modified: 0,
            };
            let temp_val_rc = Rc::from(temp_val);
            let downgraded = Rc::downgrade(&temp_val_rc);
            self.values.insert(ptr, temp_val_rc);
            downgraded
        } else {
            //Handle the intersection gracefully
            //What we are doing right now is just creating a value on top of the
            //the other value.
            println!("Intersection, {}", ptr);

            let mut highest_modified = {
                let mut highest_seen = 0;

                for x in intersection.lower()..intersection.upper() {
                    match self.values.get(&x) {
                        Some(v) => {
                            if v.times_modified > highest_seen {
                                highest_seen = v.times_modified;
                            }
                        }
                        _ => {
                            //do nothing
                        }
                    }
                }

                highest_seen
            };
            highest_modified = highest_modified + 1;

            self.space_occupied.union(&temp_set);
            let temp_val = Value {
                id: id,
                ptr: ptr,
                size: item_size,
                times_modified: highest_modified, //Need to come up with some pattern matching for this
            };
            let temp_val_rc = Rc::from(temp_val);
            // println!("Strong count: {}", Rc::strong_count(&temp_val_rc));

            let downgraded = Rc::downgrade(&temp_val_rc);
            self.values.insert(ptr, temp_val_rc);
            downgraded
        }
    }

    pub fn compute_value(&mut self, ptr: u64) -> Option<(Weak<Value>, Weak<Value>)> {
        let value = self.values.remove(&ptr);

        match value {
            Some(v) => {
                // println!("Strong count: {}", Rc::strong_count(&v));
                let mut value_unwrapped = Rc::try_unwrap(v).unwrap();
                let original = value_unwrapped.clone();

                // let mut value_unwrapped = Rc::try_unwrap(*rc_value).unwrap();
                value_unwrapped.increment();
                let original_rc = Rc::new(original);
                let updated_rc = Rc::new(value_unwrapped);

                let downgraded = Rc::downgrade(&updated_rc);
                let downgraded_original = Rc::downgrade(&original_rc);
                self.values.insert(ptr, updated_rc);
                self.old_values.push(original_rc);
                Some((downgraded_original, downgraded))
            }
            _ => {
                None
                // //Hue
                // let temp_val = Value {
                //     id: id,
                //     ptr: ptr,
                //     size: item_size,
                //     times_modified: highest_modified, //Need to come up with some pattern matching for this
                // };
                // let temp_val_rc = Rc::from(temp_val);
                // println!("Strong count: {}", Rc::strong_count(&temp_val_rc));

                // let downgraded = Rc::downgrade(&temp_val_rc);
                // self.values.insert(ptr, temp_val_rc);
                // Some((downgraded_original, downgraded))
            }
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
