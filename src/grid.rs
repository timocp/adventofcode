use gcd::Gcd;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::fmt;
use std::ops::{Add, Sub};

// xy pair (position) used as index into a grid
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct P {
    pub x: i32,
    pub y: i32,
}

pub const ORIGIN: P = P { x: 0, y: 0 };

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

// Compass directions
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Compass {
    pub fn left90(&self) -> Self {
        match self {
            Compass::North => Compass::West,
            Compass::NorthEast => Compass::NorthWest,
            Compass::East => Compass::North,
            Compass::SouthEast => Compass::NorthEast,
            Compass::South => Compass::East,
            Compass::SouthWest => Compass::SouthEast,
            Compass::West => Compass::South,
            Compass::NorthWest => Compass::SouthWest,
        }
    }

    pub fn right90(&self) -> Self {
        match self {
            Compass::North => Compass::East,
            Compass::NorthEast => Compass::SouthEast,
            Compass::East => Compass::South,
            Compass::SouthEast => Compass::SouthWest,
            Compass::South => Compass::West,
            Compass::SouthWest => Compass::NorthWest,
            Compass::West => Compass::North,
            Compass::NorthWest => Compass::NorthEast,
        }
    }
}

impl P {
    pub fn step(&self, dir: Compass) -> Self {
        match dir {
            Compass::North => P {
                x: self.x,
                y: self.y - 1,
            },
            Compass::NorthEast => P {
                x: self.x + 1,
                y: self.y - 1,
            },
            Compass::East => P {
                x: self.x + 1,
                y: self.y,
            },
            Compass::SouthEast => P {
                x: self.x + 1,
                y: self.y + 1,
            },
            Compass::South => P {
                x: self.x,
                y: self.y + 1,
            },
            Compass::SouthWest => P {
                x: self.x - 1,
                y: self.y + 1,
            },
            Compass::West => P {
                x: self.x - 1,
                y: self.y,
            },
            Compass::NorthWest => P {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }

    // direction as (dx, dy), normalised by dividing by gcd
    pub fn direction_dxdy(&self, other: &Self) -> (i32, i32) {
        if self == other {
            panic!("Attemted to calculate direction from {} to itself", self);
        }
        let diff = *other - *self;
        let gcd = diff.x.unsigned_abs().gcd(diff.y.unsigned_abs()) as i32;
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

pub fn parse_each_char(input: &str) -> impl Iterator<Item = (P, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (P::from((x, y)), c))
    })
}

// 2d grid with fixed width and height
// top-left / NW corner is (0, 0)
// setting outside of bounds panics
// reading outside of bounds returns default
pub struct Grid<T> {
    maxx: i32,
    maxy: i32,
    default: T,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    #[allow(dead_code)]
    pub fn new(width: u32, height: u32, default: T) -> Self {
        Self {
            maxx: width as i32 - 1,
            maxy: height as i32 - 1,
            default: default.clone(),
            data: vec![default; (width * height) as usize],
        }
    }

    // input -> Grid using a function that converts char to T
    pub fn from_input(input: &str, default: T, from_char: fn(char) -> T) -> Self {
        Self::from_input_by(input, default, |_p, c| from_char(c))
    }

    // input -> Grid using a closure that converts (P, char) to T
    pub fn from_input_by<F>(input: &str, default: T, mut from_char: F) -> Self
    where
        F: FnMut(P, char) -> T,
    {
        let mut last_p = ORIGIN;
        let mut data = vec![];
        for (p, c) in parse_each_char(input) {
            data.push(from_char(p, c));
            last_p = p;
        }
        Grid {
            maxx: last_p.x,
            maxy: last_p.y,
            default: default.clone(),
            data,
        }
    }

    pub fn get(&self, p: P) -> &T {
        if let Some(i) = self.index(p) {
            &self.data[i]
        } else {
            &self.default
        }
    }

