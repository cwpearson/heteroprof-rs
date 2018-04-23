extern crate interval;

pub mod allocation;
pub mod value;
pub mod dim3;
pub mod configured_call;

use self::interval::interval_set::{IntervalSet, ToIntervalSet};
use std::collections::{BTreeSet, HashMap};
use std::ops::{Index, IndexMut};
use std::ops::Range;
use cuda::allocation::{AddressSpace, Allocation};
use cuda::value::Value;
use cuda::configured_call::ConfiguredCall;
use std::rc::{Rc, Weak};
use std::option::Option;

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

    pub fn add_host_pointer(&mut self, ptr: u64) -> Weak<Value> {
        self.host_pointers.insert(ptr);
        Rc::downgrade(&self.host_value)
    }

    pub fn update_allocations(&mut self, id: u64, allocation_start: u64, allocation_size: u64) -> Weak<Value> {
     
        let mut key = {
            let mut iter = self.allocations.iter();

            let mut current_key = match iter.find(|&a| a.contains(allocation_start)) {
                Some(v) => {
                    Some(v)
                },
                _ => {
                    println!("Allocation not found!");
                    None
                }
            };
            Rc::clone(current_key.unwrap())
        };
        let rc_pointer = self.allocations.take(&key).unwrap();

        //Drop the value so there is only one strong reference pointer to key.
        //Otherwise we cannot modify the value
        drop(key);

        let mut alloc = Rc::try_unwrap(rc_pointer).unwrap();
        let value_rc = alloc.value_occupied(id, allocation_start, allocation_size);
        let alloc_insert = Rc::new(alloc);
        self.allocations.insert(alloc_insert);
        value_rc
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
