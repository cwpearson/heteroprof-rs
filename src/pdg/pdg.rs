extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;
use pdg::edge::Edge;
use pdg::compute::Compute;
use Document;
use pdg::transfer::Transfer;
use std::collections::HashMap;

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

pub fn from_document(doc: &Document) -> PDG {
    let mut pdg = PDG::new();

    for c in doc.apis() {
        // pdg.add_compute(c);
    }

    for t in doc.activities() {
        // pdg.add_transfer(t);
    }

    return pdg;
}
