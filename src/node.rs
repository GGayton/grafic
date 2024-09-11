use crate::edge::Edge;
use std::ptr::eq;

#[derive(Clone)]
pub struct Node<ID,COST> {
    pub edges: Vec<Edge<ID,COST>>
}

impl<ID,COST> Node<ID,COST> 
where
ID : PartialEq + Clone,
COST : Clone
{

    pub fn new() -> Node<ID,COST> {
        Node { edges : Vec::<Edge<ID,COST>>::new() }
    }

    pub fn push_edge(&mut self, edge : Edge<ID,COST>) {
        self.edges.push(edge);
    }

    pub fn disconnect(&mut self, to : &ID) {
        self.edges = self.edges.iter().filter(|&edge| !edge.connects(to)).cloned().collect();
    }
}

//impl<ID,COST> PartialEq for Node<ID,COST> {
    //fn eq(&self, other: &Self) -> bool {
        //return eq(self, other);
    //}
    //fn ne(&self, other: &Self) -> bool {
        //return eq(self, other);
    //}
//}



