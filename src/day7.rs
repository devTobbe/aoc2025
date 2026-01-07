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
fn beam_scanner(input: &str) -> Vec<(usize, usize)> {
    let mut retvec: Vec<(usize, usize)> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    // Get bounds of the input string
    // WARNING(tobbe): Assumes that width is uniform
    let ymax = lines.len();
    let xmax = lines[0].len();
    let ymin = 0;
    let xmin = 0;

    for (yi, y) in input.lines().enumerate() {
        for (xi, x) in y.chars().enumerate() {
            let symin = (yi - 1).clamp(ymin, ymax);
            let symax = (yi + 1).clamp(ymin, ymax);

            let sxmin = (xi - 1).clamp(xmin, xmax);
            let sxmax = (xi + 1).clamp(xmin, xmax);

            let slice = &lines[symin..symax][sxmin..sxmax];
            if should_transform(slice) {
                retvec.push((yi, xi));
            }
        }
    }
    retvec
}

// Returns an updated version of the input with beams applied
fn draw_beam(input: &str) -> String {
    let beam_cords = beam_scanner(input);
    todo!();
}

// Determines whether or not a . should be transformed into |
fn should_transform(input: &[&str]) -> bool {
    // Conditions to draw a |
    // 1. Current pos is below S
    // 2. Current pos is below | ( and not ^ and not | already)
    // 3. Directly right or left of |
    //
    // NOTE: SHOULD skip the middle, as that's the one being evaluated
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
