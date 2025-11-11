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
        Paths::new(&self.maze).shortest_path().unwrap().to_string()
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

// Precomputed distances between each point of interest
// TODO: May be faster to only compute these on demand
struct Paths {
    entrance_to_key: [Option<Path>; 26],
    key_to_key: [[Option<Path>; 26]; 26],
    // bitmap of all keys that are present
    all_keys: u32,
}

impl Paths {
    fn new(maze: &Maze) -> Self {
        let mut entrance_to_key: [Option<Path>; 26] = [const { None }; 26];
        let mut key_to_key: [[Option<Path>; 26]; 26] =
            std::array::from_fn(|_| [const { None }; 26]);

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
                if let Cell::Key(key) = maze.grid.get(*p) {
                    let mut keys_needed = 0;
                    for v in path {
                        if let Cell::Door(door) = maze.grid.get(*v) {
                            keys_needed |= 1 << door;
                        }
                    }
                    entrance_to_key[*key as usize] = Some(Path { steps, keys_needed })
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
                        for v in path {
                            if let Cell::Door(door) = maze.grid.get(*v) {
                                keys_needed |= 1 << door;
                            }
                        }
                        key_to_key[k1 as usize][*k2 as usize] = Some(Path { steps, keys_needed });
                        key_to_key[*k2 as usize][k1 as usize] = Some(Path { steps, keys_needed });
                    }
                },
            )
        }

        Paths {
            entrance_to_key,
            key_to_key,
            all_keys,
        }
    }

    fn shortest_path(&self) -> Option<u32> {
        dijkstra::shortest_path(
            &Node {
                at_key: None,
                keys_held: 0,
            },
            |n| match n.at_key {
                // TODO: Possible improvement by removing the collect() but
                // currently doesn't work as the arm types would be different
                Some(k1) => self.key_to_key[k1 as usize]
                    .iter()
                    .enumerate()
                    .filter_map(|(k2, path)| path.as_ref().map(|p| (k2 as u8, p)))
                    .filter(|(_k2, path)| (path.keys_needed & n.keys_held) == path.keys_needed)
                    .map(|(k2, path)| {
                        (
                            Node {
                                at_key: Some(k2),
                                keys_held: n.keys_held | 1 << k2,
                            },
                            path.steps,
                        )
                    })
                    .collect::<Vec<_>>(),
                None => self
                    .entrance_to_key
                    .iter()
                    .enumerate()
                    .filter_map(|(k2, path)| path.as_ref().map(|p| (k2 as u8, p)))
                    .filter(|(_k2, path)| (path.keys_needed & n.keys_held) == path.keys_needed)
                    .map(|(k2, path)| {
                        (
                            Node {
                                at_key: Some(k2),
                                keys_held: n.keys_held | 1 << k2,
                            },
                            path.steps,
                        )
                    })
                    .collect::<Vec<_>>(),
            },
            |n| n.keys_held == self.all_keys,
        )
    }
}

// state node for shortest path search
#[derive(Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct Node {
    at_key: Option<u8>, // Key we're standing at, or None if we're at entrance
    keys_held: u32,     // Bitmap of the keys that we hold
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[at={} keys_held={}]",
            match self.at_key {
                Some(k) => (k + 97) as char,
                None => '@',
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
    Door(u8), // A = 1, B = 1, etc
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
    let paths = Paths::new(&maze);
    assert_eq!(Some(4), paths.shortest_path());
}

#[test]
fn test1() {
    let test_input = "\
#########
#b.A.@.a#
#########
";
    let maze = parse_input(test_input);
    let paths = Paths::new(&maze);
    assert_eq!(Some(8), paths.shortest_path());
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
    let paths = Paths::new(&maze);
    assert_eq!(Some(86), paths.shortest_path());
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
    let paths = Paths::new(&maze);
    assert_eq!(Some(132), paths.shortest_path());
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
    let paths = Paths::new(&maze);
    assert_eq!(Some(136), paths.shortest_path());
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
    let paths = Paths::new(&maze);
    assert_eq!(Some(81), paths.shortest_path());
}
