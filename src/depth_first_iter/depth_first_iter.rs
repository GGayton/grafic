use crate::graph::Graph;

use nohash_hasher::{BuildNoHashHasher, IntSet, IsEnabled};
use std::hash::Hash;
use std::collections::HashSet;

impl<'a, ID, COST> Graph<ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
COST : Clone + Copy
{ 
    pub fn df_iter(&'a self, id: &'a ID) -> DepthFirstIter<ID, COST> {
        DepthFirstIter::<ID, COST>::new(id, self)
    }
}

pub struct DepthFirstIter<'a, ID, COST>
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
{
    graph: &'a Graph<ID, COST>,
    queue: Vec<&'a ID>,
    set: IntSet::<ID>,
}

impl<'a, ID, COST> DepthFirstIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>) -> DepthFirstIter<'a, ID, COST> {
        let mut queue = Vec::<&'a ID>::new();
        queue.push(id);
        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);
        DepthFirstIter {graph, queue, set}
    }
}

impl<'a, ID, COST> Iterator for DepthFirstIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    type Item = &'a ID;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.pop();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|id| match self.set.contains(id) {
                 false => {
                    _ = self.set.insert(id.clone());
                    self.queue.push(id);
                },
                 true => () }),
            None => ()
        };

        next
    }

} 