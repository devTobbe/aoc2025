use std::error::Error;

use crate::utils::io;

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

pub fn day6p1() -> Result<(), Box<dyn Error>> {
    let mut problems: Vec<Vec<usize>> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let file = io::read_file("inputs/d6")?;
    parse_file(&file, &mut problems, &mut operators);

    // Introduce immutability
    let problems = problems;
    let operators = operators;

    let mut total = 0;

    for (index, problem) in problems.iter().enumerate() {
        let operator = &operators[index];
        total += solve_prob(problem, operator);
    }

    println!("Total: {total}");
    Ok(())
}

pub fn day6p2() -> Result<(), Box<dyn Error>> {
    let mut problems: Vec<Vec<usize>> = vec![];
    let mut operators: Vec<Operator> = vec![];

    let file = io::read_file("inputs/d6")?;
    parse_file(&file, &mut problems, &mut operators);

    // Introduce immutability
    let problems = problems;
    let operators = operators;

    let mut total = 0;

    for (index, problem) in problems.iter().enumerate() {
        let operator = &operators[index];
        total += solve_prob(problem, operator);
    }

    println!("Total: {total}");
    Ok(())
}

fn parse_file(input: &str, problems: &mut Vec<Vec<usize>>, operators: &mut Vec<Operator>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut splits: Vec<Vec<&str>> = vec![];

    for line in lines {
        let units: Vec<&str> = line.split_whitespace().collect();
        splits.push(units);
    }

    for col in &splits[0] {
        let cnum: usize = col.parse().unwrap();
        let probvec: Vec<usize> = vec![cnum];
        problems.push(probvec);
    }

    let rowlen = splits.len();
    let collen = splits[0].len();

    for col in 0..collen {
        for row in 1..rowlen {
            if is_operator(splits[row][col]) {
                let op = str_to_op(splits[row][col]).unwrap();
                operators.push(op);
            } else {
                let unit: usize = splits[row][col].parse().unwrap();
                problems[col].push(unit);
            }
        }
    }
}

fn solve_prob(numbers: &[usize], operator: &Operator) -> usize {
    let mut total: usize = 0;
    for number in numbers {
        if total == 0 {
            total += number;
        } else {
            total = perform_op(&total, number, operator);
        }
    }
    total
}

fn perform_op(numone: &usize, numtwo: &usize, op: &Operator) -> usize {
    match op {
        Operator::Add => numone + numtwo,
        Operator::Mult => numone * numtwo,
    }
}

// Evals if a string literal is an operator
fn is_operator(s: &str) -> bool {
    s == "+" || s == "*"
}

fn str_to_op(s: &str) -> Option<Operator> {
    match s {
        "+" => Some(Operator::Add),
        "*" => Some(Operator::Mult),
        _ => None,
    }
}
