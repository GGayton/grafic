use crate::types::Identity;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Edge<Id, Cost> {
    Go { to : Id, cost: Cost },
    NoGo { to: Id } 
}

impl<Id,Cost> Edge<Id,Cost> where Id : Identity
{
    /// Returns true if the destination of this edge is dest
    pub fn connects(& self, dest : & Id) -> bool {
        match &self {
            Edge::Go { to, .. } => to == dest ,
            Edge::NoGo {..} => false 
        }
    }

    /// Returns the node id this edge connects to or originates from
    pub fn pseudo_connects(& self, dest : Id) -> bool {
        match &self {
            Edge::Go { to, .. } => *to == dest,
            Edge::NoGo {to} => *to == dest 
        }
    }

    /// Traverses the node
    pub fn traverse(& self) -> Option<&Id> {
        match &self {
            Edge::Go { to, .. } => Some(to) ,
            Edge::NoGo {..} => None 
        }
    }
}




