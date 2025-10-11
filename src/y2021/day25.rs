use crate::grid::{Compass, Grid, P};
use std::fmt;

pub struct Solver {
    input: String,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }

    fn part1(&self) -> String {
        part1(parse_input(&self.input)).to_string()
    }

    fn part2(&self) -> String {
        "N/A".to_string()
    }
}

fn part1(mut map: Map) -> usize {
    let mut step = 1;

    // println!("Initial state:\n{}", map);
    while map.step() {
        // println!("After {} steps:\n{}", step, map);
        step += 1;
    }
    // println!("After {} steps:\n{}", step, map);
    step
}

#[derive(Clone, PartialEq)]
enum Cucumber {
    EastFacing,
    SouthFacing,
    Empty,
}

impl From<char> for Cucumber {
    fn from(c: char) -> Self {
        match c {
            '>' => Self::EastFacing,
            'v' => Self::SouthFacing,
            '.' => Self::Empty,
            _ => panic!("unexpected input: {}", c),
        }
    }
}

struct Map {
    grid: Grid<Cucumber>,
}

impl Map {
    // returns true if any cucumbers moved this step
    fn step(&mut self) -> bool {
        let mut changed = false;

        // east facing herd moves first
        let mut mvlist: Vec<(P, P)> = vec![];
        for (p, c) in self.grid.iter() {
            if matches!(c, Cucumber::EastFacing) {
                let east = self.grid.wrapped_pos(p, Compass::East);
                if *self.grid.get(east) == Cucumber::Empty {
                    mvlist.push((p, east));
                    changed = true;
                }
            }
        }
        for (from, to) in mvlist {
            self.grid.set(from, Cucumber::Empty);
            self.grid.set(to, Cucumber::EastFacing);
        }

        // south facing herd moves next
        let mut mvlist: Vec<(P, P)> = vec![];
        for (p, c) in self.grid.iter() {
            if matches!(c, Cucumber::SouthFacing) {
                let south = self.grid.wrapped_pos(p, Compass::South);
                if *self.grid.get(south) == Cucumber::Empty {
                    mvlist.push((p, south));
                    changed = true;
                }
            }
        }
        for (from, to) in mvlist {
            self.grid.set(from, Cucumber::Empty);
            self.grid.set(to, Cucumber::SouthFacing);
        }

        changed
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ef = 0;
        let mut sf = 0;
        for (p, c) in self.grid.iter() {
            write!(
                f,
                "{}",
                match c {
                    Cucumber::EastFacing => {
                        ef += 1;
                        '>'
                    }
                    Cucumber::SouthFacing => {
                        sf += 1;
                        'v'
                    }
                    Cucumber::Empty => '.',
                }
            )?;
            if p.x >= self.grid.maxx() {
                writeln!(f)?;
            }
        }
        writeln!(f, "{} EF, {} SF", ef, sf)?;
        Ok(())
    }
}

fn parse_input(input: &str) -> Map {
    Map {
        grid: Grid::from_input(input, Cucumber::Empty, Cucumber::from),
    }
}

#[test]
fn test() {
    let test_input = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";
    assert_eq!(58, part1(parse_input(test_input)));
}
