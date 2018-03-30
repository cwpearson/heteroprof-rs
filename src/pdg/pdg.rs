extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;
use std::collections::{HashMap, HashSet};
use pdg::edge::Edge2;
use pdg::edge;
use pdg::node::{ComputeS, Node, TransferS, ValueS};
use Document;
use callback;
use cuda;
use cuda::allocation::{AddressSpace, Allocation};

pub struct PDG<'a> {
    next_id: usize,
    pub computes: HashSet<edge::ComputeS>,
    pub transfers: HashSet<edge::TransferS>,
    pub graph: DiGraphMap<Node, Edge2<'a>>,
}

impl<'a> PDG<'a> {
    pub fn new() -> PDG<'a> {
        return PDG {
            next_id: 0,
            computes: HashSet::new(),
            transfers: HashSet::new(),
            graph: DiGraphMap::new(),
        };
    }

    pub fn new_value(&mut self) -> Node {
        let v = ValueS::new(self.next_id);
        self.next_id += 1;
        Node::Value(v)
    }

    pub fn new_transfer(&mut self) -> Edge2 {
        let e = edge::TransferS::new();
        self.transfers.insert(e);
        Edge2::Transfer(&e)
    }
}

fn handle_cuda_malloc(cm: &callback::CudaMallocS, state: &mut cuda::State) {
    let allocation = Allocation::new(cm.ptr, cm.size, AddressSpace::UVA);
    state.allocations.insert(allocation);
    println!("Inserted allocation at {}!", allocation.pos);
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

fn handle_cuda_set_device(csd: &callback::CudaSetDeviceS, state: &mut cuda::State) {
    state[csd.calling_tid].device = csd.device;
}

fn handle_cuda_memcpy(cm: &callback::CudaMemcpyS, state: &mut cuda::State, pdg: &mut PDG) {
    // create the transfer
    let transfer = pdg.new_transfer();
    let dst = pdg.new_value();

    // find the src allocation
    let src_pos = cm.src;
    let src_size = cm.count;
    let mut iter = state.allocations.iter();
    let src_alloc = match iter.find(|&a| a.contains(src_pos)) {
        Some(alloc) => alloc,
        _ => panic!("ahh"),
    };

    // find the current value
    let src_val = src_alloc.get_value(src_pos, src_size);

    pdg.graph.add_edge(transfer, dst, Edge2::Transfer(transfer));

    // find the dst allocation
    let dst_pos = cm.dst;
}

pub fn from_document(doc: &Document) -> PDG {
    println!("WHAT");
    let mut state = cuda::State::new();
    let mut pdg = PDG::new();

    for api in doc.apis() {
        use callback::Record::*;
        match api {
            &CudaMalloc(ref m) => {
                handle_cuda_malloc(m, &mut state);
                println!("{} allocations", state.allocations.len());
            }
            _ => (),
            &CudaSetDevice(ref csd) => {
                handle_cuda_set_device(csd, &mut state);
            }
            &CudaConfigureCall(ref cc) => {
                state = handle_cuda_configure_call(cc, state);
            }
            &CudaSetupArgument(ref sa) => {
                state = handle_cuda_setup_argument(sa, state);
            }
            &CudaMemcpy(ref m) => {
                handle_cuda_memcpy(m, &mut state, &mut pdg);
            }
            _ => (),
        }
    }

    // Second pass: set up values from kernels / memcopies
    for api in doc.apis() {
        use callback::Record::*;
    }

    return pdg;
}

#[test]
fn pdg_test() {
    use std::io::BufReader;
    use decode_document;
    let data = r#"{"build":"20180327-040238+0000","git":"dirty","version":"0.1.0"}
{"calling_tid":4063,"context_uid":0,"correlation_id":198,"device":0,"hprof_kind":"cupti_callback","id":1,"name":"cudaSetDevice","symbol_name":"","wall_end":1522123362585161708,"wall_start":1522123362585079220}
{"calling_tid":4063,"context_uid":0,"correlation_id":200,"hprof_kind":"cupti_callback","id":2,"name":"cudaMalloc","ptr":140481568702464,"size":409600,"symbol_name":"","wall_end":1522123362754865543,"wall_start":1522123362586869757}
{"calling_tid":4063,"context_uid":1,"correlation_id":201,"hprof_kind":"cupti_callback","id":3,"name":"cudaMalloc","ptr":140481569112064,"size":819200,"symbol_name":"","wall_end":1522123362754944954,"wall_start":1522123362754926973}
{"calling_tid":4063,"context_uid":1,"correlation_id":202,"hprof_kind":"cupti_callback","id":4,"name":"cudaMalloc","ptr":140481569931264,"size":819200,"symbol_name":"","wall_end":1522123362754981414,"wall_start":1522123362754968138}"#;
    let mut reader = BufReader::new(data.as_bytes());
    let doc: Document = decode_document(&mut reader).unwrap();
    let pdg = from_document(&doc);
}
