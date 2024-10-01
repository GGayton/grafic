use crate::graph::Graph;

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet, IsEnabled};
use std::hash::Hash;
use std::collections::HashSet;

type SearchFn<ID> = fn(&ID) -> bool;

impl<'a, ID, COST> Graph<ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
COST : Clone + Copy
{ 
    pub fn bf_search(&'a self, id: &'a ID, search_fn: SearchFn<ID>) -> BreadthFirstSearch<ID, COST> {
        BreadthFirstSearch::<ID, COST>::new(id, self, search_fn)
    }
}

pub struct BreadthFirstSearch<'a, ID, COST>
where 
ID : Eq + PartialEq + Hash + Clone + Copy,
{
    graph: &'a Graph<ID, COST>,
    queue: Queue<&'a ID>,
    set: IntSet::<ID>,
    search_fn: SearchFn<ID>
}

impl<'a, ID, COST> BreadthFirstSearch<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>, search_fn: SearchFn<ID>) -> BreadthFirstSearch<'a, ID, COST> {
        let mut queue : Queue<&'a ID> = queue![];
        queue.add(id).expect("Failed to construct queue");
        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);
        BreadthFirstSearch {graph, queue, set, search_fn}
    }
}

impl<'a, ID, COST> Iterator for BreadthFirstSearch<'a, ID, COST> 
where 
ID : Eq + PartialEq + Hash + Clone + Copy + IsEnabled,
{
    type Item = &'a ID;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.remove().ok();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|id| 
                if !self.set.contains(id) {
                    _ = self.set.insert(id.clone());
                    if (self.search_fn)(&id) { self.queue.add(id).expect("Failed to add to queue");}
                }),
            None => ()
        };

        next
    }

} 