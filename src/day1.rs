// DAY a

use std::error::Error;

use crate::utils::io;

// Obtain Password
// Dial with arrow on it, Numbers 0-99
// Document:
// L or R (- or +) and has a distance, the amount of clicks. NOTE: STARTS AT 50

#[derive(Debug)]
struct Dial {
    val: u8,
}

#[derive(Debug)]
pub enum Side {
    Left,
    Right,
}

impl Dial {
    pub const DIAL_MAX: u8 = 100;

    pub fn new() -> Self {
        Self { val: 50 }
    }

    fn add(&mut self, i: u8) {
        self.val = (self.val + i) % Self::DIAL_MAX;
    }

    fn subtract(&mut self, i: u8) {
        self.val = (self.val - i) % Self::DIAL_MAX;
    }

    pub fn rotation(&mut self, s: Side, dist: u8) {
        match s {
            Side::Left => {
                self.subtract(dist);
            }
            Side::Right => {
                self.add(dist);
            }
        }
    }
}

fn day1() -> Result<(), Box<dyn Error>> {
    todo!();
    Ok(())
}
