
pub mod quicksort {

    pub fn pivot_first(_: &[int]) -> uint {
        0
    }

    pub fn pivot_last(v: &[int]) -> uint {
        v.len() - 1
    }

    fn max (v: &[int], a: uint, b: uint) -> uint {
        if v[a] >= v[b] { return a } else { return b };
    }

    fn min (v: &[int], a: uint, b: uint) -> uint {
        if v[a] <= v[b] { return a } else { return b };
    }

    pub fn pivot_median(v: &[int]) -> uint {
        let len = v.len();

        if len == 1 { return 0 }
        else if len == 2 { return min(v, 0,1) }

        let mid = if (len % 2) == 0 { (len / 2) - 1 } else { len / 2 };

        let median = max(v, min(v, 0,mid), min(v, max(v, 0,mid),len - 1));

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

    pub fn quick_sort(v: &mut [int], cmp: |uint|, choose_pivot: |&[int]|->uint) {
        let len = v.len();
        if  len < 2 {
            return
        }
        let pivot_index = partition(v, |v| choose_pivot(v));

        cmp(len - 1);

        quick_sort(v.slice_mut(0, pivot_index), |x| cmp(x), |v| choose_pivot(v));
        quick_sort(v.slice_mut(pivot_index + 1, len), |x| cmp(x), |v| choose_pivot(v));
    }
}
