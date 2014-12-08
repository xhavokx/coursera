use std::io::BufferedReader;
use std::io::File;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::fmt::Show;
use std::fmt;

use graph::node::Node;
use heap;

pub struct Graph {
    nodes: HashMap<uint, Node>,
    edges: Vec<(uint,uint,uint)>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph { nodes: HashMap::new(), edges: Vec::new() }
    }

    pub fn num_vertices(&self) -> uint {
        self.nodes.len()
    }
    pub fn num_edges(&self) -> uint {
        self.edges.len()
    }

    fn edges_from_string(line: Option<String>) -> Vec<(uint,uint,uint)> {
        match line {
            Some(string) => {
                let nums: Vec<(uint,uint)> = string.as_slice().trim().words().map(|x| x)
                    .map(|x| x.trim().split(',').map(|s| from_str(s.trim())).map(|x| x.unwrap()).collect::<Vec<uint>>())
                    .map(|x| match x.as_slice() {
                        [v] => Some((v,1)),
                        [v,w] => Some((v,w)),
                        _ => None })
                    .filter(|x| x.is_some())
                    .map(|x| x.unwrap())
                    .collect();
                match nums.as_slice() {
                    [(tail,_), heads..] => return heads.iter().map(|x| { let (h,w) = *x; (tail,h,w) }).collect::<Vec<(uint,uint,uint)>>(),
                    _ => Vec::<(uint,uint,uint)>::new()
                }
            },
            None => return Vec::<(uint,uint,uint)>::new(),
        }
    }

    fn insert_if_not_exist(&mut self, id: uint) {
        let ref mut nodes = self.nodes;
        match nodes.find(&id) {
            None => { nodes.insert(id, Node::new(id)); () },
            _ => ()
        }
    }

    fn add_edge(&mut self, edge: (uint,uint,uint)) {
        self.edges.push(edge);

        let (tail, head, _) = edge;

        self.insert_if_not_exist(tail);
        self.insert_if_not_exist(head);

        let ref mut nodes = self.nodes;
        nodes.find_mut(&tail).unwrap().add(edge);
        nodes.find_mut(&head).unwrap().add(edge);

        ()
    }

    pub fn from_file(filename: &str) -> Graph {
        let path = Path::new(filename);
        let mut file = BufferedReader::new(File::open(&path));

        let mut n: Vec<(uint,uint,uint)> = Vec::new();
        for e in file.lines().map(|s| Graph::edges_from_string(s.ok())) {
            n.push_all(e.as_slice());
        }

        let mut graph = Graph::new();
        for e in n.iter() {
            graph.add_edge(*e);
            ()
        }
        for (_, node) in graph.nodes.iter_mut() {
            node.dedup_edges()
        }

        graph
    }

#[allow(dead_code)]
    fn dfs_iter(&self, i: uint, visited: &mut HashSet<uint>, on_enter: |n:uint|, on_leave: |n:uint|, forward: bool) {
        let get_edges: |uint| -> Vec<uint> = |id| { self.nodes.find(&id).map_or(Vec::new(), |n| if forward { n.outgoing.clone() } else { n.incoming.clone() }.iter().rev().map(|x| { let (v,_) = *x; v} ).collect()) };
        let mut stack: Vec<uint> = vec![i];
        let mut edges: HashMap<uint,Vec<uint>> = HashMap::new();
        edges.insert(i, get_edges(i));
        on_enter(i);
        visited.insert(i);

        while !stack.is_empty() {
            let i = stack[stack.len() - 1];
            if edges.find(&i).is_none() {
                on_leave(i);
                stack.pop();
                continue;
            }
            let oj = edges.find_mut(&i).unwrap().pop();
            match oj {
                Some(j) => {
                    if !visited.contains(&j) {
                        stack.push(j);
                        edges.insert(j, get_edges(j));
                        on_enter(j);
                        visited.insert(j);
                    }
                },
                None => {
                    edges.remove(&i);
                    ()
                },
            }
        }
    }

