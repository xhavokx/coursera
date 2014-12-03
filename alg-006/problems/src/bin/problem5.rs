
extern crate problems;

use problems::graph::graph::Graph;

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let data = Graph::from_file(filename);
    println!("Vertices: {}", data.num_vertices());
    println!("Edges: {}", data.num_edges());
}
