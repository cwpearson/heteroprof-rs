pub mod allocation;
pub mod value;
pub mod dim3;
pub mod configured_call;

use std::collections::{BTreeSet, HashMap};
use std::ops::{Index, IndexMut};
use std::ops::Range;
use cuda::allocation::Allocation;
use cuda::value::Value;
use cuda::configured_call::ConfiguredCall;
use std::rc::Rc;

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
        let mut key;

        {
            //Want to clone the iterator so we do not have an immutable
            //borrow vs mutable borrow error
            let mut iter = self.allocations.iter().cloned();

            key = match iter.find(|ref a| a.contains(allocation_start)) {
                Some(v) => {
                    // let mut allocation = self.allocations.get(v);
                    // let mut allocation_mut = allocation.as_mut();
                    // let mut allocation_run = allocation.unwrap();
                    // v.value_occupied(id, allocation_start, allocation_size);
                    Some(v)
                }
                _ => {
                    println!("Allocation not found!");
                    None
                    // None
                }
            };
        }

        if let Some(key) = key {
            let mut alloc;
            {
                alloc = self.allocations.get(&key).clone();
            };

            {
                self.allocations.remove(&key);

                let mut alloc_unwrap = alloc.as_mut().unwrap();
                let mut alloc_mut = Rc::make_mut(alloc_unwrap);
                alloc_mut.value_occupied(id, allocation_start, allocation_size);
                let new_alloc_immut = alloc_mut;
                let alloc_rc = Rc::new(*new_alloc_immut);
                self.allocations.insert(alloc_rc);
            }

            // .value_occupied(id, allocation_start, allocation_size);
            // if let Some(alloc) = alloc {
            // alloc.value_occupied(id, allocation_start, allocation_size);
            // };
        };
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
