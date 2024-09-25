
#[derive(PartialEq, Clone, Copy)]
pub enum Edge<ID, COST> {
    Go { to : ID, cost: COST },
    NoGo { to: ID } 
}

impl<ID,COST> Edge<ID,COST> 
where ID : PartialEq
{
    /// Returns true if the edge destination is the 
    pub fn connects(& self, dest : & ID) -> bool {
        match &self {
            Edge::Go { to, .. } => to == dest ,
            Edge::NoGo {..} => false 
        }
    }

    /// Returns the node id this edge connects to
    pub fn destination(& self) -> &ID {
        match &self {
            Edge::Go { to, .. } => to ,
            Edge::NoGo {to} => to 
        }
    }

    /// Traverses the node
    pub fn traverse(& self) -> Option<&ID> {
        match &self {
            Edge::Go { to, .. } => Some(to) ,
            Edge::NoGo {..} => None 
        }
    }
}


//impl<ID> Edge<ID> {
    //pub fn travel_from(&self, node: Edge<ID>) -> Option<Edge<ID>> {
        //return match &self {
            //Edge::BiDirectional{ a, b} if node == *a => Some(b),
            //Edge::BiDirectional{ a, b} if node == *b => Some(a),
            //Edge::MonoDirectional { from, to } if node == *from => Some(to),
            //_ => None
        //}
    //}

    ////pub fn connect(&mut self) {
        
        ////match &self
        ////{
            ////Edge::BiDirectional { a, b} => {
                //////let test = AsMut::as_mut(a);
                //////a.edges.as_mut::<Vec<&Edge<'_>>>();
                //////let test = AsMut::<Vec<&Edge<'_>>>::as_mut(a.edges);
                ////a.push_edge(self);
                ////b.edges.push(self);
            ////},
            ////Edge::MonoDirectional { from: a, to: b } => {
                ////a.edges.push(self);
                ////b.edges.push(self);
            ////},
        ////}
        
    ////}

//}

