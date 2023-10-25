use std::{
    error::Error,
    io::{BufWriter, Cursor},
    thread,
};

use similar::TextDiff;

use crate::library::io::Scanner;

use super::Question;

pub mod brute_force;
pub mod generator;

pub fn run_stress(question: Question) {
    thread::Builder::new()
        .stack_size(200 * 1024 * 1024)
        .spawn(|| {
            stress(question).unwrap();
        })
        .unwrap()
        .join()
        .unwrap();
}

fn stress(question: Question) -> Result<(), Box<dyn Error + 'static>> {
    let mut i = 1;
    loop {
        print!("\rRunning test {i}");
        let mut case = Vec::<u8>::new();
        generator::generate_case(BufWriter::new(Cursor::new(&mut case)))?;

        let mut brute_force_output = Vec::<u8>::new();
        brute_force::solve(
            Scanner::new(Cursor::new(&case)),
            BufWriter::new(Cursor::new(&mut brute_force_output)),
        )?;

        let mut solution_output = Vec::<u8>::new();
        question.run_solve(
            Scanner::new(Cursor::new(&case)),
            BufWriter::new(Cursor::new(&mut solution_output)),
        )?;

        let bfo = String::from_utf8(brute_force_output)?;
        let so = String::from_utf8(solution_output)?;
        let diff = TextDiff::from_lines(&bfo, &so);

        if diff.ratio() < 1.0 {
            println!("\nTest {i} failed");
            println!("Input: {}", String::from_utf8(case)?);
            print!("{}", diff.unified_diff().header("brute force", "solution"));
            break;
        }

        i += 1;
    }

    Ok(())
}
