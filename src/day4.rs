use std::error::Error;

use crate::utils::io;

pub fn day4p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d4")?;
    let grid = parse_input(&file);
    let mut total = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if neighbours(&grid, col, row, 4) && grid[row][col] {
                total += 1;
                println!("{total}");
            }
        }
    }

    Ok(())
}

// Evaluate a given position
fn neighbours(grid: &[Vec<bool>], x: usize, y: usize, n: u64) -> bool {
    // Locally cast for neighbour math
    let x = x as isize;
    let y = y as isize;

    let y_roof = grid.len().saturating_sub(1) as isize;
    let x_roof = grid[0].len().saturating_sub(1) as isize;

    let floor = 0;
    let mut adjacent: u64 = 0;

    for row in y - 1..=y + 1 {
        if is_in_bounds(row, floor, y_roof) {
            for col in x - 1..=x + 1 {
                if is_in_bounds(col, floor, x_roof) {
                    if row == y && col == x {
                        continue;
                    }
                    if grid[row as usize][col as usize] {
                        adjacent += 1;
                    }
                }
            }
        }
    }
    adjacent < n
}

fn is_in_bounds(value: isize, floor: isize, roof: isize) -> bool {
    floor <= value && value <= roof
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<bool> = Vec::new();
        for char in line.chars() {
            let roll = is_roll(char);
            row.push(roll);
        }
        grid.push(row);
    }

    grid
}

fn is_roll(c: char) -> bool {
    match c {
        '@' => true,
        _ => false,
    }
}
