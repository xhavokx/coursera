#[allow(dead_code)]
struct Heap<T> {
    container: Vec<T>,
    compare_fn: fn(T,T) -> bool,
    len: uint,
}

#[allow(dead_code)]
impl<T: Clone> Heap<T> {
    fn new(compare: fn(T,T) -> bool) -> Heap<T>{
        Heap { container: Vec::new(), compare_fn: compare, len: 0 }
    }

    fn from_slice(v: &[T], compare: fn(T,T) -> bool) -> Heap<T> {
        let mut heap: Heap<T> = Heap::new(compare);
        heap.container.push_all(v);
        heap.len = v.len();
        heap.heapify();
        heap
    }

    fn insert(&mut self, item: T) {
        self.container.push(item);
        let len = self.len;
        self.bubble_up(len);
        self.len = len + 1;
    }

    fn top(&self) -> T {
        self.container[0].clone()
    }

    fn get_parent(&self, child: uint) -> uint {
        child / 2
    }

    fn get_left_child(&self, parent: uint) -> uint {
        (parent * 2) + 1
    }

    fn swap(&mut self, a: uint, b: uint) {
        self.container.as_mut_slice().swap(a,b);
    }

    fn compare(&self, a: uint, b: uint) -> bool {
        let cmp = self.compare_fn;
        cmp(self.container[a].clone(), self.container[b].clone())
    }

    fn bubble_up(&mut self, child: uint) {
        while child > 0 {
            let parent = self.get_parent(child);
            if self.compare(child, parent) {
                self.swap(child, parent);
                self.bubble_up(parent);
            } else {
                break;
            }
        }
    }
    fn bubble_down(&mut self, index: uint) {
        let mut done = false;
        let mut parent = index;

        while !done {
            let left_child = self.get_left_child(parent);
            let right_child = left_child + 1;
            let mut min_index = parent;
            if left_child < self.len
                && self.compare(left_child, min_index) {
                    min_index = left_child;
                }
            if right_child < self.len
                && self.compare(right_child, min_index) {
                    min_index = right_child;
                }
            if min_index != parent {
                self.swap(min_index, parent);
                parent = min_index
            } else {
                done = true 
            }
        }
    }

    fn heapify(&mut self) {
        for parent in range(0, self.get_parent(self.len)).rev() {
            self.bubble_down(parent)
        }
    }
}
