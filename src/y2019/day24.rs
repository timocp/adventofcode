use core::fmt;
use std::collections::{HashSet, VecDeque};

use crate::grid::parse_each_char;

pub struct Solver {
    grid: BitGrid,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            grid: BitGrid::from(input),
        }
    }

    fn part1(&self) -> String {
        self.grid.find_repeat().to_string()
    }

    fn part2(&self) -> String {
        let mut infgrid = InfiniteGrid::from(&self.grid);
        for _ in 0..200 {
            infgrid = infgrid.progress();
        }
        infgrid.count_bugs().to_string()
    }
}

// 5x5 grid represented as a u32
// (0,0) is first bit, (1,0) is second bit, ..., (4,4) is 25th bit
#[derive(Clone)]
struct BitGrid {
    data: u32,
}

impl BitGrid {
    fn get(&self, x: i32, y: i32) -> bool {
        self.data & Self::mask(x, y) > 0
    }

    fn count_neighbours(&self, x: i32, y: i32) -> usize {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
            .into_iter()
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(nx, ny)| self.get(*nx, *ny))
            .count()
    }

    fn progress(&self) -> Self {
        let mut data = 0;
        for x in 0..5 {
            for y in 0..5 {
                if match (self.get(x, y), self.count_neighbours(x, y)) {
                    (true, 1) => true,
                    (false, 1 | 2) => true,
                    (_, _) => false,
                } {
                    data += Self::mask(x, y)
                }
            }
        }
        Self { data }
    }

    fn find_repeat(&self) -> u32 {
        let mut seen: HashSet<u32> = HashSet::new();
        let mut grid = self.clone();
        loop {
            let br = grid.biodiversity_rating();
            if seen.contains(&br) {
                return br;
            }
            seen.insert(br);
            grid = grid.progress();
        }
    }

    // storage is already the biodiversity rating
    fn biodiversity_rating(&self) -> u32 {
        self.data
    }

    // returns the bitmask use to access the bit at (x,y)
    // out of bounds returns 0
    // (0,0) = 1 << 0  = 1
    // (1,0) = 1 << 1  = 2
    // (3,0) = 1 << 2  = 4
    // ...
    // (3,4) = 1 << 23 = 8388608
    // (4,4) = 1 << 24 = 16777216
    fn mask(x: i32, y: i32) -> u32 {
        if (0..5).contains(&x) && (0..5).contains(&y) {
            1 << y * 5 + x
        } else {
            0
        }
    }
}

impl From<&str> for BitGrid {
    fn from(s: &str) -> Self {
        let mut data = 0;

        for (p, c) in parse_each_char(s) {
            if c == '#' {
                data += Self::mask(p.x, p.y)
            }
        }

        Self { data }
    }
}

