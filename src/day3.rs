// TODO: Da code

use std::error::Error;

use crate::utils::io;

pub fn day3p1() -> Result<(), Box<dyn Error>> {
    const BASE: usize = 10;
    let file = io::read_file("inputs/d3t")?;
    let banks = parse_file(&file);

    let mut total: usize = 0;

    for bank in banks {
        let bank_len = bank.len();
        let mut hi = find_highest(&bank);
        if hi.0 == bank_len - 1 {
            hi = find_highest(&bank[..bank_len - 1]);
        }

        let scn_hi = find_highest(&bank[hi.0 + 1..]);
        total += BASE * hi.1 + scn_hi.1;
        println!("{} {}", hi.1, scn_hi.1);
    }

    println!("{total}");
    Ok(())
}

pub fn day3p2() -> Result<(), Box<dyn Error>> {
    const BASE: usize = 10;
    const DIGIT: usize = 12;

    let file = io::read_file("inputs/d3t")?;
    let banks = parse_file(&file);

    let mut total: usize = 0;

    for bank in banks {
        let blen = bank.len();
        let mut taken: Vec<usize> = vec![];
        let hi = find_highest(&bank[0..blen - DIGIT]);
        taken.push(hi.0);

        for i in 0..(DIGIT - 1) {
            let max = bank
                .iter()
                .enumerate()
                .filter(|(i, j)| !taken.contains(i) && i > &hi.0)
                .max_by_key(|(_, val)| *val)
                .unwrap();

            taken.push(max.0);
        }

        taken.sort();

        let mut counter = DIGIT as u32;
        let mut sum = 0;
        for i in taken {
            counter -= 1;
            sum += bank[i] * BASE.pow(counter);
        }
        total += sum;
    }

    println!("{total}");
    Ok(())
}

// Finds the highest value in a given range and returns it together with the corresponding index
// Tuple of (index, highest)
fn find_highest(bank: &[usize]) -> (usize, usize) {
    let mut highest: usize = 0;
    let mut hi_index: usize = 0;

    for (index, jolt) in bank.iter().enumerate() {
        if jolt > &highest {
            highest = *jolt;
            hi_index = index;
        }
    }

    (hi_index, highest)
}

fn parse_file(s: &str) -> Vec<Vec<usize>> {
    let mut banks: Vec<Vec<usize>> = Vec::new();

    // Prase file
    for bank in s.lines() {
        let mut entry: Vec<usize> = Vec::new();
        for jolt in bank.chars() {
            entry.push(jolt.to_digit(10).unwrap() as usize);
        }
        banks.push(entry);
    }

    banks
}
