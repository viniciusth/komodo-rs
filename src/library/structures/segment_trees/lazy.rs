use crate::library::structures::segment_trees::LazyNode;

pub struct LazySegmentTree<T, N: LazyNode<T>> {
    n: usize,
    data: Vec<N>,
    _t: std::marker::PhantomData<T>,
}

impl<T: Copy, N: LazyNode<T>> LazySegmentTree<T, N> {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            data: vec![N::new(); 4 * n],
            _t: std::marker::PhantomData,
        }
    }

    pub fn update(&mut self, ql: usize, qr: usize, value: T) {
        self.update_impl(1, 0, self.n - 1, ql, qr, value);
    }

    pub fn query(&mut self, ql: usize, qr: usize) -> T {
        self.query_impl(1, 0, self.n - 1, ql, qr).value()
    }

    fn update_impl(&mut self, node: usize, tl: usize, tr: usize, ql: usize, qr: usize, value: T) {
        self.push(node, tl, tr);
        if qr < tl || tr < ql {
            return;
        }
        if ql <= tl && tr <= qr {
            self.data[node].add_update(value);
            self.push(node, tl, tr);
            return;
        }
        let tm = (tl + tr) / 2;
        self.update_impl(2 * node, tl, tm, ql, qr, value);
        self.update_impl(2 * node + 1, tm + 1, tr, ql, qr, value);
        self.data[node] = self.data[2 * node].merge(&self.data[2 * node + 1]);
    }

    fn query_impl(&mut self, node: usize, tl: usize, tr: usize, ql: usize, qr: usize) -> N {
        self.push(node, tl, tr);
        if ql <= tl && tr <= qr {
            return self.data[node];
        }
        let tm = (tl + tr) / 2;
        if qr <= tm {
            return self.query_impl(2 * node, tl, tm, ql, qr);
        }
        if tm < ql {
            return self.query_impl(2 * node + 1, tm + 1, tr, ql, qr);
        }
        self.query_impl(2 * node, tl, tm, ql, qr)
            .merge(&self.query_impl(2 * node + 1, tm + 1, tr, ql, qr))
    }

    fn push(&mut self, node: usize, tl: usize, tr: usize) {
        let lazy = self.data[node].apply_update(tl, tr);
        if tl < tr {
            self.data[2 * node].add_update(lazy);
            self.data[2 * node + 1].add_update(lazy);
        }
    }
}
