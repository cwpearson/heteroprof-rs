pub mod allocation;
pub mod value;
pub mod dim3;
pub mod configured_call;

use std::collections::{BTreeSet, HashMap};
use std::ops::{Index, IndexMut};
use std::ops::Range;
use cuda::allocation::Allocation;
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
}

impl State {
    pub fn new() -> State {
        State {
            threads: HashMap::new(),
            allocations: BTreeSet::new(),
        }
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
