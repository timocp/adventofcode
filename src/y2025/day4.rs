use crate::grid::{ALL_DIRS, Grid};

pub struct Solver {
    grid: Grid<Cell>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            grid: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        accessible_rolls(&self.grid).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn accessible_rolls(grid: &Grid<Cell>) -> u32 {
    let mut count = 0;
    for (pos, cell) in grid.iter() {
        if matches!(cell, Cell::Paper)
            && ALL_DIRS
                .iter()
                .filter(|dir| matches!(grid.look(pos, **dir), Cell::Paper))
                .count()
                < 4
        {
            count += 1;
        }
    }
    count
}

fn parse_input(input: &str) -> Grid<Cell> {
    Grid::from_input(input, Cell::Empty, Cell::from)
}

#[derive(Clone)]
enum Cell {
    Empty,
    Paper,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '@' => Self::Paper,
            _ => panic!("unexpected input: {}", c),
        }
    }
}

#[test]
fn test() {
    let test_input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    let grid = parse_input(test_input);
    assert_eq!(13, accessible_rolls(&grid));
}
