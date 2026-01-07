// Placeholder

use std::error::Error;

use crate::utils::io;

// Constants
const BEAM: char = '|';
const START: char = 'S';
const SPLITTER: char = '^';

pub fn day7p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d7t")?;

    Ok(())
}

// Collects points where BEAMs should be applied
fn bream_scanner(input: &str) -> Vec<(usize, usize)> {
    for y in input.lines() {
        for x in y.chars() {
            todo!()
        }
    }
    todo!();
}

// Returns an updated version of the input with beams applied
fn send_beam(input: &str) -> String {
    let beam_cords = bream_scanner(input);
    todo!();
}

// Determines whether or not a . should be transformed into |
fn should_transform(input: &[&str]) -> bool {
    // Conditions to draw a |
    // 1. Current pos is below S
    // 2. Current pos is below | ( and not ^ and not | already)
    // 3. Directly right or left of |
    for y in input {
        for x in y.chars() {
            todo!()
        }
    }
    todo!();
}

// Takes a string and counts the amount of given item in a string
fn caluclate_total(input: &str, item: &char) -> usize {
    let mut total = 0;
    for char in input.chars() {
        if char == BEAM {
            total += 1;
        }
    }
    total
}
