// DAY a

use std::error::Error;

use crate::utils::io;

// Obtain Password
// Dial with arrow on it, Numbers 0-99
// Document:
// L or R (- or +) and has a distance, the amount of clicks. NOTE: STARTS AT 50

#[derive(Debug)]
struct Dial {
    val: u16,
}

#[derive(Debug)]
pub enum Side {
    Left,
    Right,
    Unknown,
}

impl Dial {
    pub const DIAL_MAX: u16 = 100;

    pub fn new() -> Self {
        Self { val: 50 }
    }

    pub fn val(&self) -> u16 {
        self.val
    }

    fn add(&mut self, i: u16) -> u16 {
        self.val = (self.val + i) % Self::DIAL_MAX;
        // Already applied ... Not quite right
        self.eval_add(i / Self::DIAL_MAX)
    }

    fn subtract(&mut self, i: u16) -> u16 {
        self.val = (self.val + Self::DIAL_MAX - (i % Self::DIAL_MAX)) % Self::DIAL_MAX;
        // Already applied ... Not quite right
        self.eval_subtract(i / Self::DIAL_MAX)
    }

    pub fn eval_input(&self, i: u16) -> u16 {
        i / Self::DIAL_MAX
    }

    pub fn eval_add(&self, i: u16) -> u16 {
        if (self.val() + i) > Self::DIAL_MAX - 1 {
            println!("ADD val: {} i {} ", self.val(), i);
            return 1;
        }
        0
    }

    pub fn eval_subtract(&self, i: u16) -> u16 {
        let casted_val: i16 = self.val() as i16;
        let casted_i: i16 = i as i16;
        if (casted_val - casted_i) < 0 {
            println!("SUB val: {} i {} ", self.val(), i);
            return 1;
        }
        0
    }

    pub fn rotation(&mut self, s: Side, dist: u16) -> u16 {
        match s {
            Side::Left => self.subtract(dist),
            Side::Right => self.add(dist),
            Side::Unknown => 0,
        }
    }
}

impl Side {
    pub fn new(s: &str) -> Side {
        match s {
            "R" => Side::Right,
            "L" => Side::Left,
            _ => Side::Unknown,
        }
    }
}

pub fn day1p1() -> Result<(), Box<dyn Error>> {
    // Read inputs file
    let file = io::read_file("inputs/day1")?;

    // Get every line
    let parts = file.split("\n");

    let mut dial = Dial::new();
    let mut counter = 0;

    for part in parts {
        if part.is_empty() {
            continue;
        }
        // Extract
        let side = &part[..1];
        let digit = &part[1..];

        // Transform
        let sside = Side::new(side);
        let ddigit = digit.to_string().parse::<u16>()?;

        println!("side: {}, ddigit: {}", side, ddigit);

        dial.rotation(sside, ddigit);
        if dial.val() == 0 {
            println!("dial is: {}\n", dial.val);
            counter += 1;
        }
    }

    println!("{}", &counter);

    Ok(())
}

// Be careful: if the dial were pointing at 50, a single rotation like R1000
// would cause the dial to point at 0 ten times before returning back to 50!
pub fn day1p2() -> Result<(), Box<dyn Error>> {
    // Read inputs file
    let file = io::read_file("inputs/day1")?;

    // Get every line
    let parts = file.split("\n");

    let mut dial = Dial::new();
    let mut counter = 0;

    for part in parts {
        if part.is_empty() {
            continue;
        }
        // Extract
        let side = &part[..1];
        let digit = &part[1..];

        // Transform
        let sside = Side::new(side);
        let ddigit = digit.to_string().parse::<u16>()?;

        println!("side: {}, ddigit: {}", side, ddigit);

        // Good, gets if ddigit loops around several times
        counter += dial.eval_input(ddigit);
        // Gets whenever it ends on 0
        counter += dial.rotation(sside, ddigit);
        if dial.val() == 0 {
            println!("dial is: {}\n", dial.val);
            counter += 1;
        }
    }

    println!("{}", &counter);

    Ok(())
}

// I have to somehow take the remainder after checking how many full spins the dial does and then
// check whether or not it goes past 0
