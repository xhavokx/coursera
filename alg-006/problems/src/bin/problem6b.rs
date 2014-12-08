extern crate problems;

use std::io::BufferedReader;
use std::io::File;

use problems::heap;

fn from_file(filename: &str) -> Vec<int> {
    let path = Path::new(filename);
    let mut file = BufferedReader::new(File::open(&path));
    let lines: Vec<int> = file.lines()
        .map(|x| x.unwrap())
        .map(|x| from_str(x.as_slice().trim()))
        .map(|x| x.unwrap())
        .collect();

    lines
}

struct Median {
    lo: Vec<int>,
    hi: Vec<int>,
}

impl Median {
    fn new() -> Median {
        Median { lo: Vec::new(), hi: Vec::new() }
    }

    fn insert_lo(&mut self, num: int) {
        self.lo.push(num);
        heap::push_heap(&mut self.lo, |a,b| b.cmp(a));
    }

    fn insert_hi(&mut self, num: int) {
        self.hi.push(num);
        heap::push_heap(&mut self.hi, |a,b| a.cmp(b));
    }

    fn rebalance_lo(&mut self) -> bool {
        if self.lo.len() > self.hi.len() && self.lo.len() - self.hi.len() > 1 {
            heap::pop_heap(&mut self.lo, |a,b| b.cmp(a));
            let i = self.lo.pop().unwrap();
            self.hi.push(i);
            heap::push_heap(&mut self.hi, |a,b| a.cmp(b));
            return true
        }
        return false
    }

    fn rebalance_hi(&mut self) -> bool {
        if self.hi.len() > self.lo.len() && self.hi.len() - self.lo.len() > 1 {
            heap::pop_heap(&mut self.hi, |a,b| a.cmp(b));
            let i = self.hi.pop().unwrap();
            self.lo.push(i);
            heap::push_heap(&mut self.lo, |a,b| b.cmp(a));
            return true
        }
        return false
    }

    fn insert(&mut self, num: int) {
        if self.lo.len() == 0 || num < self.lo[0] {
            self.insert_lo(num);
        } else if self.hi.len() == 0 || num > self.hi[0] {
            self.insert_hi(num);
        } else if self.lo.len() >= self.hi.len() {
            self.insert_lo(num);
        } else {
            self.insert_hi(num);
        }

        if !self.rebalance_lo() {
            self.rebalance_hi();
        }
    }

    fn median(&self) -> int {
        let k = self.lo.len() + self.hi.len();
        
        let i = if k % 2 == 0 { k/2 } else { (k+1)/2 };

        if i == self.lo.len() {
            return self.lo[0]
        } else if i == self.lo.len() + 1 {
            return self.hi[0]
        } else {
            fail!("BAD THINGS");
        }
    }
}

fn main() {
    let args = std::os::args();
    let filename = if args.len() > 1 { args[1].as_slice() }
                   else { println!("Usage: {} <input file>", args[0]); return };

    let mut data = from_file(filename);

    let mut m = Median::new();
    let mut medians: Vec<int> = Vec::new();;

    for num in data.iter().enumerate() {
        let (i, num) = num;
        m.insert(*num);
        medians.push(m.median());
    }
//    println!("Medians: {}", medians);
    let sum = medians.iter().fold(0, |a,b| a + *b);
    println!("Sum: {}", sum);
    println!("Result: {}", sum % 10000);
}
