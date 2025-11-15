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
        solve(&self.maze, false).unwrap().to_string()
    }

    fn part2(&self) -> String {
        solve(&self.maze, true).unwrap().to_string()
    }
}

fn parse_input(input: &str) -> Maze {
    let mut grid = Grid::from_input(input, Empty, Cell::from);
    let mut entrance: Option<Pos> = None;
    let mut exit: Option<Pos> = None;

    // Map of labels to a vec of (entrance, local_exit) pair
    let mut warp_markers: HashMap<(char, char), Vec<(Pos, Pos)>> = HashMap::new();

    let mut top_left_corner: Option<Pos> = None;
    let mut bottom_right_corner: Option<Pos> = None;

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
            Wall => {
                // track these corners to determine if warps are outer/inner
                if top_left_corner.is_none() {
                    top_left_corner = Some(p);
                }
                bottom_right_corner = Some(p);
            }
            _ => {}
        }
    }

    // lines containing outer portals
    let outer_left_x = top_left_corner.unwrap().x - 1;
    let outer_right_x = bottom_right_corner.unwrap().x + 1;
    let outer_top_y = top_left_corner.unwrap().y - 1;
    let outer_bottom_y = bottom_right_corner.unwrap().y + 1;

    // Each label should have exactly 2 (entrace,local_exits) pairs.
    // Insert them into the maze, swapping local exits so that they become warp exits.
    for (_label, v) in warp_markers.iter() {
        let p = v[0].0;
        if p.x == outer_left_x
            || p.x == outer_right_x
            || p.y == outer_top_y
            || p.y == outer_bottom_y
        {
            // the first pair is outer
            grid.set(v[0].0, OuterPortal(v[1].1));
            grid.set(v[1].0, InnerPortal(v[0].1));
        } else {
            // the first pair is inner
            grid.set(v[0].0, InnerPortal(v[1].1));
            grid.set(v[1].0, OuterPortal(v[0].1));
        }
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
    Label(char), // temporary during parsing
    InnerPortal(Pos),
    OuterPortal(Pos),
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

fn solve(maze: &Maze, recursive: bool) -> Option<usize> {
    let path = bfs::search(
        &(maze.entrance, 0),
        |(p, level)| {
            if recursive {
                recursive_neighbours(maze, p, level)
            } else {
                non_recursive_neighbours(maze, p)
            }
        },
        |(p, level)| *level == 0 && *p == maze.exit,
    );
    path.map(|v| v.len())
}

// original version, any portal warps to its companion with the same name
fn non_recursive_neighbours(maze: &Maze, p: &Pos) -> Vec<((Pos, u32), Compass)> {
    NEIGHBOURS
        .iter()
        .filter_map(|dir| {
            let p2 = p.step(*dir);
            match maze.grid.get(p2) {
                Wall => None,
                Passage => Some(((p2, 0), *dir)),
                Exit => Some(((p2, 0), *dir)),
                InnerPortal(to) | OuterPortal(to) => Some(((*to, 0), *dir)),
                _ => None,
            }
        })
        .collect()
}

// inner warps are now to a deeper copy of the maze,
// outer warps are back to the previous copy
// initial maze is outermost (level=0); warps do not lead anywhere
// entrance and exit only exist on the outermost maze.
fn recursive_neighbours(maze: &Maze, p: &Pos, level: &u32) -> Vec<((Pos, u32), Compass)> {
    NEIGHBOURS
        .iter()
        .filter_map(|dir| {
            let p2 = p.step(*dir);
            match maze.grid.get(p2) {
                Wall => None,
                Passage => Some(((p2, *level), *dir)),
                Exit => Some(((p2, *level), *dir)),
                InnerPortal(to) => {
                    // A level limit is needed to avoid infinite recursion when there's
                    // no path (like test example 2)
                    if *level < 25 {
                        Some(((*to, level + 1), *dir))
                    } else {
                        None
                    }
                }
                OuterPortal(to) => {
                    if *level > 0 {
                        Some(((*to, level - 1), *dir))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .collect()
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
    assert_eq!(Some(23), solve(&maze, false));
    assert_eq!(Some(26), solve(&maze, true));

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
    assert_eq!(Some(58), solve(&maze, false));
    assert_eq!(None, solve(&maze, true));

    let example3 = [
        "             Z L X W       C                 \n",
        "             Z P Q B       K                 \n",
        "  ###########.#.#.#.#######.###############  \n",
        "  #...#.......#.#.......#.#.......#.#.#...#  \n",
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n",
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n",
        "  #.###.#######.###.###.#.###.###.#.#######  \n",
        "  #...#.......#.#...#...#.............#...#  \n",
        "  #.#########.#######.#.#######.#######.###  \n",
        "  #...#.#    F       R I       Z    #.#.#.#  \n",
        "  #.###.#    D       E C       H    #.#.#.#  \n",
        "  #.#...#                           #...#.#  \n",
        "  #.###.#                           #.###.#  \n",
        "  #.#....OA                       WB..#.#..ZH\n",
        "  #.###.#                           #.#.#.#  \n",
        "CJ......#                           #.....#  \n",
        "  #######                           #######  \n",
        "  #.#....CK                         #......IC\n",
        "  #.###.#                           #.###.#  \n",
        "  #.....#                           #...#.#  \n",
        "  ###.###                           #.#.#.#  \n",
        "XF....#.#                         RF..#.#.#  \n",
        "  #####.#                           #######  \n",
        "  #......CJ                       NM..#...#  \n",
        "  ###.#.#                           #.###.#  \n",
        "RE....#.#                           #......RF\n",
        "  ###.###        X   X       L      #.#.#.#  \n",
        "  #.....#        F   Q       P      #.#.#.#  \n",
        "  ###.###########.###.#######.#########.###  \n",
        "  #.....#...#.....#.......#...#.....#.#...#  \n",
        "  #####.#.###.#######.#######.###.###.#.#.#  \n",
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  \n",
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  \n",
        "  #.......#.....#.#...#...............#...#  \n",
        "  #############.#.#.###.###################  \n",
        "               A O F   N                     \n",
        "               A A D   M                     \n",
    ]
    .join("");
    let maze = parse_input(&example3);
    assert_eq!(Some(77), solve(&maze, false));
    assert_eq!(Some(396), solve(&maze, true));
}
