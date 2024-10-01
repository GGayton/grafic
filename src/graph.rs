
use crate::{edge::Edge, node::Node};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub struct Graph<ID, COST> 
{
    pub nodes: HashMap<ID, Node<ID, COST>>,
}

// Constructors
impl<ID, COST> Graph<ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
COST : Clone + Copy
{ 
    pub fn new() -> Graph<ID, COST> {
        Graph { nodes : HashMap::new() }
    }

    pub fn from_sparse(nodes: Vec<ID>, edges: Vec<(ID,ID,COST)>) -> Graph<ID, COST> {
        
        let mut map = HashMap::new();
        for node in nodes.into_iter(){
            match map.entry(node) {
                Entry::Occupied(_) => panic!("nodes must be a unique list!"),
                Entry::Vacant(v) => v.insert(Node::new())
            };
        }

        let mut graph = Graph{nodes : map};

        for (from, to, cost) in edges.into_iter() {
            graph.connect_nodes(from, to, cost);
        }

        return graph;
    }
}

// Mutators 
impl<ID, COST> Graph<ID, COST> 
where
ID : Eq + PartialEq + Hash + Clone + Copy,
COST : Clone + Copy
{ 

    /// Connects two nodes in the graph.
    /// Panics if edge is misconfigured
    pub fn connect_nodes(&mut self, a : ID, b : ID, cost : COST) {

        let a_to_b = Edge::Go { to: b, cost };
        let b_to_a = Edge::Go { to: a, cost };

        let mut add_to_node = |id : ID, edge : Edge<ID, COST>| {
            match self.nodes.entry(id) {
                Entry::Occupied(mut n) => n.get_mut().edges.push(edge),
                Entry::Vacant(_) => panic!("Attempted to connect a non-existant node - edge holds an incorrect id")
            }
        };

        add_to_node(a, a_to_b);
        add_to_node(b, b_to_a);

    }

    /// Disconnects two nodes in a graph.
    /// Does nothing if the connection does not exits.
    /// Panics if the nodes do not exist.
    pub fn disconnect_nodes(&mut self, a : & ID, b : & ID) {

        self.nodes
            .get_mut(a)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(b);
        
        self.nodes
            .get_mut(b)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(a);
    }


    pub fn destroy_node(&mut self, id : & ID) {
        let connected_nodes = self.nodes
            .get(id)
            .and_then(|node| Some(node.pseudo_neighbours().cloned().collect()))
            .unwrap_or(Vec::new());

        for i in connected_nodes {
            self.nodes
                .get_mut(&i)
                .expect("Attempted to obtain a non-existant node")
                .disconnect(id);
        }

        self.nodes.remove(&id);
    }

    /// Shorthand for inserting a node
    pub fn insert_node(&mut self, id : ID) -> Option<Node<ID, COST>> {
        self.nodes.insert(id, Node::<ID, COST>::new())
    }

    /// Removes node at id, and then continues to prune away neighbours that return true on the predicate.
    /// If no nodes at id, returns.
    pub fn prune_nodes(&mut self, id : ID, predicate : fn(&ID, &Node<ID, COST>) -> bool) {


        


        //let mut cont = self.nodes.contains_key(&id);

        //let mut current_id = id;

        //let future_id


        //while cont {

            //let neighbours : Vec<ID> = self.nodes[&current_id].pseudo_neighbours().cloned().collect();
            //self.destroy_node(&current_id);

            //let 
            //for neighbour in neighbours {
                
            //}
            
        //}

    }
}

// Iterators
impl<ID, COST> Graph<ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
COST : Clone
{
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, ID, Node<ID, COST>> {
        self.nodes.iter()
    }

    pub fn into_iter(&self) -> std::collections::hash_map::IntoIter<ID, Node<ID, COST>> {
        self.nodes.clone().into_iter()
    }

    pub fn iter_mut(& mut self) -> std::collections::hash_map::IterMut<'_, ID, Node<ID, COST>> {
        self.nodes.iter_mut()
    }
}

//impl Graph<'_> {
    //pub fn connect<'a>(&'a mut self, a : &'a mut Node<'a>, b : &'a mut Node<'a>, edge_creator : fn(&'a Node, &'a Node) -> Edge<'a>) {

        ////let edge = edge_creator(a, b);

        ////a.edges.push(&edge);
        ////b.edges.push(&edge);
        ////self.edges.push(edge);
    //}

    //pub fn connect_using_id<'a>(&'a mut self, i : &u64, j : &u64, edge_creator : fn(&'a Node, &'a Node) -> Edge<'a>) {

        ////let a = &self.nodes[&i];
        ////let b = &self.nodes[&j];

        //let a = Node { id : 0, edges: Vec::<&Edge<'a>>::new() };
        //let b = Node { id : 1, edges: Vec::<&Edge<'a>>::new() };

        //let edge = edge_creator(&a, &b);

        //self.edges.push(edge);
        
    //}
//}

//pub fn build_bi_directional_graph<'a>(g : Vec<(u64, Vec<i64>)>) -> Graph<'a> {

    //let mut nodes = HashMap::<u64, Node<'_>>::new();

    //let mut edges = Vec::<Edge<'a>>::new();

    //for (id, _) in g.iter() {
        //let node = Node { id : *id, edges :Vec::<&Edge<'a>>::new()};

        //nodes.insert(*id, node);
    //}

    //for (id, connects) in g.iter() {
        //let node = nodes.get_mut(id);

        
    //}

    //return Graph {nodes : nodes, edges : edges};

//}

