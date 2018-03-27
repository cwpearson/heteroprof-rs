extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;
use pdg::edge::Edge;
use pdg::compute::Compute;
use Document;
use pdg::transfer::Transfer;
use std::collections::HashMap;
use callback;
use cuda;
use cuda::allocation::{AddressSpace, Allocation};

pub struct PDG {
    next_id: u64,
    pub edges: DiGraphMap<u64, Edge>,
    pub computes: HashMap<u64, Compute>,
    pub transfers: HashMap<u64, Transfer>,
}

impl PDG {
    pub fn new() -> PDG {
        return PDG {
            next_id: 0,
            edges: DiGraphMap::new(),
            computes: HashMap::new(),
            transfers: HashMap::new(),
        };
    }

    pub fn add_compute(&mut self, c: &Compute) {}
    pub fn add_transfer(&mut self, t: &Transfer) {}
}

fn handle_cuda_malloc(cm: &callback::CudaMallocS, mut state: cuda::State) -> cuda::State {
    let allocation = Allocation {
        id: 0,
        pos: cm.ptr,
        size: cm.size,
        address_space: AddressSpace::UVA,
    };
    state.allocations.insert(allocation);
    state
}

fn handle_cuda_configure_call(
    cc: &callback::CudaConfigureCallS,
    mut state: cuda::State,
) -> cuda::State {
    let tid = cc.calling_tid;
    state[tid].configured_call.start();
    state
}

fn handle_cuda_setup_argument(
    csa: &callback::CudaSetupArgumentS,
    mut state: cuda::State,
) -> cuda::State {
    state[csa.calling_tid].configured_call.add_arg(csa.arg);
    state
}

fn handle_cuda_memcpy(cm: &callback::CudaMemcpyS, mut state: cuda::State) -> cuda::State {
    // find the src allocation
    let src_pos = cm.src;
    let src_size = cm.count;
    let dst_pos = cm.dst;
    {
        let mut iter = state.allocations.iter();

        let src_alloc = match iter.find(|&a| a.contains(src_pos)) {
            Some(alloc) => alloc,
            _ => panic!("ahh"),
        };

        // src_alloc.get(src_pos, src_size) // get a Value at src_pos, src_size
    }

    // find the dst allocation
    state
}
pub fn from_document(doc: &Document) -> PDG {
    let mut state = cuda::State::new();
    let pdg = PDG::new();

    // First pass: set up allocations
    for api in doc.apis() {
        use callback::Record::*;
        match api {
            &CudaMalloc(ref m) => {
                state = handle_cuda_malloc(m, state);
                println!("{} allocations", state.allocations.len());
            }
            _ => (),
        }
    }

    // Second pass: set up values from kernels / memcopies
    for api in doc.apis() {
        use callback::Record::*;
        match api {
            &CudaConfigureCall(ref cc) => {
                state = handle_cuda_configure_call(cc, state);
            }
            &CudaSetupArgument(ref sa) => {
                state = handle_cuda_setup_argument(sa, state);
            }
            &CudaMemcpy(ref m) => {
                state = handle_cuda_memcpy(m, state);
            }
            _ => (),
        }
    }

    return pdg;
}
