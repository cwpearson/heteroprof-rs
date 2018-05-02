extern crate interval;

pub mod allocation;
pub mod configured_call;
pub mod dim3;
pub mod value;

use self::interval::interval_set::{IntervalSet, ToIntervalSet};
use cuda::allocation::{AddressSpace, Allocation};
use cuda::configured_call::ConfiguredCall;
use cuda::value::Value;
use std::collections::{BTreeSet, HashMap};
use std::ops::Range;
use std::ops::{Index, IndexMut};
use std::option::Option;
use std::rc::{Rc, Weak};

pub struct Thread {
    pub current_device: u64,
    pub configured_call: ConfiguredCall,
}

impl Thread {
    pub fn new() -> Thread {
        Thread {
            current_device: 0,
            configured_call: ConfiguredCall::new(),
        }
    }
}

pub struct State {
    pub threads: HashMap<u64, Thread>,
    pub allocations: BTreeSet<Rc<Allocation>>,
    pub host_pointers: BTreeSet<u64>,
    pub host_value: Rc<Value>,
    // pub values: BTreeSet<Rc<Value>>,
}

impl State {
    pub fn new() -> State {
        State {
            threads: HashMap::new(),
            allocations: BTreeSet::new(),
            host_pointers: BTreeSet::new(),
            host_value: Rc::new(Value {
                id: 0,
                ptr: 0,
                size: 0,
                times_modified: 0,
            }),
        }
    }

    pub fn add_host_pointer(&mut self, ptr: u64) -> Option<Weak<Value>> {
        self.host_pointers.insert(ptr);
        Some(Rc::downgrade(&self.host_value))
    }

    pub fn update_allocations(
        &mut self,
        id: u64,
        allocation_start: u64,
        allocation_size: u64,
    ) -> Option<Weak<Value>> {
        let key = {
            let mut iter = self.allocations.iter();

            let mut current_key = match iter.find(|&a| a.contains(allocation_start)) {
                Some(v) => Some(v),
                _ => {
                    // println!("Allocation not found!");
                    None
                }
            };

            let clone = match current_key {
                Some(v) => Some(Rc::clone(v)),
                _ => None,
            };
            clone
        };

        let return_val = match key {
            Some(v) => {
                let rc_pointer = self.allocations.take(&v).unwrap();

                //Drop the value so there is only one strong reference pointer to key.
                //Otherwise we cannot modify the value
                drop(v);

                let mut alloc = Rc::try_unwrap(rc_pointer).unwrap();
                let value_rc = alloc.value_occupied(id, allocation_start, allocation_size);
                let alloc_insert = Rc::new(alloc);
                self.allocations.insert(alloc_insert);
                Some(value_rc)
            }
            _ => None,
        };
        return_val
    }

    pub fn setup_arg_update_allocations(
        &mut self,
        id: u64,
        allocation_start: u64,
        allocation_size: u64,
    ) -> Option<Weak<Value>> {
        let key = {
            let mut iter = self.allocations.iter();

            let mut current_key = match iter.find(|&a| a.contains(allocation_start)) {
                Some(v) => Some(v),
                _ => {
                    // println!("Allocation not found!");
                    None
                }
            };

            let clone = match current_key {
                Some(v) => Some(Rc::clone(v)),
                _ => None,
            };
            clone
        };

        let return_val = match key {
            Some(v) => {
                let rc_pointer = self.allocations.take(&v).unwrap();

                //Drop the value so there is only one strong reference pointer to key.
                //Otherwise we cannot modify the value
                drop(v);

                let mut alloc = Rc::try_unwrap(rc_pointer).unwrap();
                let value_rc = alloc.value_occupied(id, allocation_start, allocation_size);
                let alloc_insert = Rc::new(alloc);
                self.allocations.insert(alloc_insert);
                Some(value_rc)
            }
            _ => None,
        };
        return_val
    }

    pub fn find_argument_values(&mut self, ptr: u64) -> Option<(Weak<Value>, Weak<Value>)> {
        let key = {
            let mut iter = self.allocations.iter();
            let mut current_key = match iter.find(|&a| a.contains(ptr)) {
                Some(v) => {
                    // println!("Found allocation for argument {}", ptr);
                    Some(v)
                }
                _ => {
                    // println!("Argument does not have corresponding allocation, {}", ptr);
                    return None;
                }
            };
            Rc::clone(current_key.unwrap())
        };

        let allocation = self.allocations.take(&key).unwrap();

        drop(key);
        let mut alloc = Rc::try_unwrap(allocation).unwrap();
        // let mut cloned_value = alloc.clone();
        let y = alloc.compute_value(ptr);
        let alloc_insert = Rc::new(alloc);
        self.allocations.insert(alloc_insert);
        y
        // println!("112");
        // Some((downgraded_val, new_val))
    }
}

impl Index<u64> for State {
    type Output = Thread;

    fn index(&self, index: u64) -> &Thread {
        &self.threads[&index]
    }
}

impl IndexMut<u64> for State {
    fn index_mut(&mut self, index: u64) -> &mut Thread {
        self.threads.entry(index).or_insert(Thread::new())
    }
}
