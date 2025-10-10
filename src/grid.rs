use std::fmt;
use std::ops::{Add, Sub};

// xy pair (position) used as index into a grid
// using i32 as index instead of usize because even for positive only,
// some puzzles need to ask for items outside the grid.
// also a future sparse grid may reasonably use negative indexes
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

pub fn parse_each_char(input: &str) -> impl Iterator<Item = (P, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (P::from((x, y)), c))
    })
}

pub struct Grid<T> {
    width: u32,
    height: u32,
    default: T,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Clone,
{
    pub fn new(width: u32, height: u32, default: T) -> Self {
        Self {
            width,
            height,
            default: default.clone(),
            data: vec![default; (width * height) as usize],
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

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn minx(&self) -> i32 {
        0
    }

    pub fn maxx(&self) -> i32 {
        (self.width - 1).try_into().unwrap()
    }

    pub fn miny(&self) -> i32 {
        0
    }

    pub fn maxy(&self) -> i32 {
        (self.height - 1).try_into().unwrap()
    }

    fn index(&self, p: P) -> Option<usize> {
        if p.x < 0 || p.x > self.maxx() && p.y < 0 || p.y > self.maxy() {
            None
        } else {
            Some(p.y as usize * self.width as usize + p.x as usize)
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
            if self.p.x + 1 == self.grid.width as i32 {
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
