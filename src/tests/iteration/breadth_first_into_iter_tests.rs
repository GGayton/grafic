use crate::graph::Graph;


fn create_graph(n : u16) -> Graph<u16, f32> {
    let ids: Vec<u16> = (0..n).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

#[test]
fn traverses_entire_graph_once() {
    let mut graph = create_graph(4);

    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);
    graph.connect_nodes(0, 2, 1.0);
    graph.connect_nodes(1, 3, 1.0);
    graph.connect_nodes(2, 2, 1.0);

    for i in 0..4 { 
        let path : Vec<u16> = graph.bf_into_iter(&i).collect();
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

    let path : Vec<u8> = graph.bf_into_iter(&0).collect();

    assert_eq!(path, [0, 1, 2, 3, 4, 5, 6])
}

#[test]
fn starting_at_non_existent_node_gives_empty_iter() {

    let graph = create_graph(4);

    let count = graph.bf_into_iter(&99).count();

    assert_eq!(count, 0)
}


#[test]
fn mutation_safe() {
    let mut graph = create_graph(10);

    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);
    graph.connect_nodes(3, 4, 1.0);
    graph.connect_nodes(4, 5, 1.0);

    graph.connect_nodes(6, 0, 1.0);
    graph.connect_nodes(7, 0, 1.0);
    graph.connect_nodes(8, 0, 1.0);
    graph.connect_nodes(9, 0, 1.0);

    let iter = graph.bf_into_iter(&0);

    // immutable borrow from iter prevents mutation here
    graph.destroy_node(&6);
    graph.destroy_node(&7);
    graph.destroy_node(&8);
    graph.destroy_node(&9);

    //let vec : Vec<u16> = iter.collect(); 


}