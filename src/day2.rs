use std::error::Error;

use crate::utils::io;

#[derive(Debug)]
struct IDPair {
    bound: (u64, u64),
}

impl IDPair {
    pub fn new(low: u64, high: u64) -> IDPair {
        IDPair { bound: (low, high) }
    }
}

pub fn day2p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/day2")?;
    let total: u64 = 0;

    let pairs = file.split(",");

    Ok(())
}
