use std::error::Error;

use crate::utils::io;

pub fn day4p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d4t")?;
    let grid = parse_input(&file);
    let total = 0;

    for row in grid {
        for col in row {}
    }

    Ok(())
}

fn eval_position(grid: &[Vec<bool>], x: usize, y: usize) -> bool {
    let slice = &grid[..=x + 1][y - 1..=y + 1];
    for row in slice {
        for col in row {}
    }
    let adjacent: usize = 0;

    false
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
