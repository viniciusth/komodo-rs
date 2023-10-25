use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::io::{create_io, Scanner};

pub fn solve<I: BufRead, O: Write>(
    mut _input: Scanner<I>,
    mut _output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
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
