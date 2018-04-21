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
use std::rc::Rc;
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
    // pub values: BTreeSet<Rc<Value>>,
}

impl State {
    pub fn new() -> State {
        State {
            threads: HashMap::new(),
            allocations: BTreeSet::new(),
        }
    }

    pub fn update_allocations(&mut self, id: u64, allocation_start: u64, allocation_size: u64) {
        let mut key = {
            let mut iter = self.allocations.iter();

            let mut current_key = match iter.find(|&a| a.contains(allocation_start)) {
                Some(v) => Some(v),
                _ => {
                    println!("Allocation not found!");
                    None
                    // None
                }
            };

            current_key
                .unwrap()
                
                /*
                If we want to handle there not being an allocation
                    //The or else should basically never happen.
            //Means we missed something in a CudaMalloc
                (|| {
                    &Rc::new(Allocation {
                        id: id,
                        pos: allocation_start,
                        size: allocation_size,
                        address_space: AddressSpace::UVA,
                        space_occupied: vec![(0, 0)].to_interval_set(),
                        values: HashMap::new(),
                    })
                })*/
                .clone()
        };

        let mut alloc = Rc::try_unwrap(self.allocations.take(&key).unwrap()).unwrap();
        alloc.value_occupied(id, allocation_start, allocation_size);
        let alloc_insert = Rc::new(alloc);
        self.allocations.insert(alloc_insert);
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
