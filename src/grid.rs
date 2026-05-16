use crate::compass::Compass;
use crate::pos::{ORIGIN, Pos};
use std::collections::HashMap;

pub fn parse_each_char(input: &str) -> impl Iterator<Item = (Pos, char)> + '_ {
    input.lines().enumerate().flat_map(|(y, line)| {
        line.chars()
            .enumerate()
            .map(move |(x, c)| (Pos::from((x, y)), c))
    })
}

// 2d grid with fixed width and height
// top-left / NW corner is (0, 0)
// setting outside of bounds panics
// reading outside of bounds returns default
#[derive(Clone)]
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

    // input -> Grid using a closure that converts (Pos, char) to T
    pub fn from_input_by<F>(input: &str, default: T, mut from_char: F) -> Self
    where
        F: FnMut(Pos, char) -> T,
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

    pub fn get(&self, p: Pos) -> &T {
        if let Some(i) = self.index(p) {
            &self.data[i]
        } else {
            &self.default
        }
    }

    pub fn set(&mut self, p: Pos, v: T) {
        if let Some(i) = self.index(p) {
            self.data[i] = v;
        } else {
            panic!("attempted to set out of bounds at {}", p);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Pos, &T)> {
        GridIter {
            grid: self,
            p: Pos { x: 0, y: 0 },
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
    pub fn bounded_pos(&self, p: Pos, dir: Compass) -> Option<Pos> {
        let p2 = p.step(dir);
        if p2.x < 0 || p2.x > self.maxx || p2.y < 0 || p2.y > self.maxy {
            None
        } else {
            Some(p2)
        }
    }

    // get neightbouring position (wrap around edges of grid)
    pub fn wrapped_pos(&self, p: Pos, dir: Compass) -> Pos {
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

    pub fn look(&self, p: Pos, dir: Compass) -> &T {
        self.get(p.step(dir))
    }

    fn index(&self, p: Pos) -> Option<usize> {
        if p.x < 0 || p.x > self.maxx || p.y < 0 || p.y > self.maxy {
            None
        } else {
            Some((p.y * (self.maxx + 1) + p.x) as usize)
        }
    }
}

struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    p: Pos,
}

impl<'a, T> Iterator for GridIter<'a, T>
where
    T: Clone,
{
    type Item = (Pos, &'a T);

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
    nw: Pos, // NW or top-left corner
    se: Pos, // SE or bottom-right corner
    default: T,
    data: HashMap<Pos, T>,
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

    pub fn get(&self, p: Pos) -> &T {
        if let Some(t) = self.data.get(&p) {
            t
        } else {
            &self.default
        }
    }

    pub fn set(&mut self, p: Pos, v: T) {
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

    pub fn iter(&self) -> impl Iterator<Item = (&Pos, &T)> {
        self.data.iter()
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

    #[allow(dead_code)]
    pub fn maxx(&self) -> i32 {
        self.se.x
    }

    pub fn miny(&self) -> i32 {
        self.nw.y
    }

    #[allow(dead_code)]
    pub fn maxy(&self) -> i32 {
        self.se.y
    }

    #[allow(dead_code)]
    pub fn look(&self, p: Pos, dir: Compass) -> &T {
        self.get(p.step(dir))
    }
}
