extern crate petgraph;

use self::petgraph::graphmap::DiGraphMap;

use pdg::node::Node;
use pdg::edge::Edge;
use Document;

type PDG = DiGraphMap<Node, Edge>;

pub fn from_document(doc: &Document) -> PDG {
    let mut pdg = PDG::new();

    for c in doc.computes() {
        pdg.add_node(Node { start: c.start });
    }

    for t in doc.transfers() {}

    return pdg;
}
