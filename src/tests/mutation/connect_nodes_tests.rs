use crate::edge::Edge;
use crate::graph::Graph;

fn create_graph() -> Graph<u16> {
    let num_nodes: u16 = 4;
    let ids: Vec<u16> = (0..num_nodes).collect();
    let edges: Vec<Edge<u16>> = Vec::new();
    Graph::<u16>::from_sparse(ids, edges)
}

#[test]
#[should_panic]
fn panics_when_node_nonexistent() {
    let mut graph: Graph<u16> = create_graph();
    let edge = Edge::MonoDirectional { from: 0, to: 5 };
    graph.connect_nodes(edge);
}

#[test]
fn increases_edge_count_by_2() {
    let mut graph: Graph<u16> = create_graph();

    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|(_, node)| node.edges.len())
            .sum::<usize>(),
        0
    );

    graph.connect_nodes(Edge::MonoDirectional { from: 0, to: 1 });
    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|(_, node)| node.edges.len())
            .sum::<usize>(),
        2
    );

    graph.connect_nodes(Edge::MonoDirectional { from: 0, to: 1 });
    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|(_, node)| node.edges.len())
            .sum::<usize>(),
        4
    );

    graph.connect_nodes(Edge::MonoDirectional { from: 1, to: 0 });
    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|(_, node)| node.edges.len())
            .sum::<usize>(),
        6
    );

    graph.connect_nodes(Edge::BiDirectional { a: 3, b: 2 });
    assert_eq!(
        graph
            .nodes
            .iter()
            .map(|(_, node)| node.edges.len())
            .sum::<usize>(),
        8
    );
}
