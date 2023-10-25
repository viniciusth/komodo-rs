use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
};

use crate::library::io::Scanner;

// checks if two segments have intersection
pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let (l1, r1, l2, r2): (i32, i32, i32, i32) =
        (input.token(), input.token(), input.token(), input.token());

    writeln!(
        &mut output,
        "{}",
        if l1.max(l2) <= r1.min(r2) {
            "YES"
        } else {
            "NO"
        }
    )?;

    Ok(())
}
