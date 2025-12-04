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

    fn add(&mut self, i: u16) {
        self.val = (self.val + i) % Self::DIAL_MAX;
    }

    fn subtract(&mut self, i: u16) {
        self.val = (self.val + Self::DIAL_MAX - (i % Self::DIAL_MAX)) % Self::DIAL_MAX;
    }

    pub fn rotation(&mut self, s: Side, dist: u16) {
        match s {
            Side::Left => {
                self.subtract(dist);
            }
            Side::Right => {
                self.add(dist);
            }
            Side::Unknown => {}
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

pub fn day1p2() -> Result<(), Box<dyn Error>> {
    todo!();
}
