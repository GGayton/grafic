use crate::edge::Edge;
use std::ptr::eq;

#[derive(Clone)]
pub struct Node<ID> {
    pub edges: Vec<Edge<ID>>
}

impl<ID> Node<ID> 
where ID : PartialEq + Clone
{

    pub fn new() -> Node<ID> {
        Node { edges : Vec::<Edge<ID>>::new() }
    }

    pub fn push_edge(&mut self, edge : Edge<ID>) {
        self.edges.push(edge);
    }

    pub fn disconnect(&mut self, a : & ID, b : &ID) {
        self.edges = self.edges.iter().filter(|&edge| !edge.connects(a, b)).cloned().collect();
    }
}

impl<ID> PartialEq for Node<ID> {
    fn eq(&self, other: &Self) -> bool {
        return eq(self, other);
    }
    fn ne(&self, other: &Self) -> bool {
        return eq(self, other);
    }
}



