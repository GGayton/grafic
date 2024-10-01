use crate::edge::Edge;

#[derive(Clone)]
pub struct Node<ID,COST> {
    pub edges: Vec<Edge<ID,COST>>
}

impl<ID,COST> Node<ID,COST> 
where
ID : PartialEq + Clone,
{
    pub fn new() -> Node<ID,COST> {
        Node { edges : Vec::<Edge<ID,COST>>::new() }
    }

    /// Add a new edge on this node
    pub fn push_edge(&mut self, edge : Edge<ID,COST>) {
        self.edges.push(edge);
    }

    /// destroy an edge on this node
    pub fn disconnect(&mut self, from : &ID) {
        self.edges.retain(|edge| !edge.connects(from));
    }
    
    /// Return all possible neighbours this node is connected to
    pub fn neighbours(&self) -> impl Iterator<Item = &ID> {
        self.edges.iter().filter_map(|edge| match edge {
            Edge::Go { to, .. } => Some(to),
            Edge::NoGo { .. } => None
        })
    }

    /// Return all possible neighbours this is node is connected to or are connect to this node
    pub fn pseudo_neighbours(&self) -> impl Iterator<Item = &ID> {
        self.edges.iter().map(|edge| match edge {
            Edge::Go {to, ..} => to,
            Edge::NoGo {to, ..} => to
        })
    }
}
