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
    /// If inclusive, the search starts after the node (and will always return non-empty if the node exists), 
    /// otherwise the search starts on the node (and can therefore return empty).
    pub fn bf_search<SEARCH : FnMut(Id) -> bool>(&'a self, id: Id, search_fn: SEARCH, inclusive : bool) -> BreadthFirstSearch<Id, Cost, SEARCH> {
        BreadthFirstSearch::<Id, Cost, SEARCH>::new(id, self, search_fn, inclusive)
    }
}

pub struct BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(Id) -> bool
{
    graph: &'a Graph<Id, Cost>,
    queue: Queue<Id>,
    set: IntSet::<Id>,
    search_fn: SEARCH
}

impl<'a, Id, Cost, SEARCH> BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(Id) -> bool
{
    pub fn new(id : Id, graph: &'a Graph<Id, Cost>, mut search_fn: SEARCH, inclusive : bool) -> BreadthFirstSearch<'a, Id, Cost, SEARCH> {

        let queue : Queue<Id> = match graph.nodes.contains_key(&id) {
            | true if inclusive => queue![id],
            | true if search_fn(id) => queue![id],
            | true if !inclusive => queue![],
            | false => queue![],
            | _ => panic!("Should not be able to read here")
        };

        let mut set = HashSet::<Id, BuildNoHashHasher<Id>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<Id>::default());
        set.insert(id);

        BreadthFirstSearch {graph, queue, set, search_fn}
    }
}

impl<'a, Id, Cost, SEARCH> Iterator for BreadthFirstSearch<'a, Id, Cost, SEARCH> where Id : Identity, SEARCH : FnMut(Id) -> bool
{
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {

        // while queue not empty
        let next = self.queue.remove().ok();

        // take next item in queue and iterate
        match next {
            Some(id) => self.graph.nodes[&id].neighbours().for_each(|&id| 
                if !self.set.contains(&id) {
                    _ = self.set.insert(id);
                    if (self.search_fn)(id) { self.queue.add(id).expect("Failed to add to queue");}
                }),
            None => ()
        };

        next
    }

} 