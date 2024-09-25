use crate::graph::Graph;

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet, IsEnabled};
use std::hash::Hash;
use std::collections::HashSet;

impl<'a, ID, COST> Graph<ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
COST : Clone + Copy
{ 
    pub fn bf_iter(&'a self, id: &'a ID) -> BreadthFirstIter<ID, COST> {
        BreadthFirstIter::<ID, COST>::new(id, self)
    }
}

pub struct BreadthFirstIter<'a, ID, COST>
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
{
    graph: &'a Graph<ID, COST>,
    queue: Queue<&'a ID>,
    set: IntSet::<ID>,
}

impl<'a, ID, COST> BreadthFirstIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>) -> BreadthFirstIter<'a, ID, COST> {
        let mut queue : Queue<&'a ID> = queue![];
        queue.add(id).expect("Failed to construct queue");
        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);
        BreadthFirstIter {graph, queue, set}
    }
}

impl<'a, ID, COST> Iterator for BreadthFirstIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
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