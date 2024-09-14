use crate::edge::Edge;

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

    pub fn disconnect(&mut self, from : &ID) {
        self.edges.retain(|edge| !edge.connects(from));
        //self.edges = self.edges.iter().filter(|&edge| !edge.connects(to)).cloned().collect();
    }
}
