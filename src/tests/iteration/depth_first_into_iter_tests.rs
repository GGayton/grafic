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
        let path : Vec<u16> = graph.df_into_iter(&i).collect();
        assert_eq!(path.len(), 4);
    }
}

#[test]
fn traverses_graph_depth_first() {
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

    let path : Vec<u8> = graph.df_into_iter(&0).collect();

    assert_eq!(path, [0, 2, 6, 5, 1, 4, 3])
}