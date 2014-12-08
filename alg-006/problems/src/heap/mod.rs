
#[cfg(test)]
mod test {
    use std::fmt::Show;

    use heap;

    #[test]
    fn test_make_heap() {
        let mut v = vec![1u,6u,2u,5u,3u,4u];
        println!("Input: {}", v);
        heap::make_heap(&mut v, |a,b| a.cmp(b));
        println!("Output: {}", v);
        assert!(v.as_slice() == [1u,3u,2u,5u,6u,4u]);
    }
    #[test]
    fn test_pop_heap() {
        let mut v = vec![1u,6u,2u,5u,3u,4u];
        println!("Input: {}", v);
        heap::make_heap(&mut v, |a,b| a.cmp(b));
        println!("Heap: {}", v);

        let mut o: Vec<uint> = Vec::new();
        while !v.is_empty() {
            heap::pop_heap(&mut v, |a,b| a.cmp(b));
            o.push(v.pop().unwrap());
            println!("Pop: {} {}", v, o);
        }
        println!("Output: {}", o);
        assert!(o.as_slice() == [1u,2u,3u,4u,5u,6u]);
    }
    #[test]
    fn test_push_heap() {
        let mut i = vec![1u,6u,2u,5u,3u,4u];
        println!("Input: {}", i);
        let mut v: Vec<uint> = Vec::new();

        heap::make_heap(&mut v, |a,b| a.cmp(b));
        println!("Heap: {}", v);
        for x in i.iter() {
            v.push(*x);
            heap::push_heap(&mut v, |a,b| a.cmp(b));
            println!("Push: {} {}", x, v);
        }
        println!("Heap: {}", v);

        let mut o: Vec<uint> = Vec::new();
        while !v.is_empty() {
            heap::pop_heap(&mut v, |a,b| a.cmp(b));
            o.push(v.pop().unwrap());
            println!("Pop: {} {}", v, o);
        }
        println!("Output: {}", o);
        assert!(o.as_slice() == [1u,2u,3u,4u,5u,6u]);
    }
    #[test]
    fn test_remove_heap() {
        let mut v = vec![1u,6u,2u,5u,3u,4u];
        heap::make_heap(&mut v, |a,b| a.cmp(b));
        println!("Input: {}", v);
        heap::remove_heap(&mut v, 2, |a,b| a.cmp(b));
        let n = v.pop();
        println!("Output: {} ({})", v, n);
        assert!(n.unwrap() == 2u);
        assert!(v.as_slice() == [1u,3u,4u,5u,6u]);
    }
}

fn get_parent(c: uint) -> uint {
    c / 2u
}
fn get_child(n: uint) -> uint {
    (2 * n) + 1
}

fn bubble_down<T>(v: &mut [T], p: uint, cmp: |a:&T,b:&T| -> Ordering) {

    let mut done = false;
    let mut min_index = p;

    while !done && get_child(min_index) < v.len() {
        let p = min_index;
        let mut c = get_child(p);

        if c < v.len() && cmp(&v[min_index], &v[c]) == Greater {
            min_index = c
        }
        c = c + 1;
        if c < v.len() && cmp(&v[min_index], &v[c]) == Greater {
            min_index = c
        }
        if min_index != p {
            v.swap(p, min_index)
        } else {
            done = true
        }
    }
}

pub fn bubble_up<T>(v: &mut [T], child: uint, cmp: |a:&T,b:&T| -> Ordering) {
    let mut child = child;
    while child > 0 {
        let parent = get_parent(child);
        if cmp(&v[parent],&v[child]) == Greater {
            v.swap(child, parent);
            child = parent;
        } else {
            break
        }
    }
}

pub fn make_heap<T> (v: &mut Vec<T>, cmp: |a:&T,b:&T| -> Ordering) {
    for parent in range(0, get_parent(v.len())).rev() {
        bubble_down(v.as_mut_slice(), parent, |a,b| cmp(a,b));
    }
}

pub fn push_heap<T> (v: &mut Vec<T>, cmp: |a:&T,b:&T| -> Ordering) {
    let new_len = v.len() - 1;
    bubble_up(v.as_mut_slice(), new_len, |a,b| cmp(a,b))
}

pub fn pop_heap<T> (v: &mut Vec<T>, cmp: |a:&T,b:&T| -> Ordering) {
    if v.is_empty() { return }
    let new_len = v.len() - 1;
    if new_len > 0 {
        v.as_mut_slice().swap(0, new_len);
        bubble_down(v.slice_mut(0, new_len), 0, cmp)
    }
}

pub fn remove_heap<T> (v: &mut Vec<T>, index: uint, cmp: |a:&T,b:&T| -> Ordering) {
    let new_len = v.len() - 1;
    if new_len > index {
        v.as_mut_slice().swap(index, new_len);
        bubble_down(v.slice_mut(0, new_len), index, cmp);
    }
}