impl fmt::Display for BitGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..5 {
            for x in 0..5 {
                if self.data & Self::mask(x, y) > 0 {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

// For part 2 it is really an infinite grid
// (2,2) is now another inner grid
// outsisde the edges are cells from an outer grid
struct InfiniteGrid {
    grids: VecDeque<BitGrid>,
}

impl InfiniteGrid {
    fn get(&self, depth: i32, x: i32, y: i32) -> bool {
        if depth < 0 || depth as usize >= self.grids.len() {
            false
        } else if x == 2 && y == 2 {
            false
        } else {
            self.grids[depth as usize].get(x, y)
        }
    }

    fn count(&self, depth: i32, x: i32, y: i32) -> i32 {
        match self.get(depth, x, y) {
            true => 1,
            false => 0,
        }
    }

    fn count_neighbours(&self, depth: i32, x: i32, y: i32) -> i32 {
        self.count_north_neighbours(depth, x, y)
            + self.count_east_neighbours(depth, x, y)
            + self.count_south_neighbours(depth, x, y)
            + self.count_west_neighbours(depth, x, y)
    }

    fn count_north_neighbours(&self, depth: i32, x: i32, y: i32) -> i32 {
        match (x, y) {
            (_, 0) => self.count(depth - 1, 2, 1),
            (2, 3) => {
                self.count(depth + 1, 0, 4)
                    + self.count(depth + 1, 1, 4)
                    + self.count(depth + 1, 2, 4)
                    + self.count(depth + 1, 3, 4)
                    + self.count(depth + 1, 4, 4)
            }
            (_, _) => self.count(depth, x, y - 1),
        }
    }

    fn count_east_neighbours(&self, depth: i32, x: i32, y: i32) -> i32 {
        match (x, y) {
            (4, _) => self.count(depth - 1, 3, 2),
            (1, 2) => {
                self.count(depth + 1, 0, 0)
                    + self.count(depth + 1, 0, 1)
                    + self.count(depth + 1, 0, 2)
                    + self.count(depth + 1, 0, 3)
                    + self.count(depth + 1, 0, 4)
            }
            (_, _) => self.count(depth, x + 1, y),
        }
    }

    fn count_south_neighbours(&self, depth: i32, x: i32, y: i32) -> i32 {
        match (x, y) {
            (_, 4) => self.count(depth - 1, 2, 3),
            (2, 1) => {
                self.count(depth + 1, 0, 0)
                    + self.count(depth + 1, 1, 0)
                    + self.count(depth + 1, 2, 0)
                    + self.count(depth + 1, 3, 0)
                    + self.count(depth + 1, 4, 0)
            }
            (_, _) => self.count(depth, x, y + 1),
        }
    }

    fn count_west_neighbours(&self, depth: i32, x: i32, y: i32) -> i32 {
        match (x, y) {
            (0, _) => self.count(depth - 1, 1, 2),
            (3, 2) => {
                self.count(depth + 1, 4, 0)
                    + self.count(depth + 1, 4, 1)
                    + self.count(depth + 1, 4, 2)
                    + self.count(depth + 1, 4, 3)
                    + self.count(depth + 1, 4, 4)
            }
            (_, _) => self.count(depth, x - 1, y),
        }
    }

    fn progress(&self) -> Self {
        let mut grids = VecDeque::new();
        for depth in -1..=(self.grids.len() as i32) {
            let mut data = 0;
            for y in 0..5 {
                for x in 0..5 {
                    if !(x == 2 && y == 2) {
                        if match (self.get(depth, x, y), self.count_neighbours(depth, x, y)) {
                            (true, 1) => true,
                            (false, 1 | 2) => true,
                            (_, _) => false,
                        } {
                            data += BitGrid::mask(x, y);
                        }
                    }
                }
            }
            // ...
            grids.push_back(BitGrid { data });
        }

        // remove empty grids at ends
        while grids.len() > 0 && grids[0].biodiversity_rating() == 0 {
            grids.pop_front();
        }
        while grids.len() > 0 && grids[grids.len() - 1].biodiversity_rating() == 0 {
            grids.pop_back();
        }

        Self { grids }
    }

    fn count_bugs(&self) -> u32 {
        self.grids.iter().map(|g| g.data.count_ones()).sum()
    }
}

impl From<&BitGrid> for InfiniteGrid {
    fn from(grid: &BitGrid) -> Self {
        Self {
            grids: VecDeque::from([grid.clone()]),
        }
    }
}

impl fmt::Display for InfiniteGrid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We don't actually track depth, so for output purposes 0 is always first
        for (depth, grid) in self.grids.iter().enumerate() {
            writeln!(f, "Depth: {}", depth)?;
            let mut s = format!("{}", grid);
            s.replace_range(14..15, "?"); // (2,2) is a deeper grid
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

#[test]
fn test_part1() {
    let test_input = "\
....#
#..#.
#..##
..#..
#....
";

    let expected1 = "\
#..#.
####.
###.#
##.##
.##..
";

    let expected2 = "\
#####
....#
....#
...#.
#.###
";

    let expected3 = "\
#....
####.
...##
#.##.
.##.#
";

    let expected4 = "\
####.
....#
##..#
.....
##...
";

    let grid = BitGrid::from(test_input);
    assert_eq!(test_input, format!("{}", grid));
    let grid = grid.progress();
    assert_eq!(expected1, format!("{}", grid));
    let grid = grid.progress();
    assert_eq!(expected2, format!("{}", grid));
    let grid = grid.progress();
    assert_eq!(expected3, format!("{}", grid));
    let grid = grid.progress();
    assert_eq!(expected4, format!("{}", grid));

    assert_eq!(2129920, BitGrid::from(test_input).find_repeat());
}

#[test]
fn test_part2() {
    let test_input = "\
....#
#..#.
#..##
..#..
#....
";
    let grid = BitGrid::from(test_input);
    let mut infgrid = InfiniteGrid::from(&grid);
    for _ in 0..10 {
        infgrid = infgrid.progress();
    }
    assert_eq!(99, infgrid.count_bugs());
}
