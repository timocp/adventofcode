use crate::grid::Compass::*;
use crate::grid::{Grid, Pos};
use std::collections::HashSet;

pub fn parse_input(input: &str) -> Grid<u8> {
    Grid::from_input(input, 9, char_to_u8)
}

pub fn part1(grid: &Grid<u8>) -> u32 {
    grid.part1()
}

pub fn part2(grid: &Grid<u8>) -> usize {
    grid.part2()
}

impl Grid<u8> {
    fn lowpoints(&self) -> impl Iterator<Item = Pos> {
        self.iter().filter_map(|(p, value)| {
            if self.look(p, North) > value
                && self.look(p, East) > value
                && self.look(p, South) > value
                && self.look(p, West) > value
            {
                Some(p)
            } else {
                None
            }
        })
    }

    fn part1(&self) -> u32 {
        self.lowpoints().map(|p| (*self.get(p) + 1) as u32).sum()
    }

    // recursively measure the size of a basin, including this point
    fn basin_size(&self, from: Pos, seen: &mut HashSet<Pos>) -> usize {
        seen.insert(from);
        let mut size = 1;
        let value = self.get(from);
        for dir in [North, East, South, West] {
            let to = from.step(dir);
            let other = self.get(to);
            if other >= value && *other != 9 && !seen.contains(&to) {
                size += self.basin_size(to, seen);
            }
        }
        size
    }

    fn part2(&self) -> usize {
        let mut seen: HashSet<Pos> = HashSet::new();
        let mut basins: Vec<usize> = vec![];
        for start in self.lowpoints() {
            basins.push(self.basin_size(start, &mut seen));
        }
        basins.sort_unstable();
        basins.iter().rev().take(3).product()
    }
}

fn char_to_u8(c: char) -> u8 {
    if c.is_ascii_digit() {
        (c as u8) - 48
    } else {
        panic!("Unexpected character: {}", c);
    }
}

#[test]
fn test() {
    let test_input = "\
2199943210
3987894921
9856789892
8767896789
9899965678
";
    let map = Grid::from_input(&test_input, 9, char_to_u8);
    assert_eq!(15, map.part1());
    assert_eq!(1134, map.part2());
}
