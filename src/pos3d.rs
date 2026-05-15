use std::fmt;

pub struct Pos3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<&str> for Pos3d {
    fn from(s: &str) -> Self {
        let mut numbers = s.splitn(3, ',');
        let x = numbers.next().unwrap().parse().unwrap();
        let y = numbers.next().unwrap().parse().unwrap();
        let z = numbers.next().unwrap().parse().unwrap();
        Self { x, y, z }
    }
}

impl Pos3d {
    #[allow(dead_code)]
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

    // If distance is only for comparisons, sqrt and floating point is not needed
    pub fn distance_cmp(&self, other: &Self) -> i64 {
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