#[allow(dead_code)]
    fn dfs_recur(&self, i: uint, visited: &mut HashSet<uint>, on_enter: |n:uint|, on_leave: |n:uint|, forward: bool) {
        visited.insert(i);
        on_enter(i);
        for e in self.nodes.find(&i).map_or(Vec::new(), |n| if forward { n.outgoing.clone() } else { n.incoming.clone() } ).iter() {
            let (j,_) = *e;
            if !visited.contains(&j) {
                self.dfs_recur(j, visited, |n| on_enter(n), |n| on_leave(n), forward)
            }
        }
        on_leave(i);
    }

    fn calculate_finish_times(&self) -> Vec<uint> {
        let mut t: uint = 0u;
        let mut visited: HashSet<uint> = HashSet::new();
        let mut keys: Vec<uint> = self.nodes.keys().map(|x| *x).collect();
        keys.as_mut_slice().sort();
        
        let mut finish: Vec<uint> = Vec::from_fn(keys.len()+1, |_| 0);

        for i in keys.iter().rev() {
            if !visited.contains(i) {
                self.dfs_iter(*i, &mut visited, |_| {}, |n| { t = t + 1; finish[n] = t; }, false);
            }
        }
        println!("# finish: {}", finish.len());
        finish
    }

    fn calculate_leader(&self, finish: Vec<uint>) -> HashMap<uint,Vec<uint>> {
        let mut leader: HashMap<uint,Vec<uint>> = HashMap::new();
        let mut visited: HashSet<uint> = HashSet::new();
        let mut fin: Vec<(uint,&uint)> = finish.iter().enumerate().collect();
        fin.as_mut_slice().sort_by(|x,y| { let (_,a) = *x; let (_,b) = *y; b.cmp(a) });
        for i in fin.iter().map(|x| { let (n,_) = *x; n} ).filter(|x| *x != 0u) {
            if !visited.contains(&i) {
                self.dfs_iter(i, &mut visited, |n| { if !leader.contains_key(&i) { leader.insert(i, Vec::new()); () }
                                                      leader.find_mut(&i).map(|v| v.push(n)); () }, |_| {}, true);
            }
        }
        println!("# leader: {}", leader.len());
        leader
    }

    pub fn calculate_scc(&self) -> Vec<Vec<uint>> {
        let finish: Vec<uint> = self.calculate_finish_times();
        let leader: HashMap<uint,Vec<uint>> = self.calculate_leader(finish);
        
        let mut scc: Vec<Vec<uint>> = Vec::new();

        for v in leader.values() {
            let mut component = v.clone();
            component.as_mut_slice().sort();
            component.dedup();
            scc.push(component);
        }
        scc.as_mut_slice().sort_by(|a,b| b.len().cmp(&(a.len())));
        scc
    }
    pub fn find(&self, id: &uint) -> Option<&Node> {
        self.nodes.find(id)
    }

    fn cmp(a: &(uint,Option<uint>), b:&(uint,Option<uint>)) -> Ordering {
        let (_,a) = *a;
        let (_,b) = *b;
        if a.is_none() {
            return Greater
        } else if b.is_none() {
            return Less
        } else {
            let a = a.unwrap();
            let b = b.unwrap();
            return a.cmp(&b);
        }
    }

    fn do_update(edges: Vec<(uint,uint)>, length: uint, to_visit: &mut Vec<(uint,Option<uint>)>) {
        let mut edge_map: HashMap<uint,(uint,Option<uint>)> = HashMap::new();
        for edge in edges.iter() {
            let (head, weight) = *edge;
            edge_map.insert(head, (head, Some(length + weight)));
        }

        for i in range(0, to_visit.len()) {
            let (head, _) = (*to_visit)[i];
            match edge_map.find(&head) {
                Some(weight) => {
//                    println!("   matched {}", head);
                    match Graph::cmp(&(*to_visit)[i], weight) {
                        Greater => {
                            (*to_visit)[i] = *weight
                        },
                        _ => {},
                    }
                },
                None => {}
            }
        }
    }

    pub fn calculate_shortest_path(&self, source: uint, destination: uint) -> Option<uint> {
        if source == destination { return Some(0u) }
        let ref nodes = self.nodes;

        let cmp = |a:&(uint,Option<uint>),b:&(uint,Option<uint>)| {
            let (_,a) = *a;
            let (_,b) = *b;
            if a.is_none() {
                return Greater
            } else if b.is_none() {
                return Less
            } else {
                let a = a.unwrap();
                let b = b.unwrap();
                return a.cmp(&b);
            }
        };

        let mut visited: HashMap<uint,uint> = HashMap::new();

        visited.insert(source, 0u);

        let mut to_visit: Vec<(uint,Option<uint>)> = Vec::new();
        for n in nodes.keys() {
            to_visit.push((*n, None));
        }

        Graph::do_update(nodes.find(&source).map_or(Vec::new(), |x| x.outgoing.clone()), 0u, &mut to_visit);

        heap::make_heap(&mut to_visit, |a,b| cmp(a,b));

//        println!("Initial: {}", to_visit);

        while !to_visit.is_empty() && !visited.contains_key(&destination) {
            heap::pop_heap(&mut to_visit, |a,b| cmp(a,b));
            let (head, length) = to_visit.pop().unwrap();
            match length {
                Some(length) => {
                    visited.insert(head, length);
                    Graph::do_update(nodes.find(&head).map_or(Vec::new(), |x| x.outgoing.clone()), length, &mut to_visit);
                },
                None => fail!("Popped unreachable node {}", head),
            }
            heap::make_heap(&mut to_visit, |a,b| cmp(a,b));

//            println!("Intermediate: {}", to_visit);
        }
        visited.find(&destination).map_or(None, |x| Some(*x))
    }
}

impl Show for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph ( Nodes: {} )", self.nodes)
    }    
}
