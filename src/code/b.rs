use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::{
    geometry::{convex_hull::convex_hull, point::Point},
    io::{create_io, Scanner},
};

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let n: usize = input.token();
    let mut points = Vec::with_capacity(n);
    for _ in 0..n {
        points.push(Point::<i32>::new(input.token(), input.token()));
    }

    let hull = convex_hull(points);
    writeln!(&mut output, "{hull:?}")?;

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
