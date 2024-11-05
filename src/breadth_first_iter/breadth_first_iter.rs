use crate::graph::Graph;
use crate::types::{Identity, Scalar};

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 
    pub fn bf_iter(&'a self, id: &'a Id) -> BreadthFirstIter<Id, Cost> {
        BreadthFirstIter::<Id, Cost>::new(id, self)
    }
}

pub struct BreadthFirstIter<'a, Id, Cost> where Id : Identity
{
    graph: &'a Graph<Id, Cost>,
    queue: Queue<&'a Id>,
    set: IntSet::<Id>,
}

impl<'a, Id, Cost> BreadthFirstIter<'a, Id, Cost> where Id : Identity
{
    pub fn new(id : &'a Id, graph: &'a Graph<Id, Cost>) -> BreadthFirstIter<'a, Id, Cost> {

        let mut queue : Queue<&'a Id> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(*id);
        
        BreadthFirstIter {graph, queue, set}
    }
}

impl<'a, Id, Cost> Iterator for BreadthFirstIter<'a, Id, Cost> where Id :Identity
{
    type Item = &'a Id;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.remove().ok();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|id| match self.set.contains(id) {
                 false => {
                    _ = self.set.insert(id.clone());
                    self.queue.add(id).expect("Failed to add to queue");
                },
                 true => () }),
            None => ()
        };

        next
    }

} 