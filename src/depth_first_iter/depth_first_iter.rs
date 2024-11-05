use crate::types::Scalar;
use crate::graph::Graph;
use crate::types::Identity;

use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 
    pub fn df_iter(&'a self, id: &'a Id) -> DepthFirstIter<Id, Cost> {
        DepthFirstIter::<Id, Cost>::new(id, self)
    }
}

pub struct DepthFirstIter<'a, Id, Cost> where Id : Identity
{
    graph: &'a Graph<Id, Cost>,
    queue: Vec<&'a Id>,
    set: IntSet::<Id>,
}

impl<'a, Id, Cost> DepthFirstIter<'a, Id, Cost> where Id : Identity
{
    pub fn new(id : &'a Id, graph: &'a Graph<Id, Cost>) -> DepthFirstIter<'a, Id, Cost> {
        let mut queue = Vec::<&'a Id>::new();
        match graph.nodes.contains_key(id) {
            | true => queue.push(id),
            | false => ()
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(*id);
        DepthFirstIter {graph, queue, set}
    }
}

impl<'a, Id, Cost> Iterator for DepthFirstIter<'a, Id, Cost> where Id : Identity
{
    type Item = &'a Id;

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