use crate::compass::Compass;
use std::f32::consts::PI;
use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse position: {:?}", self.0)
    }
}

impl std::error::Error for ParseError {}

// 2d position with x and y coordinates
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub const ORIGIN: Pos = Pos { x: 0, y: 0 };

impl From<(usize, usize)> for Pos {
    fn from(pair: (usize, usize)) -> Self {
        Pos {
            x: pair.0 as i32,
            y: pair.1 as i32,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(pair: (i32, i32)) -> Self {
        Pos {
            x: pair.0,
            y: pair.1,
        }
    }
}

impl FromStr for Pos {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.splitn(2, ',');

        let mut next_number = || -> Result<i32, ParseError> {
            numbers
                .next()
                .ok_or_else(|| ParseError(s.to_string()))?
                .trim()
                .parse()
                .map_err(|_| ParseError(s.to_string()))
        };

        Ok(Self {
            x: next_number()?,
            y: next_number()?,
        })
    }
}

impl Add for Pos {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Pos {
    pub fn walk(&self, dir: Compass, steps: i32) -> Self {
        match dir {
            Compass::North => Self {
                x: self.x,
                y: self.y - steps,
            },
            Compass::NorthEast => Self {
                x: self.x + steps,
                y: self.y - steps,
            },
            Compass::East => Self {
                x: self.x + steps,
                y: self.y,
            },
            Compass::SouthEast => Self {
                x: self.x + steps,
                y: self.y + steps,
            },
            Compass::South => Self {
                x: self.x,
                y: self.y + steps,
            },
            Compass::SouthWest => Self {
                x: self.x - steps,
                y: self.y + steps,
            },
            Compass::West => Self {
                x: self.x - steps,
                y: self.y,
            },
            Compass::NorthWest => Self {
                x: self.x - steps,
                y: self.y - steps,
            },
        }
    }

    pub fn step(&self, dir: Compass) -> Self {
        self.walk(dir, 1)
    }

    // direction as (dx, dy), normalised by dividing by gcd
    pub fn direction_dxdy(&self, other: &Self) -> (i32, i32) {
        if self == other {
            panic!("Attemted to calculate direction from {} to itself", self);
        }
        let diff = *other - *self;
        let gcd = num_integer::gcd(diff.x.unsigned_abs(), diff.y.unsigned_abs()) as i32;
        (diff.x / gcd, diff.y / gcd)
    }

    // direction in degrees (0° is north)
    pub fn direction(&self, other: &Self) -> f32 {
        let (dx, dy) = self.direction_dxdy(other);
        let mut rad = (dy as f32).atan2(dx as f32);
        if rad < 0.0 {
            rad += 2.0 * PI;
        }
        let degrees = rad * 360.0 / (2.0 * PI);
        // rotate clockwise so that up is 0
        (degrees + 90.0) % 360.0
    }

    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        let diff = *other - *self;
        diff.x.unsigned_abs() + diff.y.unsigned_abs()
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// 3d position with x, y and z coordinates
pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl FromStr for Pos3d {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = s.splitn(3, ',');

        let mut next_number = || -> Result<i32, ParseError> {
            numbers
                .next()
                .ok_or_else(|| ParseError(s.to_string()))?
                .trim()
                .parse()
                .map_err(|_| ParseError(s.to_string()))
        };

        Ok(Self {
            x: next_number()?,
            y: next_number()?,
            z: next_number()?,
        })
    }
}

impl Pos3d {
    #[allow(dead_code)]
    // manhattan distance between points
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        let dx = (other.x - self.x).unsigned_abs();
        let dy = (other.y - self.y).unsigned_abs();
        let dz = (other.z - self.z).unsigned_abs();
        dx + dy + dz
    }

    // euclidean distance between points
    #[allow(dead_code)]
    pub fn distance(&self, other: &Self) -> f64 {
        let dx = (other.x - self.x) as f64;
        let dy = (other.y - self.y) as f64;
        let dz = (other.z - self.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    // square of distances between points, useful if distance is for comparison only
    pub fn squared_distance(&self, other: &Self) -> i64 {
        let dx = other.x as i64 - self.x as i64;
        let dy = other.y as i64 - self.y as i64;
        let dz = other.z as i64 - self.z as i64;
        dx * dx + dy * dy + dz * dz
    }
}

impl fmt::Display for Pos3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Pos3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}
