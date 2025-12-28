use std::{collections::HashSet, error::Error};

use crate::utils::io;

// 1040 - TOO HIGH
pub fn day5p1() -> Result<(), Box<dyn Error>> {
    // .0 lower bound .1 higher bound
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut ingredients: Vec<usize> = Vec::new();

    let mut total = 0;

    let file = io::read_file("inputs/d5")?;

    parse_input(&file, &mut ranges, &mut ingredients);

    let ranges = ranges;
    let ingredirents = ingredients;

    for ingredient in &ingredirents {
        for range in &ranges {
            if &range.0 <= ingredient && ingredient <= &range.1 {
                total += 1;
                break;
            }
        }
    }

    println!("FINAL: {total}");

    Ok(())
}

fn parse_input<'a>(file: &'a str, ranges: &'a mut Vec<(usize, usize)>, ingredients: &'a mut Vec<usize>) {
    let lines: Vec<&str> = file.lines().collect();
    let mut fir_sec = true;

    for line in lines {
        if line.is_empty() && fir_sec {
            fir_sec = false;
            continue;
        }

        if fir_sec {
            parse_ranges(line, ranges);
        } else {
            parse_ingredients(line, ingredients);
        }
    }
}

fn parse_ranges(input: &str, ranges: &mut Vec<(usize, usize)>) {
    let bound: Vec<&str> = input.split('-').collect();

    let lower: usize = bound[0].parse().unwrap();
    let higher: usize = bound[1].parse().unwrap();

    ranges.push((lower, higher));
}

fn parse_ingredients(input: &str, ingredients: &mut Vec<usize>) {
    let id: usize = input.parse().unwrap();
    ingredients.push(id);
}
