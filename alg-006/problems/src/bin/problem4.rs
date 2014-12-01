extern crate problems;

use problems::graph::graph::Graph;

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let data = Graph::from_file(filename);
    println!("Vertices: {}", data.num_vertices());
    println!("Edges: {}", data.num_edges());

    let mut scc = data.calculate_scc();

//    for scc in scc.iter().take(5) {
//        let mut nodes = scc.clone();
//        nodes.slice_mut(0, scc.len()).sort();
//        println!("SCC len {}: {}", scc.len(), nodes);
//    }

    let mut scc_count = 0u;
    for scc in scc.iter() {
        scc_count = scc_count + scc.len()
    }
    println!("Sum SCC len: {}", scc_count);

    println!("{},{},{},{},{} | {},{},{},{},{}",
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()));

        
}
