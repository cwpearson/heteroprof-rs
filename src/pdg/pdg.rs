extern crate cuckoofilter;
extern crate interval;
extern crate petgraph;
extern crate priority_queue;

use self::interval::interval_set::{IntervalSet, ToIntervalSet};
use self::petgraph::algo::{astar, bellman_ford, dijkstra};
use self::petgraph::graph::{Graph, NodeIndex};
use self::petgraph::graphmap::DiGraphMap;
use self::petgraph::Directed;
use self::petgraph::Direction;
use self::priority_queue::PriorityQueue;

use activity;
use callback;
use cuda;
use cuda::allocation::{AddressSpace, Allocation};
use cuda::value;
pub use document::Document;
use pdg::compute::Compute;
use pdg::edge::Edge;
use pdg::transfer::Transfer;
use std::cmp::Ordering;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::i64::MAX;
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
    value_map: HashMap<u64, NodeIndex<u32>>,
    current_edge_number: u64,
    current_node_number: u64,
    pub graph: Graph<&u64, f64>,
    pub computes: HashMap<u64, Compute>,
    pub transfers: HashMap<u64, Transfer>,
}

impl PDG {
    pub fn new() -> PDG {
        return PDG {
            next_id: 0,
            value_map: HashMap::<u64, NodeIndex<u32>>::new(),
            graph: Graph::<u64, f64>::new(),
            computes: HashMap::new(),
            transfers: HashMap::new(),
            current_edge_number: 0,
            current_node_number: 0,
        };
    }

    //
    pub fn add_compute(
        &mut self,
        c: Compute,
        src_ptr: Weak<value::Value>,
        dst_ptr: Weak<value::Value>,
    ) {
        // println!("Adding compute!");
        let strong_ptr: Option<Rc<_>> = src_ptr.upgrade();
        let src_ptr_unwrap = strong_ptr.unwrap().ptr;

        let dst_ptr: Option<Rc<_>> = dst_ptr.upgrade();
        let dst_ptr_unwrap = dst_ptr.unwrap().ptr;

        // let src_ptr_unwrap = Rc::try_unwrap(strong_ptr.unwrap()).unwrap().ptr;

        // self.graph.add_edge(1, 2, 1);
        let key = {
            let node_itr = self.graph.raw_nodes().iter();
            let mut current_key;
            self.current_node_number += 1;
            for (key, value) in self.value_map.iter() {
                if *key == src_ptr_unwrap {
                    // println!("They're equal!!!");
                    current_key = *value;
                    break;
                }
            }

            // let strong_ptr: Option<Rc<_>> = src_ptr.upgrade();

            current_key
        };
        //Value map should contain ptr as key, and current node number as value
        self.value_map.insert(src_ptr_unwrap, key);

        let src_node = key; //self.graph.add_node(key);
        let dst_node: NodeIndex<u32> = self.graph.add_node(&self.current_node_number);

        self.graph.add_edge(src_node, dst_node, 1 as f64);
        self.value_map.insert(dst_ptr_unwrap, dst_node);

        self.computes.insert(self.current_edge_number, c);
        self.current_node_number += 1;
        self.current_edge_number += 1;
    }

    //Need src Value, dst Value, and Transfer
    pub fn add_transfer(
        &mut self,
        t: Transfer,
        src_ptr: Weak<value::Value>,
        dst_ptr: Weak<value::Value>,
    ) {
        let strong_ptr: Option<Rc<_>> = src_ptr.upgrade();
        let src_ptr_unwrap = strong_ptr.unwrap().ptr;
        let dst_ptr: Option<Rc<_>> = dst_ptr.upgrade();
        let dst_ptr_unwrap = dst_ptr.unwrap().ptr;

        let key = {
            // let node_itr = self.graph.nodes();
            let mut current_key;
            // self.current_node_number += 1;

            //Look to see if we have seen this value before
            //We will often see this in the case of the src of
            //of a transfer.
            for (key, value) in self.value_map.iter() {
                if *key == src_ptr_unwrap {
                    current_key = *value;
                    break;
                }
            }

            // let strong_ptr: Option<Rc<_>> = src_ptr.upgrade();

            current_key
        };

        self.current_node_number += 1;
        self.current_edge_number += 1;
        let dst_node = self.graph.add_node(&self.current_node_number);
        self.graph.add_edge(key, dst_node, 1);
        self.value_map.insert(dst_ptr_unwrap, dst_node);

        self.transfers.insert(self.current_edge_number, t);
        self.current_node_number += 1;
        self.current_edge_number += 1;
    }

