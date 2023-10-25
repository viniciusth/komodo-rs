type SparseTableOperation<T> = Box<dyn Fn(T, T) -> T>;

pub struct SparseTable<T> {
    pub table: Vec<Vec<T>>,
    pub log2: Vec<usize>,
    pub operation: SparseTableOperation<T>,
}

impl<T> SparseTable<T>
where
    T: Ord + Copy,
{
    pub fn new(input: Vec<T>) -> Self {
        let operation = Box::new(|a: T, b: T| a.min(b));
        Self::build(input, operation)
    }

    pub fn new_with_operation(input: Vec<T>, operation: SparseTableOperation<T>) -> Self {
        Self::build(input, operation)
    }

    fn build(input: Vec<T>, operation: SparseTableOperation<T>) -> Self {
        let n = input.len();
        let mut log2 = vec![0; n + 1];
        for i in 2..=n {
            log2[i] = log2[i / 2] + 1;
        }

        let mut table = Vec::with_capacity(log2[n] + 1);
        table.push(input);
        for i in 1..=log2[n] {
            let last = &table[i - 1];
            let mut cur = last.clone();
            let pw = 1 << (i - 1);
            for j in 0..(n - pw) {
                cur[j] = operation(last[j], last[j + pw]);
            }
            table.push(cur);
        }

        Self {
            table,
            log2,
            operation,
        }
    }

    /// RMQ over range, merges [l, l + 2^k] and (r - 2^k, r].
    pub fn query(&self, l: usize, r: usize) -> T {
        let k = self.log2[r - l + 1];
        (self.operation)(self.table[k][l], self.table[k][r + 1 - (1 << k)])
    }
}

#[cfg(test)]
mod tests {
    use super::SparseTable;

    #[test]
    fn simple_array_check_all_subarrays() {
        let arr = vec![5, 5, 4, 4, 3, 3, 2, 3, 2, 4, 5, 5];

        let st = SparseTable::new(arr.clone());

        for i in 0..arr.len() {
            let mut min_el = arr[i];
            for j in i..arr.len() {
                min_el = min_el.min(arr[j]);
                assert_eq!(min_el, st.query(i, j));
            }
        }
    }
}
