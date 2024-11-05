use crate::types::Scalar;
use crate::graph::Graph;
use crate::types::Identity;

use queues::*;
use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

/*
    Breadth-first search 

    A breadth-first iterator through the graph, starting at a source node.
    The search is dictated by the SEARCH closure, which can cut some searches
    off early.

*/

impl<'a, Id, Cost> Graph<Id, Cost> where Id : Identity, Cost : Scalar
{ 
    pub fn bf_search<SEARCH : FnMut(&Id) -> bool>(&'a self, id: &'a Id, search_fn: SEARCH) -> BreadthFirstSearch<Id, Cost, SEARCH> {
        BreadthFirstSearch::<Id, Cost, SEARCH>::new(id, self, search_fn)
    }
}

pub struct BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(&Id) -> bool
{
    graph: &'a Graph<Id, Cost>,
    queue: Queue<&'a Id>,
    set: IntSet::<Id>,
    search_fn: SEARCH
}

impl<'a, Id, Cost, SEARCH> BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(&Id) -> bool
{
    pub fn new(id : &'a Id, graph: &'a Graph<Id, Cost>, search_fn: SEARCH) -> BreadthFirstSearch<'a, Id, Cost, SEARCH> {
        let mut queue : Queue<&'a Id> = queue![];
        match graph.nodes.contains_key(id) {
            | true => queue.add(id).expect("Failed to construct queue"),
            | false => None
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(*id);

        BreadthFirstSearch {graph, queue, set, search_fn}
    }
}

impl<'a, Id, Cost, SEARCH> Iterator for BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(&Id) -> bool
{
    type Item = &'a Id;

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