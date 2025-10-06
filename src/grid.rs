use std::fmt;
use std::ops::{Add, Sub};

// xy pair (position) used as index into a grid
// using i32 as index instead of usize because even for positive only,
// some puzzles need to ask for items outside the grid.
// also a future sparse grid may reasonably use negative indexes
#[derive(Copy, Clone, PartialEq)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

impl From<(usize, usize)> for P {
    fn from(pair: (usize, usize)) -> Self {
        P {
            x: pair.0 as i32,
            y: pair.1 as i32,
        }
    }
}

impl fmt::Debug for P {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for P {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for P {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub fn each_char(input: &str) -> impl Iterator<Item = (P, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (P::from((x, y)), c))
    })
}
