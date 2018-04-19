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

    pub fn update_allocations(&mut self, allocation_start: u64, allocation_size: u64) {
        let mut iter = self.allocations.iter();

        //let mut alloc =
        match iter.find(|&a| a.contains(allocation_start)) {
            Some(v) => {
                println!("Allocation found!");
                v.value_occupied(allocation_start, allocation_size);
                // Some(v)
            }
            _ => {
                println!("Allocation not found!");
                // None
            }
        } //;

        // let mut alloc_mut = alloc.as_mut();
        // let mut hi = match alloc_mut {
        //     Some(v) => {
        //         v = v.get_mut();
        //         let mut z = Rc::get_mut(&mut v);
        //         if let Some(z) = z {
        //             z.value_occupied(allocation_start, allocation_size);
        //         }
        //     }
        //     None => {
        //         //Do nothing
        //     }
        // }
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
