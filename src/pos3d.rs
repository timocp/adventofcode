use std::fmt;
use std::str::FromStr;

pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug)]
pub struct ParseError(String);

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

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "failed to parse position: {:?}", self.0)
    }
}

impl std::error::Error for ParseError {}

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
