use crate::{
    bfs,
    grid::{Compass, Compass::*, Grid, Pos},
};
use Cell::*;
use std::collections::HashMap;

pub struct Solver {
    maze: Maze,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            maze: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        solve(&self.maze).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn parse_input(input: &str) -> Maze {
    let mut grid = Grid::from_input(input, Empty, Cell::from);
    let mut entrance: Option<Pos> = None;
    let mut exit: Option<Pos> = None;

    // Map of labels to a vec of (entrance, local_exit) pair
    let mut warp_markers: HashMap<(char, char), Vec<(Pos, Pos)>> = HashMap::new();

    // iterate over each marker.  Record entrance, exit and each warp with its local exit
    let each_pos: Vec<Pos> = grid.iter().map(|(p, _cell)| p).collect();
    for p in each_pos {
        match grid.get(p) {
            Label(c) => {
                let (p2, c2, warp, target) = assess_marker(&grid, p);
                if *c == 'A' && c2 == 'A' {
                    grid.set(p, Empty);
                    grid.set(p2, Empty);
                    grid.set(target, Entrance);
                    entrance = Some(target);
                } else if *c == 'Z' && c2 == 'Z' {
                    grid.set(p, Empty);
                    grid.set(p2, Empty);
                    grid.set(target, Exit);
                    exit = Some(target);
                } else {
                    warp_markers
                        .entry((*c, c2))
                        .and_modify(|v| v.push((warp, target)))
                        .or_insert(vec![(warp, target)]);
                    grid.set(p, Empty);
                    grid.set(p2, Empty);
                }
            }
            _ => {}
        }
    }

    // Each label should have exactly 2 (entrace,local_exits) pairs.
    // Insert them into the maze, swapping local exits so that they become warp exits.
    for (_label, v) in warp_markers.iter() {
        grid.set(v[0].0, Warp(v[1].1));
        grid.set(v[1].0, Warp(v[0].1));
    }

    Maze {
        grid,
        entrance: entrance.unwrap(),
        exit: exit.unwrap(),
    }
}

// For the first character of each label, work out where the other character is
// and what they point to in the maze.
// We are parsing L-R and T-B, so the other marker will always be East or South.
//
// returns:
// (
//   position of the other character
//   the other character
//   position of which char is immediately outside the maze (a warp entrance)
//   position of which char is adjacent and inside the maze (a warp exit)
// )
fn assess_marker(grid: &Grid<Cell>, p: Pos) -> (Pos, char, Pos, Pos) {
    if let Label(c2) = grid.look(p, East) {
        if let Empty = grid.look(p, West) {
            (p.step(East), *c2, p.step(East), p.step(East).step(East))
        } else {
            (p.step(East), *c2, p, p.step(West))
        }
    } else if let Label(c2) = grid.look(p, South) {
        if let Empty = grid.look(p, North) {
            (p.step(South), *c2, p.step(South), p.step(South).step(South))
        } else {
            (p.step(South), *c2, p, p.step(North))
        }
    } else {
        panic!("Malformed marker at {}", p);
    }
}

#[derive(Clone, Debug)]
enum Cell {
    Empty,
    Wall,
    Passage,
    Entrance,
    Exit,
    Label(char),
    Warp(Pos),
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            ' ' => Self::Empty,
            '#' => Self::Wall,
            '.' => Self::Passage,
            'A'..='Z' => Self::Label(c),
            _ => panic!("unexpected input: {}", c),
        }
    }
}

struct Maze {
    grid: Grid<Cell>,
    entrance: Pos,
    exit: Pos,
}

const NEIGHBOURS: [Compass; 4] = [North, East, South, West];

fn solve(maze: &Maze) -> usize {
    bfs::search(
        &maze.entrance,
        |p| {
            NEIGHBOURS
                .iter()
                .filter_map(|dir| {
                    let p2 = p.step(*dir);
                    match maze.grid.get(p2) {
                        Wall => None,
                        Passage => Some((p2, dir)),
                        Exit => Some((p2, dir)),
                        Warp(to) => Some((*to, dir)),
                        _ => None,
                    }
                })
                .collect::<Vec<_>>()
        },
        |p| *p == maze.exit,
    )
    .unwrap()
    .len()
}

#[test]
fn test() {
    let example1 = [
        "         A           \n",
        "         A           \n",
        "  #######.#########  \n",
        "  #######.........#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #######.#######.#  \n",
        "  #####  B    ###.#  \n",
        "BC...##  C    ###.#  \n",
        "  ##.##       ###.#  \n",
        "  ##...DE  F  ###.#  \n",
        "  #####    G  ###.#  \n",
        "  #########.#####.#  \n",
        "DE..#######...###.#  \n",
        "  #.#########.###.#  \n",
        "FG..#########.....#  \n",
        "  ###########.#####  \n",
        "             Z       \n",
        "             Z       \n",
    ]
    .join("");
    let maze = parse_input(&example1);
    assert_eq!(23, solve(&maze));

    let example2 = [
        "                   A               \n",
        "                   A               \n",
        "  #################.#############  \n",
        "  #.#...#...................#.#.#  \n",
        "  #.#.#.###.###.###.#########.#.#  \n",
        "  #.#.#.......#...#.....#.#.#...#  \n",
        "  #.#########.###.#####.#.#.###.#  \n",
        "  #.............#.#.....#.......#  \n",
        "  ###.###########.###.#####.#.#.#  \n",
        "  #.....#        A   C    #.#.#.#  \n",
        "  #######        S   P    #####.#  \n",
        "  #.#...#                 #......VT\n",
        "  #.#.#.#                 #.#####  \n",
        "  #...#.#               YN....#.#  \n",
        "  #.###.#                 #####.#  \n",
        "DI....#.#                 #.....#  \n",
        "  #####.#                 #.###.#  \n",
        "ZZ......#               QG....#..AS\n",
        "  ###.###                 #######  \n",
        "JO..#.#.#                 #.....#  \n",
        "  #.#.#.#                 ###.#.#  \n",
        "  #...#..DI             BU....#..LF\n",
        "  #####.#                 #.#####  \n",
        "YN......#               VT..#....QG\n",
        "  #.###.#                 #.###.#  \n",
        "  #.#...#                 #.....#  \n",
        "  ###.###    J L     J    #.#.###  \n",
        "  #.....#    O F     P    #.#...#  \n",
        "  #.###.#####.#.#####.#####.###.#  \n",
        "  #...#.#.#...#.....#.....#.#...#  \n",
        "  #.#####.###.###.#.#.#########.#  \n",
        "  #...#.#.....#...#.#.#.#.....#.#  \n",
        "  #.###.#####.###.###.#.#.#######  \n",
        "  #.#.........#...#.............#  \n",
        "  #########.###.###.#############  \n",
        "           B   J   C               \n",
        "           U   P   P               \n",
    ]
    .join("");
    let maze = parse_input(&example2);
    assert_eq!(58, solve(&maze));
}
