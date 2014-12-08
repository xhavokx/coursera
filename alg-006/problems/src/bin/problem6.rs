use std::io::BufferedReader;
use std::io::File;
use std::collections::HashSet;

fn from_file(filename: &str) -> HashSet<int> {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let lines: HashSet<int> = file.lines()
        .map(|x| x.unwrap())
        .map(|x| from_str(x.as_slice().trim()))
        .map(|x| x.unwrap())
        .collect();

    lines
}

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let nums = from_file(filename);

    let mut result: HashSet<int> = HashSet::new();

    for i in range(-10000,10001) {
        for x in nums.iter() {
            let x = *x;
            let y = i - x;
            if x != y && nums.contains(&y) {
                result.insert(i);
                println!("Found {}", i);
                break;
            }
        }
    }

    println!("Result: {}", result);
    println!("Count: {}", result.len());
}
