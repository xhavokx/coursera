use std::collections::HashSet;

use graph::graph::Graph;
use graph::node::Node;


#[deriving(Show)]
pub enum Direction {
    Forward,
    Reverse,
}

pub struct DFSIterator<'a> {
    graph: &'a Graph,
    visited: HashSet<uint>,
    to_visit: Vec<uint>,
    direction: Direction,
}

impl<'a> DFSIterator<'a> {

    pub fn new(graph: &'a Graph) -> DFSIterator<'a> {
        DFSIterator { graph: graph, visited: HashSet::new(), to_visit: Vec::new(), direction: Forward }
    }
    pub fn get_visited(&self) -> HashSet<uint> {
        self.visited.clone()
    }
    pub fn starting_at(&mut self, id: uint) -> DFSIterator<'a> {
        self.to_visit.clear();
        self.to_visit.push(id);
        self.clone()
    }
    pub fn having_visited(&mut self, visited: HashSet<uint>) -> DFSIterator<'a> {
        self.visited = visited.clone();
        self.clone()
    }
    pub fn in_reverse(&self) -> DFSIterator<'a> {
        let mut reverse = self.clone();
        reverse.direction = match self.direction { Forward => Reverse, Reverse => Forward };
        reverse
    }
}

impl<'a> Clone for DFSIterator<'a> {
    fn clone(&self) -> DFSIterator<'a> {
        DFSIterator { graph: self.graph, visited: self.visited.clone(), to_visit: self.to_visit.clone(), direction: self.direction }
    }
}

impl<'a> Iterator<&'a Node> for DFSIterator<'a> {
    fn next(&mut self) -> Option<&'a Node> {
        let ref mut visited = self.visited;
        let ref mut to_visit = self.to_visit;
        let ref graph = self.graph;
        let ref direction = self.direction;
        loop {
            match to_visit.as_slice() {
                [] => return None,
                _ => {
                    let maybe_node = to_visit.pop().and_then(|id| graph.find(&id));
                    match maybe_node {
                        None => return None,
                        Some(node) => {
                            visited.insert(node.id);
                            let edges: &Vec<(uint,uint)> = match *direction { Forward => &node.outgoing, Reverse => &node.incoming };
                            let next: Vec<uint> = edges.iter().rev()
                                .map(|e| { let (n,_) = *e; n })
                                .filter(|id| !visited.contains(id))
                                .collect();
                            match next.as_slice() {
                                [] => return Some(node),
                                slice @ _ => { to_visit.push(node.id); to_visit.push_all(slice); continue }
                            }
                        }
                    }
                }
            }
        }
    }
}
