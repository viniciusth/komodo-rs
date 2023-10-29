use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::{
    io::{create_io, Scanner},
    structures::segment_trees::{lazy::LazySegmentTree, SumNode},
};

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let (n, q) = (input.token::<usize>(), input.token::<usize>());
    let mut seg = LazySegmentTree::<i64, SumNode<i64>>::new(n);
    for i in 0..n {
        let x = input.token::<i64>();
        seg.update(i, i, x);
    }
    for _ in 0..q {
        let t = input.token::<usize>();
        match t {
            1 => {
                let (a, b, u) = (
                    input.token::<usize>(),
                    input.token::<usize>(),
                    input.token::<i64>(),
                );
                seg.update(a - 1, b - 1, u);
            }
            2 => {
                let k = input.token::<usize>();
                writeln!(output, "{}", seg.query(k - 1, k - 1))?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error + 'static>> {
    // 200MB
    const STACK_SIZE: usize = 200 * 1024 * 1024;
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let (input, output) = create_io();
            solve(input, output).unwrap()
        })
        .unwrap()
        .join()
        .unwrap();
    Ok(())
}
