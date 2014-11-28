use std::io::BufferedReader;
use std::io::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cell::RefCell;
use std::fmt::Formatter;
use std::fmt::Show;
use std::fmt;

pub struct Node {
    id: uint,
    outgoing: RefCell<Vec<uint>>,
    incoming: RefCell<Vec<uint>>,
}

impl Node {
    pub fn new(id: uint) -> Node {
        Node { id: id, outgoing: RefCell::new(Vec::new()), incoming: RefCell::new(Vec::new()) }
    }
    
    pub fn add_outgoing(&self, id: uint) {
        if self.id != id { self.outgoing.borrow_mut().push(id) }
    }
    pub fn add_incoming(&self, id: uint) {
        if self.id != id { self.incoming.borrow_mut().push(id) }
    }
    pub fn get_outgoing(&self) -> Vec<uint>{
        self.outgoing.borrow().clone()
    }
    pub fn get_incoming(&self) -> Vec<uint> {
        self.incoming.borrow().clone()
    }
    pub fn get_id(&self) -> uint {
        self.id
    }
}

impl Show for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node ( id: {}, outgoing: {}, incoming: {} )", self.get_id(), self.get_outgoing(), self.get_incoming())
    }
}

pub struct Graph {
    edges: RefCell<HashSet<(uint,uint)>>,
    nodes: RefCell<HashMap<uint, Node>>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { edges: RefCell::new(HashSet::new()), nodes: RefCell::new(HashMap::new()) }
    }
    
    pub fn load_from_file(filename: &str) -> Graph {
        let path = Path::new(filename);
        let mut file = BufferedReader::new(File::open(&path));

        let mut graph = Graph::new();

        for line in file.lines().map(|x| x.unwrap()) {
            let nums: Vec<uint> = line.as_slice().trim().split(' ')
                .map(|x| x.trim()).map(|x| from_str(x)).filter(|x| x.is_some()).map(|x| x.unwrap())
                .collect();
            if nums.len() != 2 { fail!("Edge must have 2 vertices {}", line) }
            
            let tail = nums[0];
            let head = nums[1];
            
            graph.add_edge((tail,head));
        }
        graph
    }

    pub fn insert_node(&mut self, id: uint) {
        let mut nodes = self.nodes.borrow_mut();
        match nodes.find(&id) {
            None => { nodes.insert(id, Node::new(id)); () },
            _ => ()
        }
    }
    
    pub fn add_edge(&mut self, edge: (uint,uint)) {
        self.edges.borrow_mut().insert(edge);
        
        let (tail, head) = edge;
        if tail == head { return };
        
        self.insert_node(tail);
        self.insert_node(head);

        self.nodes.borrow_mut().find(&tail).map(|node| node.add_outgoing(head));
        self.nodes.borrow_mut().find(&head).map(|node| node.add_incoming(tail));
    }

    fn dfs_iter(&self, id: &uint, get_edges: |n: Option<&Node>| -> Vec<uint>, visited: &mut HashSet<uint>, post: |id:&uint|) {
        let mut empty = false;
        let mut edges = Vec::<uint>::new();
        edges.push(*id);

        while !empty {
            match edges.pop() {
                Some(edge) => {
                    visited.insert(edge);
                    let new_edges: Vec<uint> = get_edges(self.nodes.borrow().find(&edge))
                        .iter().rev().map(|x| *x).filter(|ne| !visited.contains(ne)).collect();

                    match new_edges.as_slice() {
                        [] => { post(&edge); },
                        _ => { edges.push(edge); edges.push_all(new_edges.as_slice()) },
                    }
                },
                None => empty = true
            }
        }
    }

    pub fn calculate_scc(&self) -> Vec<Vec<uint>> {
        let mut keys: Vec<uint> = self.nodes.borrow().keys().map(|x| *x).collect();
        keys.as_mut_slice().sort_by(|a,b| b.cmp(a));
        
        let mut finish: Vec<uint> = Vec::new();
        let mut leader: HashMap<uint,RefCell<HashSet<uint>>> = HashMap::new();
        let mut visited: HashSet<uint> = HashSet::new();
        
        for id in keys.iter() {
            if !visited.contains(id) {
                visited.insert(*id);
                self.dfs_iter(id, |x| x.map_or(Vec::<uint>::new(), |x| x.get_incoming()), &mut visited, |id| finish.push(*id));
            }
        }

        println!("Finish times: {}", finish);

        visited.clear();
        for id in finish.iter().rev() {
            if !visited.contains(id) {
                visited.insert(*id);
                self.dfs_iter(id, |x| x.map_or(Vec::<uint>::new(), |x| x.get_outgoing()), &mut visited,
                              |n| { if !leader.contains_key(id) { leader.insert(*id, RefCell::new(HashSet::new())); () }
                                    leader.find(id).map(|x| x.borrow_mut().insert(*n)); () });
            }
        }
        let mut scc: Vec<Vec<uint>> = Vec::new();
        for set in leader.values() {
            let mut component = Vec::new();
            for node in set.borrow().iter() {
                component.push(*node);
            }
            component.slice_mut(0, set.borrow().len()).sort();
            scc.push(component);
        }
        scc
    }
}
