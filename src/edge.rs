
#[derive(PartialEq, Clone, Copy)]
pub enum Edge<ID> {
    BiDirectional { a : ID, b: ID },
    MonoDirectional { from: ID, to: ID }
}

impl<ID> Edge<ID> 
where ID : PartialEq
{
    pub fn connects(& self, x : & ID, y : & ID) -> bool {
        match &self {
            Edge::MonoDirectional { from, to } => (*from == *x && *to == *y) || (*from == *x && *to == *y),
            Edge::BiDirectional { a, b } => (*a == *x && *b == *y) || ( *a == *y && *b == *x) 
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

