use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::io::{create_io, Scanner};

// Checks if two segments have intersection, wrong for stress testing purposes
pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let (l1, r1, l2, r2): (i32, i32, i32, i32) =
        (input.token(), input.token(), input.token(), input.token());

    let contains = |l, r, p| l <= p && p <= r;

    writeln!(
        &mut output,
        "{}",
        if contains(l1, r1, l2) || contains(l1, r1, r2) {
            "YES"
        } else {
            "NO"
        }
    )?;

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
