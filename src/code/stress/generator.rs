use std::{
    error::Error,
    io::{BufWriter, Write},
};

use rand::Rng;

pub fn generate_case<T: Write>(mut output: BufWriter<T>) -> Result<(), Box<dyn Error + 'static>> {
    let mut rng = rand::thread_rng();
    let mut ranges: [u8; 4] = rng.gen();

    if ranges[0] > ranges[1] {
        ranges.swap(0, 1);
    }

    if ranges[2] > ranges[3] {
        ranges.swap(2, 3);
    }

    writeln!(
        &mut output,
        "{} {} {} {}",
        ranges[0], ranges[1], ranges[2], ranges[3]
    )?;

    Ok(())
}
