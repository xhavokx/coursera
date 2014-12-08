extern crate problems;

use std::io::BufferedReader;
use std::io::File;

use problems::sort::quicksort::quick_sort;
use problems::sort::quicksort::pivot_first;
use problems::sort::quicksort::pivot_last;
use problems::sort::quicksort::pivot_median;

fn load_data(filename: &str) -> Vec<int> {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let nums: Vec<int> = file.lines()
        .map(|x| from_str(x.unwrap().as_slice().trim()))
        .map(|x| x.unwrap()).collect();

    nums
}

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() } else { "QuickSort.txt" };
    let data = load_data(filename.as_slice());
    let len = data.len();
    let mut cmps = 0u;

    let mut s_first = data.clone();
    let mut s_last = data.clone();
    let mut s_median = data.clone();

    let mut s = s_first.slice_mut(0, len);
    quick_sort(s, |x:uint| cmps += x, |v| pivot_first(v));
    println!("Pivot: First, Length: {}, Comparisons: {}", len, cmps);
    cmps = 0;

    s = s_last.slice_mut(0, len);
    quick_sort(s, |x:uint| cmps += x, |v| pivot_last(v));
    println!("Pivot: last, Length: {}, Comparisons: {}", len, cmps);
    cmps = 0;

    s = s_median.slice_mut(0, len);
    quick_sort(s, |x:uint| cmps += x, |v| pivot_median(v));
    println!("Pivot: Median, Length: {}, Comparisons: {}", len, cmps);
}
// First: 162085
// Last: 164123
// Median: 138382
