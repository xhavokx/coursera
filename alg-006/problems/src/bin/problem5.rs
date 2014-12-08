
extern crate problems;

use std::collections::HashMap;

use problems::graph::graph::Graph;

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let data = Graph::from_file(filename);
    println!("Vertices: {}", data.num_vertices());
    println!("Edges: {}", data.num_edges());

    let mut paths: HashMap<uint, Option<uint>> = HashMap::new();

    let source = 1u;
    for destination in range(0,200) {
        let destination = destination + 1;
        let length = data.calculate_shortest_path(source, destination);
        paths.insert(destination, length);
        println!("{} -> {}: {}", source, destination, length);
    }

    let vertices = vec![7u,37u,59u,82u,99u,115u,133u,165u,188u,197u];
    for i in vertices.iter() {
        print!("{},", paths.find(i).map_or(None, |x| *x).map_or(1000000u, |x| x))
    }
    println!("");
}
