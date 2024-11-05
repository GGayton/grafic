use crate::graph::Graph;
use crate::types::{Identity, Scalar};

use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 
    pub fn df_into_iter(&'a self, id: &'a Id) -> DepthFirstIntoIter<Id, Cost> {
        DepthFirstIntoIter::<Id, Cost>::new(id, self)
    }
}

pub struct DepthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    graph: &'a Graph<Id, Cost>,
    queue: Vec<Id>,
    set: IntSet::<Id>,
}

impl<'a, Id, Cost> DepthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    pub fn new(id : &'a Id, graph: &'a Graph<Id, Cost>) -> DepthFirstIntoIter<'a, Id, Cost> {
        let mut queue = Vec::<Id>::new();
        match graph.nodes.contains_key(id) {
            | true => queue.push(*id),
            | false => ()
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(*id);

        DepthFirstIntoIter {graph, queue, set}
    }
}

impl<'a, Id, Cost> Iterator for DepthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.pop();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|id| match self.set.contains(id) {
                 false => {
                    _ = self.set.insert(id.clone());
                    self.queue.push(*id);
                },
                 true => () }),
            None => ()
        };

        next
    }

} 