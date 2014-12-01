use std::io::BufferedReader;
use std::io::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::fmt::Show;
use std::fmt;

pub struct Edge {
    pub tail: uint,
    pub head: uint,
    pub weight: uint,
}

impl Edge {
    pub fn new(tail: uint, head: uint) -> Edge {
        Edge { tail: tail, head: head, weight: 1u }
    }

    pub fn with_weight(tail: uint, head: uint, weight: uint) -> Edge {
        Edge { tail: tail, head: head, weight: weight }
    }
}

impl Clone for Edge {
    fn clone(&self) -> Edge {
        Edge { tail: self.tail, head: self.head, weight: self.weight }
    }
}

impl Show for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge ( tail: {}, head: {}, weight: {} )", self.tail, self.head, self.weight)
    }
}
