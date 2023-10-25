use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::{
    io::{create_io, Scanner},
    structures::sparse_table::SparseTable,
};

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let n: usize = input.token();
    let mut a = Vec::<i32>::with_capacity(n);
    for _ in 0..n {
        a.push(input.token());
    }
    let s = SparseTable::new(a);

    let l = input.token::<usize>() - 1;
    let r = input.token::<usize>() - 1;
    writeln!(&mut output, "{}", s.query(l, r))?;

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