    fn longest_path(&mut self, start_node: NodeIndex, sinks: &Vec<NodeIndex>) -> u64 {
        let values = dijkstra(self.graph, start_node, Some(sinks[0]), edge_cost);

        0
    }

    pub fn num_nodes(&mut self) -> usize {
        self.graph.node_count()
    }

    pub fn num_edges(&mut self) -> usize {
        self.graph.edge_count()
    }

    pub fn find_longest_path(&mut self) -> u64 {
        let mut sources = vec![];
        let mut sinks = vec![];
        let mut longest_path = vec![];

        //Must scope immutable self so that it doesn't freak out with mutable reference
        {
            let mut _nodes = self.graph.node_indices();

            for node in _nodes {
                let x = node;
                let incoming = self.graph.neighbors_directed(node, Direction::Incoming);
                if incoming.count() == 0 {
                    sources.push(node);
                }

                let outgoing = self.graph.neighbors_directed(node, Direction::Outgoing);
                let x = outgoing.count();
                // println!("Node number is: {}", node);
                // println!("Outgoing count is: {}", x);
                if x == 0 {
                    sinks.push(node);
                }
            }
        }

        //Now look for the longest path using
        //Dijkstra
        println!("Found sinks and sources");
        for source in sources {
            let path_length = self.longest_path(source, &sinks);
            println!("Path lenght {}", path_length);
            longest_path.push(path_length);
        }

        // let source = sources[1];
        // let path_length = { self.longest_path(source, &sinks) };
        // longest_path.push(path_length);

        // *longest_path.iter().max().unwrap()

        0
    }
}

fn edge_cost(cm: u64) -> u64 {
    return 1;
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

fn handle_cuda_launch(
    graph: &mut PDG,
    csa: &callback::CudaLaunchS,
    mut state: cuda::State,
) -> cuda::State {
    for arg in state[csa.calling_tid].configured_call.args.clone() {
        let val_option = state.find_argument_values(arg);

        match val_option {
            Some(s) => {
                // println!("Arg: {}", arg);
                let (original_val, new_val) = s;
                let temp_duration = csa.wall_end - csa.wall_start;
                let comp = Compute {
                    completed: 1.0,
                    correlation_id: csa.correlation_id,
                    cuda_device_id: 1,
                    duration: temp_duration,
                    kind: String::from("fix"),
                    name: csa.symbol_name.clone(),
                    start: csa.wall_start,
                    stream_id: 1,
                    //Fill in
                };
                graph.add_compute(comp, original_val, new_val);
            }
            _ => {
                // println!("Don't do anything {}", arg);
            }
        }
    }
    state[csa.calling_tid].configured_call.finish();
    state
}

fn handle_cuda_setup_argument(
    graph: &mut PDG,
    csa: &callback::CudaSetupArgumentS,
    mut state: cuda::State,
) -> cuda::State {
    state[csa.calling_tid].configured_call.add_arg(csa.arg);

    //Count does not matter here
    state.setup_arg_update_allocations(csa.id, csa.arg, 1);

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

    match (src_rc, dst_rc) {
        (Some(v), Some(y)) => {
            let transfer = Transfer {
                correlation_id: cm.correlation_id,
                cuda_device_id: 50, //Need to get this from activity
                kind: memcpy_kind,
                start: cm.wall_start,
                dur: duration,
                stream_id: 1, //Need to get this from activity
            };
            graph.add_transfer(transfer, v, y);
        }
        _ => {
            //Don't do anything
        }
    }

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
            }
            &CudaConfigureCall(ref cc) => {
                state = handle_cuda_configure_call(cc, state);
            }
            &CudaSetupArgument(ref sa) => {
                state = handle_cuda_setup_argument(&mut pdg, sa, state);
            }
            &CudaMemcpy(ref m) => {
                state = handle_cuda_memcpy(&mut pdg, m, state);
            }
            &CudaLaunch(ref cl) => {
                state = handle_cuda_launch(&mut pdg, cl, state);
            }
            _ => {
                println!("Nothing");
            }
        }
    }

    // for activity in doc.activities() {
    //     use activity::Record::*;
    //     match activity {
    //         &Kernel3(ref m) => {
    //             //Handle kernel launch activity
    //         }
    //         &Memcpy(ref m) => {
    //             //Handle memcpy activity
    //             state = handle_memcpy_activity(&mut pdg, m, state);
    //         }
    //         _ => panic!("Unexpected activity encountered!"),
    //     }
    // }

    // for cudnn_call in doc.cudnn_calls() {
    //     use cudnn::Record::*;
    //     match cudnn_call {
    //         _ => panic!("Unexpected cudnn activity encountered!"),
    //     }
    // }

    return pdg;
}
