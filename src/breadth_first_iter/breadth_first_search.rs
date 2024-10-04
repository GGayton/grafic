use crate::types::Scalar;
use crate::graph::Graph;
use crate::types::Identity;

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, ID, COST> Graph<ID, COST> where ID : Identity, COST : Scalar
{ 
    pub fn bf_search<SEARCH : FnMut(&ID) -> bool>(&'a self, id: &'a ID, search_fn: SEARCH) -> BreadthFirstSearch<ID, COST, SEARCH> {
        BreadthFirstSearch::<ID, COST, SEARCH>::new(id, self, search_fn)
    }
}

pub struct BreadthFirstSearch<'a, ID, COST, SEARCH> where ID : Identity, SEARCH : FnMut(&ID) -> bool
{
    graph: &'a Graph<ID, COST>,
    queue: Queue<&'a ID>,
    set: IntSet::<ID>,
    search_fn: SEARCH
}

impl<'a, ID, COST, SEARCH> BreadthFirstSearch<'a, ID, COST, SEARCH> where ID : Identity, SEARCH : FnMut(&ID) -> bool
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>, search_fn: SEARCH) -> BreadthFirstSearch<'a, ID, COST, SEARCH> {
        let mut queue : Queue<&'a ID> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);

        BreadthFirstSearch {graph, queue, set, search_fn}
    }
}

impl<'a, ID, COST, SEARCH> Iterator for BreadthFirstSearch<'a, ID, COST, SEARCH> where ID : Identity, SEARCH : FnMut(&ID) -> bool
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