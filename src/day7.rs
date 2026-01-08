use std::error::Error;

use crate::utils::io;

// Constants
const BEAM: char = '|';
const START: char = 'S';
const SPLITTER: char = '^';

pub fn day7p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d7t")?;
    let mut beamcoors = beam_scanner(&file);

    let mut manifold = file.clone();

    println!("{manifold}");
    for coor in &beamcoors {
        manifold = draw_beam(&manifold, coor);
    }
    println!("{manifold}");
    let mut cont = true;
    while cont {
        beamcoors = beam_scanner(&manifold);
        if beamcoors.is_empty() {
            cont = false
        }
        for coor in &beamcoors {
            manifold = draw_beam(&manifold, coor);
        }
        println!("{manifold}");
    }

    let total = caluclate_total(&manifold, BEAM);

    println!("{manifold}");
    println!("Total: {total}");
    Ok(())
}

// Collects points where BEAMs should be applied
fn beam_scanner(input: &str) -> Vec<(usize, usize)> {
    let mut retvec: Vec<(usize, usize)> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    // Get bounds of the input string
    // WARNING(tobbe): Assumes that width is uniform
    let ymax = lines.len() as isize;
    let xmax = lines[0].len() as isize;
    let ymin: isize = 0;
    let xmin: isize = 0;

    for (yi, y) in input.lines().enumerate() {
        // Skip first row
        if yi == 0 {
            continue;
        }
        for (xi, x) in y.chars().enumerate() {
            let iyi = yi as isize;
            let ixi = xi as isize;
            // WARNING: Has to be isize
            let symin = (iyi - 1).clamp(ymin, ymax - 1) as usize;
            let symax = (iyi + 1).clamp(ymin, ymax - 1) as usize;

            let sxmin = (ixi - 1).clamp(xmin, xmax - 1) as usize;
            let sxmax = (ixi + 1).clamp(xmin, xmax - 1) as usize;

            let row_slice = &lines[symin..=symax];
            let mut col_slices = Vec::new();

            for row in row_slice {
                col_slices.push(&row[sxmin..=sxmax]);
            }

            if should_transform(&col_slices) {
                retvec.push((yi, xi));
            }
        }
    }
    retvec
}

// Returns an updated version of the input with beams applied
fn draw_beam(input: &str, coor: &(usize, usize)) -> String {
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    lines[coor.0][coor.1] = BEAM;

    let rebuild: String = lines
        .into_iter()
        .map(|chars| chars.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    rebuild
}

// Determines whether or not a . should be transformed into |
fn should_transform(input: &[&str]) -> bool {
    let middle_y = input.len() / 2;
    let middle_x = input[middle_y].len() / 2;

    // Conditions to draw a |
    // 1. Current pos is below S
    // 2. Current pos is below | ( and not ^ and not | already)
    // 3. Directly right or left of |
    //
    // NOTE: SHOULD skip the middle, as that's the one being evaluated
    // NOTE: Should probably account for 2x2 2x3 matrices
    if let Some(middle) = input[middle_y].chars().nth(middle_x) {
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
            if (yi, xi) == (middle_y, middle_x) {
                continue;
            }

            // APPLY
            if (yi, xi) == (0, 1) && (x == START || x == BEAM) {
                return true;
            }
            if (xi == 0 || xi == 2) && yi == 1 && x == SPLITTER {
                return true;
            }
        }
    }
    false
}

// Takes a string and counts the amount of given item in a string
fn caluclate_total(input: &str, item: char) -> usize {
    let mut total = 0;
    for char in input.chars() {
        if char == BEAM {
            total += 1;
        }
    }
    total
}
