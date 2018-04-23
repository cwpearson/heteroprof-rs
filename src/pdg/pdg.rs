extern crate cuckoofilter;
extern crate interval;
extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;
use self::interval::interval_set::{IntervalSet, ToIntervalSet};
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
use std::rc::{Rc, Weak};


//Values as nodes and then computes and tranfsers as edges
/*
With cudaLaucn you do cudaConfigureCall and cudaSetupArguments
Do a memcpy and it is an allocation that I can't find, good idea to generate an allocation that is on the host, maybe a warning and print it to standardError. 

Graph get edges in constant time
*/
/*
What to do if a large value is then split into multiple small values, but a portion of it remains
untouched
*/

//Node within the graph will be a unique identifier for a value
//For now could just increment a counter
pub struct PDG {
    next_id: u64,
    value_map: HashMap<u64, Weak<value::Value>>,
    current_edge_number: u64,
    current_node_number: u64,
    pub graph: DiGraphMap<u64, u64>,
    pub computes: HashMap<u64, Compute>,
    pub transfers: HashMap<u64, Transfer>,
}

impl PDG {
    pub fn new() -> PDG {
        return PDG {
            next_id: 0,
            value_map: HashMap::new(),
            graph: DiGraphMap::new(),
            computes: HashMap::new(),
            transfers: HashMap::new(),
            current_edge_number: 0,
            current_node_number: 0,
        };
    }

    //
    pub fn add_compute(&mut self, c: &Compute) {

        
    }

    //Need src Value, dst Value, and Transfer
    pub fn add_transfer(&mut self, t: Transfer, src_ptr: Weak<value::Value>, dst_ptr: Weak<value::Value>) {
        // self.graph.add_edge(1, 2, 1);
        let key = {
            let node_itr = self.graph.nodes();
            let mut current_key = self.current_node_number;
            self.current_node_number += 1;

            //Look to see if we have seen this value before
            //We will often see this in the case of the src of 
            //of a transfer.
            for (key, value) in self.value_map.iter() {

                let strong_value: Option<Rc<_>> = value.upgrade();
                let strong_ptr: Option<Rc<_>> = src_ptr.upgrade();


                if strong_value.unwrap() == strong_ptr.unwrap() {
                    current_key = *key;
                }
            }
            current_key
        };
        
        self.graph.add_edge(key, self.current_node_number, self.current_edge_number);
        self.transfers.insert(self.current_edge_number, t);                    
        self.current_node_number += 1;
        self.current_edge_number += 1;

        // match node_itr {
        //     Some(node_itr) => {
                
        //     }
        //     None => {
        //         self.graph.add_edge(1, 3, self.current_edge_number);
        //         self.transfers.insert(self.current_edge_number, *t);
        //         self.current_edge_number += 1;
        //     }
        // }
    }
}

fn handle_cuda_malloc(cm: &callback::CudaMallocS, mut state: cuda::State) -> cuda::State {
    //rustc says there is an error here, however no issues
    let set = vec![(0, 0)].to_interval_set();

    let mut allocation = Rc::new(Allocation::new(0, cm.ptr, cm.size, AddressSpace::UVA));
    state.allocations.insert(Rc::clone(&allocation));

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
    let duration = cm.wall_end - cm.wall_start;

    //Identify what type of memory copy that it is
    let memcpy_kind = match cm.cuda_memcpy_kind {
        0 => String::from("cudaMemcpyHostToHost"),
        1 => String::from("cudaMemcpyHostToDevice"),
        2 => String::from("cudaMemcpyDeviceToHost"),
        3 => String::from("cudaMemcpyDeviceToDevice"),
        _ => panic!("Memcpy kind not recognized, this should NEVER happen"),
    };

    let (src_rc, dst_rc) = match cm.cuda_memcpy_kind {
        //Decide on what behaviour to exhibit depending on memcpy_kind
        // 0 => {
        //     //Will probably never be seeing this
        // }
        1 => {
            //Create a value on the host -- for now do nothing
            let src_rc = state.add_host_pointer(cm.src);
            //Create a value on the Device
            let dst_rc = state.update_allocations(cm.id, cm.dst, cm.count);

            (src_rc, dst_rc)
        }
        2 => {
            //Create a value on the host -- for now do nothing
            let dst_rc = state.add_host_pointer(cm.dst);
            //Update value on the gpu
            let src_rc = state.update_allocations(cm.id, cm.src, cm.count);

            (src_rc, dst_rc)
        }
        3 => {
            //Update values within allocations for both src and 
            //dst.
            let dst_rc = state.update_allocations(cm.id, cm.dst, cm.count);
            let src_rc = state.update_allocations(cm.id, cm.src, cm.count);

            (src_rc, dst_rc)
        }
        _ => {
            panic!("This should never happen, input file may be corrupted");
        }
    };

     let transfer = Transfer {
                correlation_id: cm.correlation_id,
                cuda_device_id: 50, //Need to get this from activity
                kind: memcpy_kind,
                start: cm.wall_start,
                dur: duration,
                stream_id: 1, //Need to get this from activity
    };
    graph.add_transfer(transfer, src_rc, dst_rc);


  
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
                println!("Handling memcpy!");
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
