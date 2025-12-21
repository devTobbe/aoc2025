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

    fn perform_movement_left(&mut self, dist: u16) {
        self.val = (self.val + Self::DIAL_MAX - (dist % Self::DIAL_MAX)) % Self::DIAL_MAX;
    }

    fn perform_movement_right(&mut self, dist: u16) {
        self.val = (self.val + dist) % Self::DIAL_MAX;
    }

    fn add(&mut self, i: u16) -> u16 {
        let full_passes = i / Self::DIAL_MAX;
        let remainder = i % Self::DIAL_MAX;

        let cross = if self.val + remainder > 100 { 1 } else { 0 };
        let zeroed = if self.val == 0 { 1 } else { 0 };
        println!("fp: {}, cross: {}", full_passes, cross);
        self.perform_movement_right(i);
        full_passes + cross + zeroed
    }

    fn subtract(&mut self, i: u16) -> u16 {
        let full_passes = i / Self::DIAL_MAX;
        let remainder = i % Self::DIAL_MAX;
        let old = self.val;

        self.perform_movement_left(i);
        let cross = if (remainder != 0) && old < self.val {
            1
        } else {
            0
        };
        println!(
            "old:{}, val:{}, fp: {}, extra: {}, remainder: {}",
            old, self.val, full_passes, cross, remainder
        );
        full_passes + cross
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
        match s.to_uppercase().as_str() {
            "R" => Side::Right,
            "L" => Side::Left,
            _ => Side::Unknown,
        }
    }
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

        // Gets whenever it ends on 0
        counter += dial.rotation(sside, ddigit);
        //if dial.val() == 0 {
        //    counter += 1;
        //}
        println!("dial is: {}, counter is {}\n", dial.val, counter);
    }

    println!("{}", &counter);

    Ok(())
}

// I have to somehow take the remainder after checking how many full spins the dial does and then
// check whether or not it goes past 0
