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

// Attempts:
// 168127598850742 - Too Low
// 170418192256861 - CORRECT
pub fn day3p2() -> Result<(), Box<dyn Error>> {
    const BASE: usize = 10;
    const DIGIT: usize = 12;

    let file = io::read_file("inputs/d3")?;
    let banks = parse_file(&file);

    let mut total: usize = 0;

    for bank in banks {
        let blen = bank.len();
        let mut taken: Vec<usize> = vec![];

        for iter in 0..DIGIT {
            let floor = if iter == 0 { 0 } else { taken[iter - 1] + 1 };
            let roof = blen - (DIGIT - iter);

            let hi = find_highest(&bank[floor..=roof]);

            taken.push(hi.0 + floor);
        }

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
