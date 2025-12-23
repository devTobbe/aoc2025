// TODO: Da code

use std::error::Error;

use crate::utils::io;

pub fn day3p1() -> Result<(), Box<dyn Error>> {
    const BASE: u32 = 10;
    let file = io::read_file("inputs/d3")?;
    let banks = parse_file(&file);

    let mut total: u32 = 0;

    for bank in banks {
        let bank_len = bank.len();
        let mut hi = find_highest(&bank);
        if hi.0 == bank_len - 1 {
            hi = find_highest(&bank[..bank_len - 1]);
        }

        let scn_hi = find_highest(&bank[hi.0+1..]);
        total += BASE * hi.1 + scn_hi.1;
        println!("{} {}", hi.1, scn_hi.1);
    }

    println!("{total}");
    Ok(())
}

// Finds the highest value in a given range and returns it together with the corresponding index
// Tuple of (index, highest)
fn find_highest(bank: &[u32]) -> (usize, u32) {
    let mut highest: u32 = 0;
    let mut hi_index: usize = 0;

    for (index, jolt) in bank.iter().enumerate() {
        if jolt > &highest {
            highest = *jolt;
            hi_index = index;
        }
    }

    (hi_index, highest)
}

fn parse_file(s: &str) -> Vec<Vec<u32>> {
    let mut banks: Vec<Vec<u32>> = Vec::new();

    // Prase file
    for bank in s.lines() {
        let mut entry: Vec<u32> = Vec::new();
        for jolt in bank.chars() {
            entry.push(jolt.to_digit(10).unwrap());
        }
        banks.push(entry);
    }

    banks
}
