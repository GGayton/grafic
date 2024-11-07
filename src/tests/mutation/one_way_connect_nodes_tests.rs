use crate::graph::Graph;

fn create_graph() -> Graph<u16, f32> {
    let num_nodes: u16 = 4;
    let ids: Vec<u16> = (0..num_nodes).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

fn count_edges(graph: &Graph<u16, f32>) -> usize {
    graph
        .nodes
        .iter()
        .map(|(_, node)| node.edges.len())
        .sum::<usize>()
}

fn count_neighbours(graph: &Graph<u16, f32>) -> usize {
    graph
        .nodes
        .iter()
        .map(|(_, node)| node.neighbours().count())
        .sum::<usize>()
}

#[test]
#[should_panic]
fn panics_when_node_nonexistent() {
    let mut graph: Graph<u16, f32> = create_graph();
    graph.one_way_connect_nodes(0,5,1.0);
}

#[test]
fn increases_edge_count_by_2() {
    let mut graph: Graph<u16, f32> = create_graph();

    assert_eq!(count_edges(&graph), 0);

    graph.one_way_connect_nodes(0, 1, 1.0);
    assert_eq!(count_edges(&graph), 2);

    graph.one_way_connect_nodes(0, 1, 1.0);
    assert_eq!(count_edges(&graph), 4);

    graph.one_way_connect_nodes(1, 0, 1.0);
    assert_eq!(count_edges(&graph), 6);

    graph.one_way_connect_nodes(3,2, 1.0);
    assert_eq!(count_edges(&graph), 8);
}

#[test]
fn increases_neighbour_count_by_1() {
    let mut graph: Graph<u16, f32> = create_graph();

    assert_eq!(count_neighbours(&graph), 0);

    graph.one_way_connect_nodes(0, 1, 1.0);
    assert_eq!(count_neighbours(&graph), 1);

    graph.one_way_connect_nodes(0, 1, 1.0);
    assert_eq!(count_neighbours(&graph), 2);

    graph.one_way_connect_nodes(1, 0, 1.0);
    assert_eq!(count_neighbours(&graph), 3);

    graph.one_way_connect_nodes(3,2, 1.0);
    assert_eq!(count_neighbours(&graph), 4);
}