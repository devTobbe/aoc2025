use std::error::Error;

use crate::utils::io;

#[derive(Debug)]
struct IDPair {
    range: (u64, u64),
}

impl IDPair {
    pub fn new(low: u64, high: u64) -> IDPair {
        IDPair { range: (low, high) }
    }

    fn iter(&self) -> impl Iterator<Item = u64> {
        self.range.0..self.range.1
    }
}

pub fn day2p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/day2")?;
    let total: u64 = 0;
    let mut pairs: Vec<IDPair> = Vec::new();

    let str_pairs = file.split(",");

    for pair in str_pairs {
        let splits: Vec<&str> = pair.split("-").collect();
        let low: u64 = splits[0].parse()?;
        let high: u64 = splits[1].parse()?;
        pairs.push(IDPair::new(low, high));
    }

    Ok(())
}
