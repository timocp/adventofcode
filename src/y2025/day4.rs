use crate::grid::{ALL_DIRS, Grid, Pos};

pub fn part1(grid: &Grid<Cell>) -> usize {
    accessible_paper(grid).len()
}

pub fn part2(grid: &Grid<Cell>) -> usize {
    total_removeable_paper(grid)
}

fn accessible_paper(grid: &Grid<Cell>) -> Vec<Pos> {
    let mut positions = vec![];
    for (pos, cell) in grid.iter() {
        if matches!(cell, Cell::Paper)
            && ALL_DIRS
                .iter()
                .filter(|dir| matches!(grid.look(pos, **dir), Cell::Paper))
                .count()
                < 4
        {
            positions.push(pos);
        }
    }
    positions
}

// Removes accesible paper from the grid, returning the number of rolls that were removed
fn remove_paper(grid: &mut Grid<Cell>) -> usize {
    let accessible = accessible_paper(grid);
    for p in &accessible {
        grid.set(*p, Cell::Empty);
    }
    accessible.len()
}

fn total_removeable_paper(grid: &Grid<Cell>) -> usize {
    let mut grid = grid.clone();
    let mut total = 0;
    loop {
        let removed = remove_paper(&mut grid);
        if removed == 0 {
            return total;
        }
        total += removed;
    }
}

pub fn parse_input(input: &str) -> Grid<Cell> {
    Grid::from_input(input, Cell::Empty, Cell::from)
}

#[derive(Clone)]
pub enum Cell {
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
    assert_eq!(13, accessible_paper(&grid).len());
    assert_eq!(43, total_removeable_paper(&grid));
}
