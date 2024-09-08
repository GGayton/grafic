#[cfg(test)]
pub mod connect_nodes_test {
    use crate::graph::Graph;
    use crate::edge::Edge;


    #[test]
    fn add_nodes() {

    }

    #[test]
    fn remove_nodes() {

    }

    #[test]
    #[should_panic]
    fn connect_nodes() {
        let num_nodes : u16 = 4;

        let mut rng = rand::thread_rng();
        let ids : Vec<u16> = (0..num_nodes).collect();

        let edges : Vec<Edge<u16>> = Vec::new();
        
        let graph = Graph::<u16>::from_sparse(ids, edges);

        assert_

    }

    #[test]
    fn disconnect_nodes() {

    }

    #[test]
    fn try_connect_nodes() {

    }

    #[test]
    fn try_disconnect_nodes() {
        
    }

}