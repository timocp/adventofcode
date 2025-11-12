use crate::grid::{Compass, Grid, Pos};
use crate::{bfs, dijkstra};
use std::fmt;

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
        Search::new(&self.maze).shortest_path().unwrap().to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn parse_input(input: &str) -> Maze {
    let mut entrance: Option<Pos> = None;
    let mut keys: Vec<Option<Pos>> = vec![None; 26];
    let grid = Grid::from_input_by(input, Cell::Wall, |p, c| match c {
        '@' => {
            entrance = Some(p);
            Cell::Passage
        }
        '#' => Cell::Wall,
        '.' => Cell::Passage,
        'a'..='z' => {
            let num = c as u8 - b'a';
            keys[num as usize] = Some(p);
            Cell::Key(num)
        }
        'A'..='Z' => {
            let num = c as u8 - b'A';
            Cell::Door(num)
        }
        _ => panic!("unhandled input: {}", c),
    });
    Maze {
        grid,
        entrance: entrance.expect("entrance missing"),
        keys,
    }
}

struct Maze {
    grid: Grid<Cell>,
    entrance: Pos,
    keys: Vec<Option<Pos>>,
}

impl Maze {
    fn neighbours(&self, from: &Pos) -> Vec<(Pos, Compass)> {
        [
            (from.step(Compass::North), Compass::North),
            (from.step(Compass::East), Compass::East),
            (from.step(Compass::South), Compass::South),
            (from.step(Compass::West), Compass::West),
        ]
        .into_iter()
        .filter(|(p, _d)| *self.grid.get(*p) != Cell::Wall)
        .collect()
    }
}

// Representing a movement between 2 places in the maze
struct Path {
    // number of steps needed
    steps: u32,
    // bitmask of which doors are used on this path
    keys_needed: u32,
}

const ENTRANCE: u8 = 26;

// Info needed for shortest distance search
struct Search {
    // Precomputed paths from k1 to k2 (if valid).  last position [26] is for paths from entrance
    paths: [[Option<Path>; 27]; 27],
    // bitmap of all keys that are present
    all_keys: u32,
}

impl Search {
    fn new(maze: &Maze) -> Self {
        let mut paths: [[Option<Path>; 27]; 27] = std::array::from_fn(|_| [const { None }; 27]);

        //print!("{}", maze);

        // precompute goal (holding all keys)
        let mut all_keys = 0;
        for key in maze
            .keys
            .iter()
            .enumerate()
            .filter_map(|(k, pos)| pos.as_ref().map(|_| k as u8))
        {
            all_keys |= 1 << key;
        }
        //println!("all_keys: {}", keys_to_string(all_keys));

        // precompute paths from entrance to all keys
        // TODO: could probably exit early in some situations, but this is already quite
        // fast compared to the dijkstra search later
        bfs::traverse(
            &maze.entrance,
            |p| maze.neighbours(p),
            |p, steps, path| {
                if let Cell::Key(k2) = maze.grid.get(*p) {
                    let mut keys_needed = 0;
                    let mut keys_on_path = 0;
                    for v in path {
                        match maze.grid.get(*v) {
                            Cell::Door(door) => keys_needed |= 1 << door,
                            Cell::Key(_) => keys_on_path += 1,
                            _ => (),
                        };
                    }
                    if keys_on_path == 0 {
                        paths[ENTRANCE as usize][*k2 as usize] = Some(Path { steps, keys_needed });
                    }
                }
            },
        );

        // precompute paths from each key to every other key
        for (k1, from) in maze
            .keys
            .iter()
            .enumerate()
            .filter_map(|(key, from)| (*from).map(|pos| (key as u8, pos)))
        {
            bfs::traverse(
                &from,
                |p| maze.neighbours(p),
                |p, steps, path| {
                    if let Cell::Key(k2) = maze.grid.get(*p)
                        && *k2 > k1
                    {
                        let mut keys_needed = 0;
                        let mut keys_on_path = 0;
                        for v in path {
                            match maze.grid.get(*v) {
                                Cell::Door(door) => keys_needed |= 1 << door,
                                Cell::Key(_) => keys_on_path += 1,
                                _ => (),
                            };
                        }
                        if keys_on_path == 1 {
                            paths[k1 as usize][*k2 as usize] = Some(Path { steps, keys_needed });
                            paths[*k2 as usize][k1 as usize] = Some(Path { steps, keys_needed });
                        }
                    }
                },
            )
        }

        Search { paths, all_keys }
    }

    fn shortest_path(&self) -> Option<u32> {
        dijkstra::shortest_path(
            &Node {
                location: ENTRANCE,
                keys_held: 0,
            },
            |n| {
                self.paths[n.location as usize]
                    .iter()
                    .enumerate()
                    .filter_map(|(k2, path)| path.as_ref().map(|p| (k2 as u8, p)))
                    .filter(|(_k2, path)| (path.keys_needed & n.keys_held) == path.keys_needed)
                    .map(|(k2, path)| {
                        (
                            Node {
                                location: k2,
                                keys_held: n.keys_held | 1 << k2,
                            },
                            path.steps,
                        )
                    })
                    .collect::<Vec<_>>()
            },
            |n| n.keys_held == self.all_keys,
        )
    }
}

// state node for shortest path search
#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Node {
    location: u8,   // Key we're standing at, or 26 if we're at entrance
    keys_held: u32, // Bitmap of the keys that we hold
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[at={} keys_held={}]",
            if self.location == ENTRANCE {
                '@'
            } else {
                (self.location + 97) as char
            },
            keys_to_string(self.keys_held)
        )
    }
}

fn keys_to_string(keys: u32) -> String {
    let mut s = "".to_string();
    for k in 'a'..='z' {
        if keys & 1 << (k as u8 - 97) > 0 {
            s.push(k)
        }
    }
    s
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Wall,
    Passage,
    Key(u8),  // a = 0, b = 1, etc
    Door(u8), // A = 0, B = 1, etc
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (p, c) in self.grid.iter() {
            write!(
                f,
                "{}",
                match c {
                    Cell::Wall => '█',
                    Cell::Passage =>
                        if p == self.entrance {
                            '@'
                        } else {
                            '·'
                        },
                    Cell::Key(k) => (k + b'a') as char,
                    Cell::Door(d) => (d + b'A') as char,
                }
            )?;
            if p.x == self.grid.maxx() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[test]
fn test0() {
    let test_input = "\
#########
#b...@..#
#########
";
    let maze = parse_input(test_input);
    assert_eq!(Some(4), Search::new(&maze).shortest_path());
}

#[test]
fn test1() {
    let test_input = "\
#########
#b.A.@.a#
#########
";
    let maze = parse_input(test_input);
    assert_eq!(Some(8), Search::new(&maze).shortest_path());
}

#[test]
fn test2() {
    let test_input = "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";
    let maze = parse_input(test_input);
    assert_eq!(Some(86), Search::new(&maze).shortest_path());
}

#[test]
fn test3() {
    let test_input = "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
";
    let maze = parse_input(test_input);
    assert_eq!(Some(132), Search::new(&maze).shortest_path());
}

#[test]
fn test4() {
    let test_input = "\
#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################
";
    let maze = parse_input(test_input);
    assert_eq!(Some(136), Search::new(&maze).shortest_path());
}

#[test]
fn test5() {
    let test_input = "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
";
    let maze = parse_input(test_input);
    assert_eq!(Some(81), Search::new(&maze).shortest_path());
}
