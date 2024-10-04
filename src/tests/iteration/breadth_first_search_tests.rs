use crate::graph::Graph;

const NUM_NODES: u16 = 4;

fn create_graph() -> Graph<u16, f32> {
    let ids: Vec<u16> = (0..NUM_NODES).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

#[test]
fn traverses_entire_graph_once() {
    let mut graph = create_graph();

    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);
    graph.connect_nodes(0, 2, 1.0);
    graph.connect_nodes(1, 3, 1.0);
    graph.connect_nodes(2, 2, 1.0);

    for i in 0..NUM_NODES { 
        let path : Vec<u16> = graph.bf_search(&i, |_| true).cloned().collect();
        assert_eq!(path.len(), 4);
    }
}

#[test]
fn traverses_graph_breadth_first() {
    let mut graph = Graph::<u8, f32>::new();

    for i in 0..7 {
        _ = graph.insert_node(i);
    }

    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(0, 2, 1.0);

    graph.connect_nodes(1, 3, 1.0);
    graph.connect_nodes(1, 4, 1.0);

    graph.connect_nodes(2, 5, 1.0);
    graph.connect_nodes(2, 6, 1.0);

    let path : Vec<u8> = graph.bf_search(&0, |_| true).cloned().collect();

    assert_eq!(path, [0, 1, 2, 3, 4, 5, 6])
}

#[test]
fn starting_at_non_existent_node_gives_empty_iter() {

    let graph = create_graph();

    let count = graph.bf_search(&99, |_| true).count();

    assert_eq!(count, 0)
}

#[test]
fn follows_state_in_closure() {
    let mut graph = create_graph();
    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);
    graph.connect_nodes(3, 1, 1.0);
    graph.connect_nodes(2, 0, 1.0);

    let mut i = 0;

    let count = graph.bf_search(&0, |_| {
        if i <= 2 {
            i += 1;
            true
        } else {
            false
        }
    }).count();

    assert_eq!(count, 4);
}