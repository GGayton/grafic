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
    pub fn bf_into_iter(&'a self, id: &'a ID) -> BreadthFirstIntoIter<ID, COST> {
        BreadthFirstIntoIter::<ID, COST>::new(id, self)
    }
}

pub struct BreadthFirstIntoIter<'a, ID, COST>
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
{
    graph: &'a Graph<ID, COST>,
    queue: Queue<ID>,
    set: IntSet::<ID>
}

impl<'a, ID, COST> BreadthFirstIntoIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>) -> BreadthFirstIntoIter<'a, ID, COST> {
        let mut queue : Queue<ID> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(*id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);
        
        BreadthFirstIntoIter {graph, queue, set}
    }
}

impl<'a, ID, COST> Iterator for BreadthFirstIntoIter<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    type Item = ID;

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