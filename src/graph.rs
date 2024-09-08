
use crate::{edge::Edge, node::Node};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub struct Graph<ID> 
{
    pub nodes: HashMap<ID, Node<ID>>,
}

// Constructors
impl<ID> Graph<ID> 
where ID : Eq + PartialEq + Hash + Clone + Copy
{ 
    pub fn new() -> Graph<ID> {
        Graph { nodes : HashMap::new() }
    }

    pub fn from_sparse(nodes: Vec<ID>, edges: Vec<Edge<ID>>) -> Graph<ID> {
        
        let mut map = HashMap::new();
        for node in nodes.into_iter(){
            match map.entry(node) {
                Entry::Occupied(_) => panic!("nodes must be a unique list!"),
                Entry::Vacant(v) => v.insert(Node::new())
            };
        }

        let mut graph = Graph{nodes : map};

        for edge in edges.into_iter() {
            graph.connect_nodes(edge);
        }

        return graph;
    }
}

// Mutators 
impl<ID> Graph<ID> 
where ID : Eq + PartialEq + Hash + Clone + Copy
{ 

    /// Connects two nodes in the graph.
    /// Panics if edge is misconfigured
    pub fn connect_nodes(&mut self, edge : Edge<ID>) {

        let mut add_to_node = |id : ID| {
            match self.nodes.entry(id) {
                Entry::Occupied(mut n) => n.get_mut().edges.push(edge),
                Entry::Vacant(_) => panic!("Attempted to connect a non-existant node - edge holds an incorrect id")
            }
        };

        match edge {
            Edge::MonoDirectional { from, to } => {
                add_to_node(from);
                add_to_node(to);
            }
            Edge::BiDirectional { a, b } => {
                add_to_node(a);
                add_to_node(b);
            }
        }

        // /match edge {
            // /Edge::MonoDirectional { from, to } => {
                // /self.nodes.entry(from).and_modify(|e| e.edges.push(edge));

                // /self.nodes.entry(to).and_modify(|e| e.edges.push(edge));
            // /}
            // /Edge::BiDirectional { a, b } => {
                // /self.nodes.entry(a).and_modify(|e| e.edges.push(edge));
                // /self.nodes.entry(b).and_modify(|e| e.edges.push(edge));
            // /}
        // /}
    }

    /// Disconnects two nodes in a graph.
    /// Does nothing if the connection does not exits.
    /// Panics if the nodes do not exist.
    pub fn disconnect_nodes(&mut self, a : & ID, b : & ID) {

        self.nodes
            .get_mut(a)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(a, b);
        
        self.nodes
            .get_mut(b)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(a, b);
    }

}

// Iterators
impl<ID> Graph<ID> 
where ID : Eq + PartialEq + Hash + Clone + Copy
{
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, ID, Node<ID>> {
        self.nodes.iter()
    }

    pub fn into_iter(&self) -> std::collections::hash_map::IntoIter<ID, Node<ID>> {
        self.nodes.clone().into_iter()
    }

    pub fn iter_mut(& mut self) -> std::collections::hash_map::IterMut<'_, ID, Node<ID>> {
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

