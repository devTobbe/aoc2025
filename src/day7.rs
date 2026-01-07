// Placeholder

use std::error::Error;

use crate::utils::io;

pub fn day7p1() -> Result<(), Box<dyn Error>> {
    let file = io::read_file("inputs/d7t")?;
    let startingpoint = find_start(&file);

    Ok(())
}

// Finds the startingpoint of the tachyon beam
fn find_start(input: &str) -> Option<(usize, usize)> {
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == 'S' {
                return Some((y, x));
            }
        }
    }
    None
}
