
use nohash_hasher::IntSet;
use crate::node::PseudoNode;
use crate::types::{Identity, Scalar};
use crate::{edge::Edge, node::Node};
use std::arch::x86_64::_SIDD_CMP_EQUAL_ORDERED;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub struct Graph<Id, Cost> 
{
    pub nodes: HashMap<Id, Node<Id, Cost>>,
}

// Constructors
impl<Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 
    pub fn new() -> Graph<Id, Cost> {
        Graph { nodes : HashMap::new() }
    }

    pub fn from_sparse(nodes: Vec<Id>, edges: Vec<(Id,Id,Cost)>) -> Graph<Id, Cost> {
        
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
impl<Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 

    fn add_to_node(&mut self, id : Id, edge : Edge<Id, Cost>) {
            match self.nodes.entry(id) {
                Entry::Occupied(mut n) => n.get_mut().edges.push(edge),
                Entry::Vacant(_) => panic!("Attempted to connect a non-existant node - edge holds an incorrect id")
            }
    }

    /// Connects two nodes in the graph.
    /// Panics if edge is misconfigured
    pub fn connect_nodes(&mut self, a : Id, b : Id, cost : Cost) {

        let a_to_b = Edge::Go { to: b, cost };
        let b_to_a = Edge::Go { to: a, cost };

        self.add_to_node(a, a_to_b);
        self.add_to_node(b, b_to_a);
    }

    pub fn one_way_connect_nodes(&mut self, from : Id, to : Id, cost : Cost) {

        let go = Edge::Go { to, cost };
        let no_go = Edge::NoGo { to: from };

        self.add_to_node(from, go);
        self.add_to_node(to, no_go);
    }

    /// Disconnects two nodes in a graph.
    /// Does nothing if the connection does not exits.
    /// Panics if the nodes do not exist.
    pub fn disconnect_nodes(&mut self, a : & Id, b : & Id) {

        self.nodes
            .get_mut(a)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(b);
        
        self.nodes
            .get_mut(b)
            .expect("Attempted to obtain a non-existant node")
            .disconnect(a);
    }


    pub fn destroy_node(&mut self, id : & Id) {
        if let Some(connected_nodes) = self.nodes
            .get(id)
            .and_then(|node| Some(node.pseudo_neighbours().cloned().collect::<Vec<_>>())) {
            
        for i in connected_nodes {
            self.nodes
                .get_mut(&i)
                .expect("Attempted to obtain a non-existant node")
                .disconnect(id);
            
            }
        }

        self.nodes.remove(&id);
    }

    /// Shorthand for inserting a node
    pub fn insert_node(&mut self, id : Id) -> Option<Node<Id, Cost>> {
        self.nodes.insert(id, Node::<Id, Cost>::new())
    }

    /// Removes node at id, and then continues to prune away neighbours that return true on the predicate.
    /// If no nodes at id, returns.
    pub fn prune_nodes<'a>(&'a mut self, id : Id, predicate : fn(PseudoNode<Id, Cost>) -> bool) 
    {
        // The pseudo state of the graph
        // Contains ids of nodes to removed
        let mut set = IntSet::<Id>::default();

        // Function first creates a pseudo node (a node with flagged nodes removed), then
        // forwards the psuedo node to the predicate.
        // The predicate decides both nodes to remove and the search direction.
        let func = |id : & Id| { 
            
            let pseudo_node = PseudoNode::new(
                *id, 
                self.nodes[id]
                    .edges
                    .iter()
                    .filter(|&edge| { 
                        match edge {
                            | Edge::Go { to, .. } => !set.contains(to),
                            | Edge::NoGo { .. } => false
            }}).cloned().collect());
            
            if predicate(pseudo_node)
            {
                set.insert(*id);
                true   
            } else {
                false
            }
        
         };

        self.bf_search(&id, func);

        for i in set {
            self.destroy_node(&i);
        }
        
        ()

    }
}

// Iterators
impl<Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Id, Node<Id, Cost>> {
        self.nodes.iter()
    }

    pub fn into_iter(&self) -> std::collections::hash_map::IntoIter<Id, Node<Id, Cost>> {
        self.nodes.clone().into_iter()
    }

    pub fn iter_mut(& mut self) -> std::collections::hash_map::IterMut<'_, Id, Node<Id, Cost>> {
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

