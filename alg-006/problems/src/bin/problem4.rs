extern crate problems;

use problems::graph::Graph;

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let data = Graph::load_from_file(filename);

    let mut scc = data.calculate_scc();
    let count = scc.len();

    scc.slice_mut(0, count).sort_by(|a,b| b.len().cmp(&(a.len())));

    for scc in scc.iter().take(5) {
        let mut nodes = scc.clone();
        nodes.slice_mut(0, scc.len()).sort();
        println!("SCC len {}: {}", scc.len(), nodes);
    }

    println!("{},{},{},{},{}",
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()),
             scc.remove(0).map_or(0, |x| x.len()));
}
