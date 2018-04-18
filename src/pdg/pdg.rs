extern crate cuckoofilter;
extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;
use pdg::edge::Edge;
use pdg::compute::Compute;
use pdg::transfer::Transfer;
use std::collections::HashMap;
use callback;
use activity;
use cuda;
use cuda::allocation::{AddressSpace, Allocation};
use cuda::value;
pub use document::Document;
use std::rc::Rc;

//Values as nodes and then computes and tranfsers as edges
/*
With cudaLaucn you do cudaConfigureCall and cudaSetupArguments
Do a memcpy and it is an allocation that I can't find, good idea to generate an allocation that is on the host, maybe a warning and print it to standardError. 

Graph get edges in constant time
*/

thread_local!(static filter_set: cuckoofilter = cuckoofilter::new());

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
    let allocation = Rc::new(Allocation {
        id: 0,
        pos: cm.ptr,
        size: cm.size,
        address_space: AddressSpace::UVA,
    });
    state.allocations.insert(Rc::clone(&allocation));

    //An entire allocation may not be completely dedicated to one Value.
    //This is not correct!
    // let val_result = value::val_from_malloc(cm, &allocation);
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

fn handle_cuda_memcpy(
    graph: &mut PDG,
    cm: &callback::CudaMemcpyS,
    mut state: cuda::State,
) -> cuda::State {
    // find the src allocation
    let src_pos = cm.src;
    let src_size = cm.count;
    let dst_pos = cm.dst;

    //Identify what type of memory copy that it is
    let memcpy_kind = match cm.cuda_memcpy_kind {
        0 => String::from("cudaMemcpyHostToHost"),
        1 => String::from("cudaMemcpyHostToDevice"),
        2 => String::from("cudaMemcpyDeviceToHost"),
        3 => String::from("cudaMemcpyDeviceToDevice"),
        _ => panic!("Memcpy kind not recognized, this should NEVER happen"),
    };

    let src_success = filter_set.contains(src_pos);
    let dst_success = filter_set.contains(dst_pos);

    if (!src_success) {
        //Create a new value for the source, as it has not been seen before
        match cm.cuda_memcpy_kind {
            //Decide on what behaviour to exhibit depending on memcpy_kind
            0 => {
                //Will probably never be seeing this
            }
            1 => {
                //Create a value on the host -- for now do nothing
            }
            2 => {
                //This should also be rare
            }
            3 => {
                //Fill in later
            }
        }
    } else {
        let mut iter = state.allocations.iter();

        let src_alloc = match iter.find(|&a| a.contains(src_pos)) {
            Some(alloc) => {
                println!("Src found!");
                Some(alloc)
            }
            _ => {
                println!("Src not found!");
                None
            }
        };
    }

    if (!dst_success) {
        //Create a new value for the destination
        match cm.cuda_memcpy_kind {
            //Decide on what behaviour to exhibit depending on memcpy_kind
            0 => {
                //Will probably never be seeing this
            }
            1 => {
                //Create a value on the Device
            }
            2 => {
                //Create a value on the host -- for now do nothing
            }
            3 => {
                //Fill in later
            }
        }
    } else {
        let mut iter = state.allocations.iter();

        let dst_alloc = match iter.find(|&a| a.contains(dst_pos)) {
            Some(alloc) => {
                println!("Dst found!");
                Some(alloc)
            }
            _ => {
                println!("Dst not found!");
                None
            }
        };
    }

    //This should theoretically be switched to values
    {}

    {}

    let duration = cm.wall_end - cm.wall_start;

    //We cannot get all information from callback, need to combine it with activity
    let transfer = &Transfer {
        correlation_id: cm.correlation_id,
        cuda_device_id: 50,
        kind: memcpy_kind,
        start: cm.wall_start,
        dur: duration,
        stream_id: 1,
    };
    graph.add_transfer(transfer);
    // find the dst allocation
    state
}

fn handle_memcpy_activity(
    graph: &mut PDG,
    cm: &activity::MemcpyS,
    mut state: cuda::State,
) -> cuda::State {
    // let src_pos = cm.src;
    // let src_size = cm.count;
    // let dst_pos = cm.dst;
    // {
    //     let mut iter = state.allocations.iter();

    //     let src_alloc = match iter.find(|&a| a.contains(src_pos)) {
    //         Some(alloc) => alloc,
    //         _ => panic!("ahh"),
    //     };
    // }

    // {
    //     let mut iter = state.allocations.iter();

    //     let dst_alloc = match iter.find(|&a| a.contains(dst_pos)) {
    //         Some(alloc) => alloc,
    //         _ => panic!("ahhh"),
    //     };
    // }
    state
}

/*
    This is where we want to build the dependence graph for allocations etc.
*/
pub fn from_document(doc: &Document) -> PDG {
    let mut state = cuda::State::new();
    let mut pdg = PDG::new();

    // Do first pass through all APIs
    for api in doc.apis() {
        use callback::Record::*;
        match api {
            &CudaMalloc(ref m) => {
                state = handle_cuda_malloc(m, state);
                println!("{} allocations", state.allocations.len());
            }
            &CudaConfigureCall(ref cc) => {
                state = handle_cuda_configure_call(cc, state);
            }
            &CudaSetupArgument(ref sa) => {
                state = handle_cuda_setup_argument(sa, state);
            }
            &CudaMemcpy(ref m) => {
                state = handle_cuda_memcpy(&mut pdg, m, state);
            }
            _ => (),
        }
    }

    for activity in doc.activities() {
        use activity::Record::*;
        match activity {
            &Kernel3(ref m) => {
                //Handle kernel launch activity
            }
            &Memcpy(ref m) => {
                //Handle memcpy activity
                state = handle_memcpy_activity(&mut pdg, m, state);
            }
            _ => panic!("Unexpected activity encountered!"),
        }
    }

    for cudnn_call in doc.cudnn_calls() {
        use cudnn::Record::*;
        match cudnn_call {
            _ => panic!("Unexpected cudnn activity encountered!"),
        }
    }

    return pdg;
}
