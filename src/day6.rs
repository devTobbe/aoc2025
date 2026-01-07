use std::error::Error;

use crate::utils::io;

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
}

// Orchestrator for solve of day6p1
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

// Orchestrator for solve of day6p1
pub fn day6p2() -> Result<(), Box<dyn Error>> {
    let mut total = 0;

    let file = io::read_file("inputs/d6")?;

    // Introduce immutability
    let problems = parse_numbers(&file);
    let operators = parse_operators(&file);
    let roachprob = to_roachvec(&problems, &operators.len());

    for (index, problem) in roachprob.iter().enumerate() {
        let operator = &operators[operators.len()-1-index];
        total += solve_prob(problem, operator);
    }

    println!("Total: {total}");
    Ok(())
}

fn to_roachvec(numbers: &Vec<Vec<char>>, nop: &usize) -> Vec<Vec<usize>> {
    let mut newprobvec: Vec<Vec<usize>> = vec![];

    let longlen = find_longest_child_len(numbers);
    let rowlen = numbers.len();

    let mut newstrvec: Vec<Vec<String>> = vec![Vec::new(); *nop];

    let mut counter = 0;
    // TODO(tobbe): Change this to follow new logic, skip on ' ' and if vec empty continue
    for col in (0..longlen).rev() {
        let mut newstr = String::from("");
        for row in 0..rowlen {
            if let Some(chr) = numbers[row].get(col) {
                if *chr == ' ' {
                    continue;
                }
                newstr.push(*chr);
            }
        }

        if newstr.is_empty() {
            counter += 1;
            continue;
        }
        newstrvec[counter].push(newstr);
    }

    for strvec in newstrvec {
        let mut uvec: Vec<usize> = vec![];
        for str in strvec {
            let newu: usize = str.parse().unwrap();
            uvec.push(newu);
        }
        newprobvec.push(uvec);
    }

    newprobvec
}

fn find_longest_child_len(tv: &Vec<Vec<char>>) -> usize {
    let mut longest = 0;
    for vec in tv {
        if longest < vec.len() {
            longest = vec.len();
        }
    }
    longest
}

// Parses file and prepares data, probably has too much responsibility
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

// Parses file and prepares data, probably has too much responsibility
// This is much better lol
fn parse_numbers(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let mut splits: Vec<Vec<char>> = vec![];

    for (index, line) in lines.iter().enumerate() {
        if index == lines.len() - 1 {
            break;
        }
        let units: Vec<char> = line.chars().collect();
        splits.push(units);
    }
    splits
}

// Parse operators
fn parse_operators(input: &str) -> Vec<Operator> {
    let lines: Vec<&str> = input.lines().collect();
    let mut operators: Vec<Operator> = vec![];

    let ops = lines.last().unwrap();
    for op in ops.chars() {
        if op == ' ' {
            continue;
        }
        let convop = str_to_op(&op.to_string()).unwrap();
        operators.push(convop);
    }

    operators
}

// Takes a array of numbers (problem) and an operator and applies the operator
// on all numbers for a combined total
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

// Performs an operation depending on OP
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

// Turns a string into an Operator
fn str_to_op(s: &str) -> Option<Operator> {
    match s {
        "+" => Some(Operator::Add),
        "*" => Some(Operator::Mult),
        _ => None,
    }
}
