use crate::Part;
use std::fmt;

pub fn run(input: &str, part: Part) -> String {
    let mut grid = parse_input(input);
    if part == Part::Two {
        grid.broken();
    }
    for _ in 0..100 {
        grid = grid.step();
    }
    format!("{}", grid.count())
}

#[derive(Debug)]
struct Grid {
    size: usize,
    // each row is a u128, each bit is a light
    lights: Vec<u128>,
    // true if corners are permanently on (part 2)
    broken: bool,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            size,
            lights: vec![0; size],
            broken: false,
        }
    }

    fn turn_on(&mut self, x: usize, y: usize) {
        self.lights[y] |= 1 << x;
    }

    fn turn_off(&mut self, x: usize, y: usize) {
        self.lights[y] &= !(1 << x);
    }

    fn count(&self) -> u32 {
        self.lights.iter().map(|l| l.count_ones()).sum()
    }

    fn broken(&mut self) {
        self.broken = true;
        self.turn_on(0, 0);
        self.turn_on(0, self.size - 1);
        self.turn_on(self.size - 1, 0);
        self.turn_on(self.size - 1, self.size - 1);
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= self.size as i32 || ny < 0 || ny >= self.size as i32 {
                    continue;
                }
                if self.lights[ny as usize] & (1 << nx) != 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn step(&self) -> Grid {
        let mut new_grid = Self::new(self.size);
        for y in 0..self.size {
            for x in 0..self.size {
                let current = self.lights[y] & (1 << x) != 0;
                match (current, self.count_neighbours(x, y)) {
                    (true, 2) | (true, 3) => new_grid.turn_on(x, y),
                    (true, _) => new_grid.turn_off(x, y),
                    (false, 3) => new_grid.turn_on(x, y),
                    (false, _) => new_grid.turn_off(x, y),
                }
            }
        }
        if self.broken {
            new_grid.broken();
        }
        new_grid
    }
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let mut grid = Self::new(s.lines().count());
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => grid.turn_on(x, y),
                    '.' => grid.turn_off(x, y),
                    _ => panic!("unexpected char: {}", c),
                }
            }
        }
        grid
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.lights {
            for x in 0..self.size {
                write!(f, "{}", if row & (1 << x) != 0 { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(input: &str) -> Grid {
    input.into()
}

#[test]
fn test_part1() {
    let mut grid = parse_input(test_input());
    assert_eq!(15, grid.count());
    let expected = vec![
        "..##..\n..##.#\n...##.\n......\n#.....\n#.##..\n",
        "..###.\n......\n..###.\n......\n.#....\n.#....\n",
        "...#..\n......\n...#..\n..##..\n......\n......\n",
        "......\n......\n..##..\n..##..\n......\n......\n",
    ];
    for e in expected {
        grid = grid.step();
        assert_eq!(e, format!("{}", grid));
    }
    assert_eq!(4, grid.count());
}

#[test]
fn test_part2() {
    let mut grid = parse_input(test_input());
    grid.broken();
    assert_eq!(17, grid.count());
    let expected = vec![
        "#.##.#\n####.#\n...##.\n......\n#...#.\n#.####\n",
        "#..#.#\n#....#\n.#.##.\n...##.\n.#..##\n##.###\n",
        "#...##\n####.#\n..##.#\n......\n##....\n####.#\n",
        "#.####\n#....#\n...#..\n.##...\n#.....\n#.#..#\n",
        "##.###\n.##..#\n.##...\n.##...\n#.#...\n##...#\n",
    ];
    for e in expected {
        grid = grid.step();
        assert_eq!(e, format!("{}", grid));
    }
    assert_eq!(17, grid.count());
}

#[cfg(test)]
fn test_input() -> &'static str {
    ".#.#.#
...##.
#....#
..#...
#.#..#
####.."
}
