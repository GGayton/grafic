use crate::graph::Graph;
use crate::types::{Identity, Scalar};

use nohash_hasher::{BuildNoHashHasher, IntSet};
use std::collections::HashSet;

impl<'a, ID, COST> Graph<ID, COST> where ID : Identity, COST : Scalar
{ 
    pub fn df_into_iter(&'a self, id: &'a ID) -> DepthFirstIntoIter<ID, COST> {
        DepthFirstIntoIter::<ID, COST>::new(id, self)
    }
}

pub struct DepthFirstIntoIter<'a, ID, COST> where ID : Identity
{
    graph: &'a Graph<ID, COST>,
    queue: Vec<ID>,
    set: IntSet::<ID>,
}

impl<'a, ID, COST> DepthFirstIntoIter<'a, ID, COST> where ID : Identity
{
    pub fn new(id : &'a ID, graph: &'a Graph<ID, COST>) -> DepthFirstIntoIter<'a, ID, COST> {
        let mut queue = Vec::<ID>::new();
        match graph.nodes.contains_key(id) {
            | true => queue.push(*id),
            | false => ()
        };

        let mut set = HashSet::<ID, BuildNoHashHasher<ID>>::with_capacity_and_hasher(graph.nodes.len(), BuildNoHashHasher::<ID>::default());
        set.insert(*id);

        DepthFirstIntoIter {graph, queue, set}
    }
}

impl<'a, ID, COST> Iterator for DepthFirstIntoIter<'a, ID, COST> where ID : Identity
{
    type Item = ID;

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