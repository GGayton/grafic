use crate::types::{ Scalar, Identity };
use crate::graph::Graph;

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;


impl<'a, Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{
    /// Returns a breadth first iterator that iterates the entire graph once in a breadth first manner.
    /// Will return an empty iterator if graph does not contain node of id
    pub fn bf_into_iter(&'a self, id: &'a Id) -> BreadthFirstIntoIter<Id, Cost> {
        BreadthFirstIntoIter::<Id, Cost>::new(id, self)
    }
}

pub struct BreadthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    graph: &'a Graph<Id, Cost>,
    queue: Queue<Id>,
    set: IntSet::<Id>
}

impl<'a, Id, Cost> BreadthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    pub fn new(id : &'a Id, graph: &'a Graph<Id, Cost>) -> BreadthFirstIntoIter<'a, Id, Cost> {
        let mut queue : Queue<Id> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(*id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(*id);

        BreadthFirstIntoIter {graph, queue, set}
    }
}

impl<'a, Id, Cost> Iterator for BreadthFirstIntoIter<'a, Id, Cost> where Id : Identity
{
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.remove().ok();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|id| match self.set.contains(id) {
                 false => {
                    _ = self.set.insert(id.clone());
                    self.queue.add(id.clone()).expect("Failed to add to queue");
                },
                 true => () }),
            None => ()
        };

        return next;        
    }

} 