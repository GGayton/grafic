use crate::{edge::Edge, graph::Graph};

fn create_graph() -> Graph<u16, f32> {
    let num_nodes: u16 = 4;
    let ids: Vec<u16> = (0..num_nodes).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

fn count_edges(graph: &Graph<u16, f32>, predicate: fn(&Edge<u16, f32>) -> bool) -> usize {
    graph
        .nodes
        .iter()
        .map(|(_, node)| 
            node.edges
                .iter()
                .filter(|&edge| predicate(edge))
                .count())
        .sum::<usize>()
}

#[test]
fn does_nothing_when_node_nonexistent() {
    let mut graph: Graph<u16, f32> = create_graph();
    graph.destroy_node(5);
}

#[test]
fn no_edges_to_destroyed_node() {
    let mut graph: Graph<u16, f32> = create_graph();

    graph.connect_nodes(1,0,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,2,1.0);
    graph.connect_nodes(1,3,1.0);

    graph.destroy_node(1);

    fn connects_to_1(edge : &Edge<u16, f32>) -> bool {
        edge.pseudo_connects(1)
    }

    let count = count_edges(&graph, connects_to_1);

    assert_eq!(count, 0);
}

#[test]
fn removes_node() {
    let mut graph: Graph<u16, f32> = create_graph();

    graph.connect_nodes(1,0,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,2,1.0);
    graph.connect_nodes(1,3,1.0);

    graph.destroy_node(1);

    assert_eq!(graph.nodes.len(), 3);
}



#[test]
fn retains_unaffected_edges() {
    let mut graph: Graph<u16, f32> = create_graph();

    graph.connect_nodes(1,0,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,1,1.0);
    graph.connect_nodes(1,2,1.0);
    graph.connect_nodes(1,3,1.0);

    graph.connect_nodes(0,0,1.0);
    graph.connect_nodes(3,2,1.0);
    graph.connect_nodes(2,0,1.0);
    graph.connect_nodes(3,2,1.0);
    graph.connect_nodes(0,3,1.0);

    graph.destroy_node(1);

    fn any(_ : &Edge<u16, f32>) -> bool {
        true
    }

    let count = count_edges(&graph, any);

    assert_eq!(count, 10)
}

