use std::collections::HashSet;

use crate::grid::{Compass, Grid};

pub struct Solver {
    grid: Grid<bool>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            grid: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        find_repeat(&self.grid).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn parse_input(input: &str) -> Grid<bool> {
    Grid::from_input(input, false, |c| match c {
        '#' => true,
        '.' => false,
        _ => panic!(),
    })
}

fn progress(grid: &Grid<bool>) -> Grid<bool> {
    let mut new = Grid::new(grid.width(), grid.height(), false);
    for (p, bug) in grid.iter() {
        let neighbours = [
            grid.look(p, Compass::North),
            grid.look(p, Compass::East),
            grid.look(p, Compass::South),
            grid.look(p, Compass::West),
        ]
        .iter()
        .filter(|v| ***v)
        .count();
        new.set(
            p,
            match (bug, neighbours) {
                (true, 1) => true,
                (true, _) => false,
                (false, 1 | 2) => true,
                (false, _) => false,
            },
        )
    }
    new
}

#[allow(dead_code)]
fn grid_to_string(grid: &Grid<bool>) -> String {
    let mut s = String::new();
    for (p, bug) in grid.iter() {
        s.push(match bug {
            true => '#',
            false => '.',
        });
        if p.x == grid.maxx() {
            s.push('\n');
        }
    }
    s
}

fn biodiversity_rating(grid: &Grid<bool>) -> usize {
    grid.iter()
        .enumerate()
        .filter_map(|(i, (_, bug))| if *bug { Some(1 << i) } else { None })
        .sum()
}

fn find_repeat(grid: &Grid<bool>) -> usize {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut grid = grid.clone();
    loop {
        let br = biodiversity_rating(&grid);
        if seen.contains(&br) {
            return br;
        }
        seen.insert(br);
        grid = progress(&grid);
    }
}

#[test]
fn test() {
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

    let grid = parse_input(test_input);
    assert_eq!(test_input, grid_to_string(&grid));
    let grid = progress(&grid);
    assert_eq!(expected1, grid_to_string(&grid));
    let grid = progress(&grid);
    assert_eq!(expected2, grid_to_string(&grid));
    let grid = progress(&grid);
    assert_eq!(expected3, grid_to_string(&grid));
    let grid = progress(&grid);
    assert_eq!(expected4, grid_to_string(&grid));

    assert_eq!(2129920, find_repeat(&parse_input(test_input)));

    let test_input = "\
.....
.....
.....
#....
.#...
";

    assert_eq!(2129920, biodiversity_rating(&parse_input(test_input)));
}
