
use std::io::BufferedReader;
use std::io::File;
use std::collections::HashSet;
use std::hash::Hash;
use std::rand;
use std::num;

#[deriving(Eq,PartialEq,Hash,Show,Clone)]
struct Node {
    id: uint,
    adjacent: Vec<uint>,
}

impl Node {
    fn new(id: uint) -> Node {
        Node { id: id, adjacent: Vec::new() }
    }
    fn new_with_vec(id: uint, adjacent: Vec<uint>) -> Node {
        Node { id: id, adjacent: adjacent }
    }
    fn new_with_slice(id: uint, adjacent: &[uint]) -> Node {
        let mut adj: Vec<uint> = Vec::new();
        adj.push_all(adjacent);
        Node { id: id, adjacent: adj }
    }
}


#[deriving(Eq, PartialEq, Hash, Show, Clone)]
struct Edge {
    tail: uint,
    head: uint,
}

impl Edge {
    fn new (tail: uint, head: uint) -> Edge {
        if tail < head { Edge { tail: tail, head: head } }
        else { Edge { tail: head, head: tail } }
    }
}

fn load_data(filename: &str) -> Vec<String> {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    lines
}

fn build_graph(data: Vec<String>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();

    for line in data.iter() {
        let nums: Vec<uint> = line.as_slice().trim().split('\t').map(|x| from_str(x)).map(|x| x.unwrap()).collect();
        nodes.push(Node::new_with_slice(nums[0], nums.slice(1, nums.len())));
    }

    nodes
}

fn build_edges(nodes: Vec<Node>) -> Vec<Edge> {
    let mut edge_set: HashSet<Edge> = HashSet::new();

    for node in nodes.iter() {
        let from = node.id;
        for to in node.adjacent.iter() {
            edge_set.insert(Edge::new(from, (*to)));
        }
    }
    let mut edges: Vec<Edge> = Vec::new();

    for edge in edge_set.iter() {
        edges.push((*edge));
    }
    edges
}

fn unique_nodes(edges: &[Edge]) -> uint {
    let mut node_ids: HashSet<uint> = HashSet::new();

    for edge in edges.iter() {
        node_ids.insert(edge.tail);
        node_ids.insert(edge.head);
    }

    node_ids.len()
}
fn merge_node(from: uint, to: uint, edge: Edge) -> Edge {
    Edge { tail: if edge.tail == from { to } else { edge.tail },
           head: if edge.head == from { to } else { edge.head } }
}

fn is_loop(edge: Edge) -> bool {
    edge.tail == edge.head
}

fn remove_loops(edges: Vec<Edge>) -> Vec<Edge> {
    let mut result: Vec<Edge> = Vec::new();

    for edge in edges.iter() {
        if is_loop((*edge)) {
            result.push( Edge { tail: edge.tail, head: edge.head } )
        }
    }

    result
}

fn choose_edge(edges: &[Edge]) -> (uint,uint) {
    let x = rand::random::<f64>();
    let idx = ((x * (edges.len() as f64)) as uint);
    (edges[idx].tail, edges[idx].head)
}

fn min_cut(edges: Vec<Edge>) -> uint {

    let nodes = unique_nodes(edges.as_slice());
    if nodes <= 2 {
        return edges.clone().len();
    } else {
        let (tail,head) = choose_edge(edges.as_slice());
        let mut next: Vec<Edge> = Vec::new();
        for edge in edges.iter() {
            let new_edge = merge_node(head, tail, *edge);
            if !is_loop(new_edge) { next.push(new_edge) }
        }
        return min_cut(next);
    }
}

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() } else { "QuickSort.txt" };

    let data = load_data(filename);

    let nodes = build_graph(data);

    println!("Nodes: {}", nodes.len());

    let edges = build_edges(nodes);

    let mut runs: Vec<uint> = Vec::new();
    let mut min = edges.len();

    for i in range(1u,100u) {
        runs.push(min_cut(edges.clone()));
        println!("Run: {}, Cut: {}", i, runs[i-1]);
        if runs[i-1] < min { min = runs[i-1] }
    }

    println!("Min Cut: {}", min);
}
