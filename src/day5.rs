use std::{collections::HashSet, error::Error};

use crate::utils::io;

pub fn day5p1() -> Result<(), Box<dyn Error>> {
    let mut ranges: HashSet<usize> = HashSet::new();
    let mut ingredients: Vec<usize> = Vec::new();

    let mut total = 0;

    let file = io::read_file("inputs/d5")?;

    parse_input(&file, &mut ranges, &mut ingredients);

    let ranges = ranges;
    let ingredirents = ingredients;

    for ingredient in ingredirents {
        if ranges.contains(&ingredient) {
            total += 1;
        }
    }

    println!("FINAL: {total}");

    Ok(())
}

fn parse_input<'a>(file: &'a str, ranges: &'a mut HashSet<usize>, ingredients: &'a mut Vec<usize>) {
    let lines: Vec<&str> = file.lines().collect();
    let mut fir_sec = true;

    for line in lines {
    println!("line");
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

fn parse_ranges(input: &str, ranges: &mut HashSet<usize>) {
    let bound: Vec<&str> = input.split('-').collect();

    let lower: usize = bound[0].parse().unwrap();
    let high: usize = bound[1].parse().unwrap();

    for id in lower..=high {
        ranges.insert(id);
    }
}

fn parse_ingredients(input: &str, ingredients: &mut Vec<usize>) {
    let id: usize = input.parse().unwrap();
    ingredients.push(id);
}