    pub fn set(&mut self, p: P, v: T) {
        if let Some(i) = self.index(p) {
            self.data[i] = v;
        } else {
            panic!("attempted to set out of bounds at {}", p);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (P, &T)> {
        GridIter {
            grid: self,
            p: P { x: 0, y: 0 },
        }
    }

    #[allow(dead_code)]
    pub fn width(&self) -> u32 {
        (self.maxx + 1).try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn height(&self) -> u32 {
        (self.maxy + 1).try_into().unwrap()
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[allow(dead_code)]
    pub fn minx(&self) -> i32 {
        0
    }

    pub fn maxx(&self) -> i32 {
        self.maxx
    }

    #[allow(dead_code)]
    pub fn miny(&self) -> i32 {
        0
    }

    #[allow(dead_code)]
    pub fn maxy(&self) -> i32 {
        self.maxy
    }

    // get neighbouring position (None if it would move off grid)
    #[allow(dead_code)]
    pub fn bounded_pos(&self, p: P, dir: Compass) -> Option<P> {
        let p2 = p.step(dir);
        if p2.x < 0 || p2.x > self.maxx || p2.y < 0 || p2.y > self.maxy {
            None
        } else {
            Some(p2)
        }
    }

    // get neightbouring position (wrap around edges of grid)
    pub fn wrapped_pos(&self, p: P, dir: Compass) -> P {
        let mut p2 = p.step(dir);
        if p2.x < 0 {
            p2.x = self.maxx;
        } else if p2.x > self.maxx {
            p2.x = 0;
        }
        if p2.y < 0 {
            p2.y = self.maxy;
        } else if p2.y > self.maxy {
            p2.y = 0;
        }
        p2
    }

    pub fn look(&self, p: P, dir: Compass) -> &T {
        self.get(p.step(dir))
    }

    fn index(&self, p: P) -> Option<usize> {
        if p.x < 0 || p.x > self.maxx || p.y < 0 || p.y > self.maxy {
            None
        } else {
            Some((p.y * (self.maxx + 1) + p.x) as usize)
        }
    }
}

struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    p: P,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Clone,
{
    type Item = (P, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.grid.index(self.p) {
            let p = self.p;
            if self.p.x == self.grid.maxx {
                self.p.y += 1;
                self.p.x = 0;
            } else {
                self.p.x += 1;
            }
            Some((p, &self.grid.data[i]))
        } else {
            None
        }
    }
}

// 2d grid without fixed width and height
// unbounded, indexes may be negative
// reading a cell that hasn't been set returns default
// interface is as close to Grid as possible
pub struct SparseGrid<T> {
    nw: P, // NW or top-left corner
    se: P, // SE or bottom-right corner
    default: T,
    data: HashMap<P, T>,
}

impl<T> SparseGrid<T>
where
    T: Clone,
{
    pub fn new(default: T) -> Self {
        Self {
            nw: ORIGIN,
            se: ORIGIN,
            default: default.clone(),
            data: HashMap::new(),
        }
    }

    pub fn get(&self, p: P) -> &T {
        if let Some(t) = self.data.get(&p) {
            t
        } else {
            &self.default
        }
    }

    pub fn set(&mut self, p: P, v: T) {
        if self.data.is_empty() {
            self.nw = p;
            self.se = p;
        } else {
            if p.x < self.nw.x {
                self.nw.x = p.x;
            } else if p.x > self.se.x {
                self.se.x = p.x;
            }
            if p.y < self.nw.y {
                self.nw.y = p.y;
            } else if p.y > self.se.y {
                self.se.y = p.y;
            }
        }
        self.data.insert(p, v);
    }

    pub fn width(&self) -> u32 {
        if self.data.is_empty() {
            0
        } else {
            (self.se.x - self.nw.x + 1).try_into().unwrap()
        }
    }

    pub fn height(&self) -> u32 {
        if self.data.is_empty() {
            0
        } else {
            (self.se.y - self.nw.y + 1).try_into().unwrap()
        }
    }

    pub fn len(&self) -> u32 {
        self.data.len().try_into().unwrap()
    }

    // min/max functions return 0 if the grid is empty (not accurate but simpler)

    pub fn minx(&self) -> i32 {
        self.nw.x
    }

    pub fn maxx(&self) -> i32 {
        self.se.x
    }

    pub fn miny(&self) -> i32 {
        self.nw.y
    }

    pub fn maxy(&self) -> i32 {
        self.se.y
    }

    pub fn look(&self, p: P, dir: Compass) -> &T {
        self.get(p.step(dir))
    }
}
