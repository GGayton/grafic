
use crate::{edge::Edge, graph::Graph};
use rand::{distributions::Uniform, prelude::*};

#[test]
fn from_sparse_generates() {
    let num_nodes: u16 = 1000;
    let num_edges: u16 = 10000;

    let mut rng = rand::thread_rng();
    let ids: Vec<u16> = (0..num_nodes).collect();

    let range = Uniform::<u16>::new(0, num_nodes);

    let edges: Vec<Edge<u16>> = (0..num_edges)
        .map(|_| (rng.sample(&range), rng.sample(&range)))
        .map(|(a, b)| Edge::<u16>::BiDirectional { a: a, b: b })
        .collect();

    let graph = Graph::<u16>::from_sparse(ids, edges);

    assert_eq!(graph.iter().count(), usize::from(num_nodes));
    // Edges are installed on eahc connecting node
    assert_eq!(
        graph
            .iter()
            .map(|(_, node)| node.edges.iter().count())
            .sum::<usize>(),
        usize::from(num_edges * 2)
    );
}
