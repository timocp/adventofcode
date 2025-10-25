use super::intcode::Vm;
use crate::bfs;
use crate::grid::{Compass, ORIGIN, Pos, SparseGrid};

pub struct Solver {
    // pre-explored map of the ship
    ship: SparseGrid<Cell>,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            ship: explore_ship(Vm::from(input)),
        }
    }

    fn part1(&self) -> String {
        bfs::search(
            &ORIGIN,
            |p| next_steps(&self.ship, p),
            |p| *self.ship.get(*p) == Cell::Oxygen,
        )
        .unwrap()
        .len()
        .to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

#[derive(Clone, PartialEq)]
enum Cell {
    Unknown,
    Empty,
    Wall,
    Oxygen,
}

fn next_steps(ship: &SparseGrid<Cell>, from: &Pos) -> Vec<(Pos, Compass)> {
    [
        (from.step(Compass::North), Compass::North),
        (from.step(Compass::South), Compass::South),
        (from.step(Compass::West), Compass::West),
        (from.step(Compass::East), Compass::East),
    ]
    .into_iter()
    .filter(|(p, _d)| *ship.get(*p) != Cell::Wall)
    .collect()
}

// Explore ship
// Use BFS to find an unexplored cell, control the robot to find out what's there
// Repeat until no unknown cells
fn explore_ship(mut program: Vm) -> SparseGrid<Cell> {
    let mut ship = SparseGrid::new(Cell::Unknown);
    ship.set(ORIGIN, Cell::Empty);

    let mut robot = ORIGIN;

    while let Some(path) = bfs::search(
        &robot,
        |p| next_steps(&ship, p),
        |p| *ship.get(*p) == Cell::Unknown,
    ) {
        for (pos, dir) in path {
            let command = match dir {
                Compass::North => 1,
                Compass::South => 2,
                Compass::West => 3,
                Compass::East => 4,
                _ => panic!(),
            };
            let output = program.run(&[command]).into_iter().next().unwrap();
            match output {
                0 => {
                    // Robot hit a wall
                    ship.set(pos, Cell::Wall);
                }
                1 => {
                    // Robot moved into an empty space
                    ship.set(pos, Cell::Empty);
                    robot = pos;
                }
                2 => {
                    // Robot moved into the oxygen square
                    ship.set(pos, Cell::Oxygen);
                    robot = pos;
                }
                _ => panic!(),
            }
        }
    }
    // println!("{}", draw_ship(&ship, robot));

    ship
}

#[allow(dead_code)]
fn draw_ship(ship: &SparseGrid<Cell>, robot: Pos) -> String {
    let mut output = "".to_string();
    for y in ship.miny()..=ship.maxy() {
        for x in ship.minx()..=ship.maxx() {
            let pos = (x, y).into();
            output.push(if pos == robot {
                'R'
            } else if pos == ORIGIN {
                '*'
            } else {
                match ship.get(pos) {
                    Cell::Unknown => '·',
                    Cell::Wall => '█',
                    Cell::Oxygen => 'O',
                    Cell::Empty => ' ',
                }
            });
        }
        output.push('\n');
    }
    output
}
