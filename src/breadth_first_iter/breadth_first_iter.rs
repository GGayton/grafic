use crate::graph::Graph;
use crate::types::{Identity, Scalar};

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, ID, COST> Graph<ID, COST> where ID : Identity, COST : Scalar
{ 
    pub fn bf_iter(&'a self, id: &'a ID) -> BreadthFirstIter<ID, COST> {
        BreadthFirstIter::<ID, COST>::new(id, self)
    }
}

pub struct BreadthFirstIter<'a, ID, COST> where ID : Identity
{
    graph: &'a Graph<ID, COST>,
    queue: Queue<&'a ID>,
    set: IntSet::<ID>,
}

impl<'a, ID, COST> BreadthFirstIter<'a, ID, COST> where ID : Identity
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>) -> BreadthFirstIter<'a, ID, COST> {

        let mut queue : Queue<&'a ID> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);
        
        BreadthFirstIter {graph, queue, set}
    }
}

impl<'a, ID, COST> Iterator for BreadthFirstIter<'a, ID, COST> where ID :Identity
{
    type Item = &'a ID;

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