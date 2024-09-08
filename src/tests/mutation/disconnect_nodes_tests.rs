use crate::edge::Edge;
use crate::graph::Graph;

fn create_graph() -> Graph<u16> {
    let num_nodes: u16 = 4;
    let ids: Vec<u16> = (0..num_nodes).collect();
    let edges: Vec<Edge<u16>> = Vec::new();
    Graph::<u16>::from_sparse(ids, edges)
}

fn count_edges(graph: &Graph<u16>) -> usize {
    graph
        .nodes
        .iter()
        .map(|(_, node)| node.edges.len())
        .sum::<usize>()
}

#[test]
#[should_panic]
fn panics_when_node_nonexistent() {
    let mut graph: Graph<u16> = create_graph();
    let edge = Edge::MonoDirectional { from: 0, to: 5 };
    graph.connect_nodes(edge);
}

#[test]
fn decreases_edge_count_by_2() {
    let mut graph: Graph<u16> = create_graph();

    assert_eq!(count_edges(&graph), 0);

    graph.connect_nodes(Edge::MonoDirectional { from: 0, to: 1 });
    graph.connect_nodes(Edge::MonoDirectional { from: 1, to: 2 });
    graph.connect_nodes(Edge::BiDirectional { a: 2, b: 3 });

    assert_eq!(count_edges(&graph), 6);

    graph.disconnect_nodes(&1,&2);
    assert_eq!(count_edges(&graph), 4);

    graph.disconnect_nodes(&0,&1);
    assert_eq!(count_edges(&graph), 2);

    graph.disconnect_nodes(&2,&3);
    assert_eq!(count_edges(&graph), 0);
}

#[test]
fn disconnect_nonexistent_edge_does_nothing() {
    let mut graph: Graph<u16> = create_graph();

    assert_eq!(count_edges(&graph), 0);

    graph.connect_nodes(Edge::MonoDirectional { from: 0, to: 1 });
    graph.connect_nodes(Edge::BiDirectional { a: 1, b: 2 });
    graph.connect_nodes(Edge::MonoDirectional { from: 2, to: 3 });
    assert_eq!(count_edges(&graph), 6);

    graph.disconnect_nodes(&1,&3);
    assert_eq!(count_edges(&graph), 6);

    graph.disconnect_nodes(&2,&2);
    assert_eq!(count_edges(&graph), 6);
}