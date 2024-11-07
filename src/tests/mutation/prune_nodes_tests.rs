use crate::graph::Graph;

fn create_graph(num_nodes : u16) -> Graph<u16, f32> {
    let ids: Vec<u16> = (0..num_nodes).collect();
    let edges: Vec<(u16, u16, f32)> = Vec::new();
    Graph::<u16, f32>::from_sparse(ids, edges)
}

#[test]
fn does_nothing_if_starting_node_non_existent() {
    let mut graph = create_graph(4);

    graph.prune_nodes(10, |_,_| { true });
}

#[test]
fn removes_items_when_predicate_is_true() {
    let mut graph = create_graph(10);

    graph.prune_nodes(0, |_,_| {true});

    assert_eq!(graph.nodes.len(), 9)
}

#[test]
fn removes_all_items_when_predicate_is_true_on_connected_graph() {

    let mut graph = create_graph(4);
    graph.connect_nodes(0, 1, 1.0);
    graph.connect_nodes(1, 2, 1.0);
    graph.connect_nodes(2, 3, 1.0);

    graph.prune_nodes(0, |_,_| {true});

    assert_eq!(graph.nodes.len(), 0)
}

#[test]
fn remove_all_unconnected_connected_nodes() {

    let mut graph = create_graph(100);

    // connect all in big loop
    for x in (0..100)
        .step_by(2)
        .collect::<Vec<u16>>()
        .windows(2) {

            if let [prev, next] = x { graph.connect_nodes(*prev, *next, 1.0); }

        }
    graph.connect_nodes(0, 98, 1.0);

    for i in 0..100 {
        graph.prune_nodes(i, | _, pnode | {
            pnode.edges.len() < 2
        });
    }

    assert_eq!(graph.nodes.len(), 50);


    for node in graph.nodes.values() {
        assert!( node.edges.len() == 2 )
    }

    for id in graph.nodes.keys() {
        assert!( id % 2 == 0)
    }
}

#[test]
fn remove_all_singly_connected_nodes() {

    let mut graph = create_graph(103);

    // connect all in big loop
    for x in (0..100)
        .collect::<Vec<u16>>()
        .windows(2) {

            if let [prev, next] = x { graph.one_way_connect_nodes(*next, *prev, 1.0); }

        }

    graph.connect_nodes(0,   100, 1.0);
    graph.connect_nodes(100, 101, 1.0);
    graph.connect_nodes(101, 102, 1.0);
    graph.connect_nodes(102, 0,   1.0);

    
    //println!("{:?}", graph.nodes[&99].neighbours().cloned().collect::<Vec<_>>());
    graph.prune_nodes(99, | _, pnode | {
        //println!("{:?}", pnode.neighbours().count());
        pnode.neighbours().count() < 2
    });


    assert_eq!(graph.nodes.len(), 4);
    assert!(graph.nodes.contains_key(&0));
    assert!(graph.nodes.contains_key(&100));
    assert!(graph.nodes.contains_key(&101));
    assert!(graph.nodes.contains_key(&102));


}