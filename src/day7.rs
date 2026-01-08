use std::error::Error;

use crate::utils::io;

// Constants
const BEAM: char = '|';
const START: char = 'S';
const SPLITTER: char = '^';

pub fn day7p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d7")?;
    let manifold = beam_scanner(&file);

    let total = caluclate_total(&manifold);

    println!("{manifold}");
    println!("Total: {total}");
    Ok(())
}

// Collects points where BEAMs should be applied
fn beam_scanner(input: &str) -> String {
    let mut lines: Vec<String> = input.lines().map(String::from).collect();

    // Get bounds of the input string
    // WARNING(tobbe): Assumes that width is uniform
    let ymax = lines.len() as isize;
    let xmax = lines[0].len() as isize;
    let ymin: isize = 0;
    let xmin: isize = 0;

    for yi in 0..lines.len() {
        // Skip first row
        if yi == 0 {
            continue;
        }
        for xi in 0..lines[yi].len() {
            let iyi = yi as isize;
            let ixi = xi as isize;
            // WARNING: Has to be isize
            let symin = (iyi - 1).clamp(ymin, ymax - 1) as usize;
            let symax = (iyi).clamp(ymin, ymax - 1) as usize;

            let sxmin = (ixi - 1).clamp(xmin, xmax - 1) as usize;
            let sxmax = (ixi + 1).clamp(xmin, xmax - 1) as usize;

            let row_slice = &lines[symin..=symax];
            let mut col_slices = Vec::new();

            for row in row_slice {
                col_slices.push(&row[sxmin..=sxmax]);
            }

            let mid_in_slice = (yi - symin, xi - sxmin);
            if should_transform(&col_slices, mid_in_slice) {
                lines[yi].replace_range(xi..=xi, &BEAM.to_string());
            }
        }
    }
    let output: String = lines.join("\n");
    output
}

// Determines whether or not a . should be transformed into |
fn should_transform(input: &[&str], middle: (usize, usize)) -> bool {
    let (my, mx) = middle;

    // Conditions to draw a |
    // 1. Current pos is below S
    // 2. Current pos is below | ( and not ^ and not | already)
    // 3. Directly right or left of |
    //
    // NOTE: SHOULD skip the middle, as that's the one being evaluated
    // NOTE: Should probably account for 2x2 2x3 matrices
    if let Some(middle) = input[my].chars().nth(mx) {
        if middle == SPLITTER {
            return false;
        }
        if middle == BEAM {
            return false;
        }
    }
    for (yi, y) in input.iter().enumerate() {
        for (xi, x) in y.chars().enumerate() {
            // Continue
            if (yi, xi) == (my, mx) {
                continue;
            }

            // Above
            if yi + 1 == my && xi == mx && (x == START || x == BEAM) {
                return true;
            }

            // Sides
            if yi == my && xi + 1 == mx && x == SPLITTER {
                return true;
            }
            if yi == my && xi == mx + 1 && x == SPLITTER {
                return true;
            }
        }
    }
    false
}

// Takes a string and counts the amount of given item in a string
fn caluclate_total(input: &str) -> usize {
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut total = 0;

    // Get bounds of the input string
    // WARNING(tobbe): Assumes that width is uniform
    let ymax = lines.len() as isize;
    let xmax = lines[0].len() as isize;
    let ymin: isize = 0;
    let xmin: isize = 0;

    for yi in 0..lines.len() {
        // Skip first row
        if yi == 0 {
            continue;
        }
        for xi in 0..lines[yi].len() {
            let iyi = yi as isize;
            let ixi = xi as isize;
            // WARNING: Has to be isize
            let symin = (iyi - 1).clamp(ymin, ymax - 1) as usize;
            let symax = (iyi).clamp(ymin, ymax - 1) as usize;

            let sxmin = (ixi - 1).clamp(xmin, xmax - 1) as usize;
            let sxmax = (ixi + 1).clamp(xmin, xmax - 1) as usize;

            let row_slice = &lines[symin..=symax];
            let mut col_slices = Vec::new();

            for row in row_slice {
                col_slices.push(&row[sxmin..=sxmax]);
            }

            let mid_in_slice = (yi - symin, xi - sxmin);
            if check_split(&col_slices, mid_in_slice) {
                total += 1;
            }
        }
    }
    total
}

fn check_split(input: &[&str], middle: (usize, usize)) -> bool {
    let (my, mx) = middle;

    if let Some(mid) = input[my].chars().nth(mx)
        && mid == SPLITTER
    {
        for (yi, y) in input.iter().enumerate() {
            for (xi, x) in y.chars().enumerate() {
                // Continue
                if (yi, xi) == (my, mx) {
                    continue;
                }
                if yi + 1 == my && xi == mx && x == BEAM {
                    return true;
                }
            }
        }
    }
    false
}
