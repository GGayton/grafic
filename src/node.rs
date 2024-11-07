use crate::types::{Scalar, Identity};
use crate::edge::Edge;

#[derive(Clone)]
pub struct Node<Id,Cost> {
    pub edges: Vec<Edge<Id,Cost>>
}

impl<Id,Cost> Node<Id,Cost> where Id : Identity,
{

    pub fn from_edges(edges : Vec<Edge<Id,Cost>>) -> Node<Id, Cost> {
        Node { edges }
    }

    pub fn new() -> Node<Id,Cost> {
        Node { edges : Vec::<Edge<Id,Cost>>::new() }
    }

    /// Add a new edge on this node
    pub fn push_edge(&mut self, edge : Edge<Id,Cost>) {
        self.edges.push(edge);
    }

    /// destroy an edge on this node
    pub fn disconnect(&mut self, from : Id) {
        self.edges.retain(|edge| !edge.pseudo_connects(from));
    }
    
    /// Return all possible neighbours this node is connected to
    pub fn neighbours(&self) -> impl Iterator<Item = &Id> {
        self.edges.iter().filter_map(|edge| match edge {
            Edge::Go { to, .. } => Some(to),
            Edge::NoGo { .. } => None
        })
    }

    /// Return all possible neighbours this is node is connected to or are connect to this node
    pub fn pseudo_neighbours(&self) -> impl Iterator<Item = &Id> {
        self.edges.iter().map(|edge| match edge {
            Edge::Go {to, ..} => to,
            Edge::NoGo {to, ..} => to
        })
    }
}

/// Used for temporary states, creating a temporary node of an updated state for testing
pub struct PseudoNode<Id, Cost> 
where 
Id: Identity,
Cost: Scalar,
{
    pub id : Id,
    pub edges: Vec<Edge<Id,Cost>>
}

impl<Id, Cost> PseudoNode<Id, Cost> 
where 
Id: Identity,
Cost: Scalar
{
    pub fn new(id : Id, edges : Vec<Edge<Id,Cost>>) -> Self {
        Self { id, edges }
    }
}

