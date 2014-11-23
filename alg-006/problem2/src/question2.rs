
use std::io::BufferedReader;
use std::io::File;

fn load_data(filename: &str) -> Vec<int> {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let nums: Vec<int> = file.lines()
        .map(|x| from_str(x.unwrap().as_slice().trim()))
        .map(|x| x.unwrap()).collect();

    nums
}
fn pivot_first(_: &[int]) -> uint {
    0
}

fn pivot_last(v: &[int]) -> uint {
    v.len() - 1
}

fn max (v: &[int], a: uint, b: uint) -> uint {
    if v[a] >= v[b] { return a } else { return b };
}

fn min (v: &[int], a: uint, b: uint) -> uint {
    if v[a] <= v[b] { return a } else { return b };
}

fn pivot_median(v: &[int]) -> uint {
    let len = v.len();

    if len == 1 { return 0 }
    else if len == 2 { return min(v, 0,1) }

    let mid = if (len % 2) == 0 { (len / 2) - 1 } else { len / 2 };

    let median = max(v, min(v, 0,mid), min(v, max(v, 0,mid),len - 1));

//    println!("Len {}, first {} ({}), mid {} ({}), last {} ({}), median {} ({})",
//             len, v[0u], 0u, v[mid], mid, v[len - 1], len - 1, v[median], median);

    median
}

fn partition(v: &mut[int], choose_pivot: |&[int]|->uint) -> uint {
    let len = v.len();
    let pivot_index = choose_pivot(v);

    v.swap(pivot_index, 0);

    let mut store_index = 1;
    for i in range(1, len) {
        if v[i] < v[0] {
            v.swap(i, store_index);
            store_index += 1;
        }
    }

    v.swap(store_index - 1, 0);
    store_index - 1
}

fn quick_sort(v: &mut [int], cmp: |uint|, choose_pivot: |&[int]|->uint) {
    let len = v.len();
    if  len < 2 {
        return
    }
    let pivot_index = partition(v, |v| choose_pivot(v));

    cmp(len - 1);

//    println!("pivot: {}:   {} and {}", v[pivot_index], v.slice(0, pivot_index), v.slice(pivot_index + 1, len));

    quick_sort(v.slice_mut(0, pivot_index), |x| cmp(x), |v| choose_pivot(v));
    quick_sort(v.slice_mut(pivot_index + 1, len), |x| cmp(x), |v| choose_pivot(v));
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
