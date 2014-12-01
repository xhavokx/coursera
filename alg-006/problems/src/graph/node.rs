use std::fmt::Formatter;
use std::fmt::Show;
use std::fmt;

pub struct Node {
    pub id: uint,
    pub outgoing: Vec<(uint, uint)>,
    pub incoming: Vec<(uint, uint)>,
}

impl Node {
    pub fn new(id: uint) -> Node {
        Node{ id: id, outgoing: Vec::new(), incoming: Vec::new() }
    }

    pub fn add(&mut self, edge: (uint,uint,uint)) {
        match edge {
            (tail, head, weight) if tail == self.id => self.outgoing.push((head,weight)),
            (tail, head, weight) if head == self.id => self.incoming.push((tail,weight)),
            _ => ()       
        }
    }
    pub fn dedup_edges(&mut self) {
        self.outgoing.sort();
        self.outgoing.dedup();

        self.incoming.sort();
        self.incoming.dedup();
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node ( id: {}, outgoing: {}, incoming: {} )", self.id, self.outgoing, self.incoming)
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        Node { id: self.id, outgoing: self.outgoing.clone(), incoming: self.incoming.clone() }
    }
}
