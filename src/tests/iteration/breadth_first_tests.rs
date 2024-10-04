
use crate::graph::Graph;

const NUM_NODES: u16 = 4;

fn create_graph() -> Graph<u16, f32> {
    let ids: Vec<u16> = (0..NUM_NODES).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

#[test]
fn breadth_first_identical_search() {
    
    let mut graph = create_graph();
    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);
    graph.connect_nodes(3, 1, 1.0);
    graph.connect_nodes(2, 0, 1.0);

    let search : Vec::<u16> = graph.bf_search(&0, |_| true).cloned().collect();
    let iter : Vec::<u16> = graph.bf_iter(&0).cloned().collect();
    let into_iter : Vec::<u16> = graph.bf_into_iter(&0).collect();

    assert_eq!(search, iter);
    assert_eq!(search, into_iter);

}

