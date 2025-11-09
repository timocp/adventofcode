use super::intcode::Vm;
use crate::grid::{Compass, Compass::*, Grid, ORIGIN, Pos};
use std::fmt;
use std::ops::Range;

pub struct Solver {
    vm: Vm,
    map: Grid<Cell>,
    robot_start_pos: Pos,
    robot_start_dir: Compass,
    debug: bool, // if true, display all output from the program
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        let vm = Vm::from(input);

        // initial camera output is used to construct the map and robot starting position
        let output = vm.clone().run(&[]);
        let width = output.iter().position(|&i| i == 10).unwrap() as u32;
        let height = (output.len() as u32 - 1) / (width + 1);
        let mut map = Grid::new(width, height, Cell::Unknown);
        let mut robot_start_pos = ORIGIN;
        let mut robot_start_dir: Option<Compass> = None;
        let mut p = ORIGIN;
        for i in output.iter().map(|i| *i as u8) {
            if i == b'\n' {
                p.x = 0;
                p.y += 1;
            } else {
                match i {
                    b'#' => {
                        map.set(p, Cell::Scaffold);
                    }
                    b'.' => {
                        map.set(p, Cell::Space);
                    }
                    b'<' | b'>' | b'^' | b'v' => {
                        map.set(p, Cell::Scaffold);
                        robot_start_pos = p;
                        robot_start_dir = Some(match i {
                            b'<' => West,
                            b'>' => East,
                            b'^' => North,
                            b'v' => South,
                            _ => unreachable!(),
                        });
                    }
                    _ => {
                        panic!("Unhandled input: {}", i);
                    }
                }
                p.x += 1;
            }
        }

        Self {
            vm,
            map,
            robot_start_pos,
            robot_start_dir: robot_start_dir.unwrap(),
            debug: false,
        }
    }

    fn part1(&self) -> String {
        sum_intersections(&self.map).to_string()
    }

    fn part2(&self) -> String {
        let mut vm = self.vm.clone();
        vm.direct_write(0, 2); // instruct robot to wake up

        // solve the actual problem
        let path = find_path(&self.map, self.robot_start_pos, self.robot_start_dir);
        let (a, b, c, main) = split_paths(&path);

        // play the solution into the VM
        run_program(&mut vm, None, self.debug);
        // Main:
        run_program(
            &mut vm,
            Some(
                main.iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            ),
            self.debug,
        );
        // Function A:
        run_program(&mut vm, Some(moves_to_string(&path[a])), self.debug);
        // Function B:
        run_program(&mut vm, Some(moves_to_string(&path[b])), self.debug);
        // Function C:
        run_program(&mut vm, Some(moves_to_string(&path[c])), self.debug);
        // Continuous video feed?
        let output = run_program(
            &mut vm,
            Some((if self.debug { "y" } else { "n" }).to_string()),
            self.debug,
        );

        output.last().unwrap().to_string()
    }
}

// Input is a string, if present is converted to intcode input with an ASCII newline added.
// If debug is true, the ASCII output of the program will be printed to the console
fn run_program(vm: &mut Vm, input: Option<String>, debug: bool) -> Vec<i64> {
    let input: Vec<i64> = match input {
        Some(string) => {
            let mut v: Vec<_> = string.chars().map(|c| c as i64).collect();
            v.push(b'\n' as i64);
            v
        }
        None => vec![],
    };
    if debug {
        println!("INPUT: {:?}", input);
    }
    let output = vm.run(&input);
    if debug {
        println!("{}", output_to_string(&output));
    }
    output
}

// split a path of moves into 3 functions (a, b, c) and a main routine that
// defines the order to use them
// a, b and c are returned as ranges into the original path
//
// constraint:
// each path must be rendered as a string in <= 20 characters so it can be entered
// into the VM's input prompts
//
// Assumptions:
// A will always start at 0
// B will follow A (maybe not true in general? A could repeat)
// C may start anywhere after where B starts (it could overlap with either A or B)
fn split_paths(path: &[Move]) -> (Range<usize>, Range<usize>, Range<usize>, Vec<char>) {
    let a_from = 0;
    for a_to in (1..path.len()).rev() {
        let a = a_from..(a_to + 1);
        if moves_size(&path[a.clone()]) <= 20 {
            let b_from = a_to + 1;
            for b_to in ((b_from + 1)..path.len()).rev() {
                let b = b_from..(b_to + 1);
                if moves_size(&path[b.clone()]) <= 20 {
                    for c_from in (b_from + 1)..path.len() {
                        for c_to in ((c_from + 1)..path.len()).rev() {
                            let c = c_from..(c_to + 1);
                            if moves_size(&path[c.clone()]) <= 20
                                && let Some(mut main) = check_solution(path, 0, &a, &b, &c)
                            {
                                main.reverse();
                                return (a, b, c, main);
                            }
                        }
                    }
                }
            }
        }
    }
    panic!("No solution found");
}

fn check_solution(
    path: &[Move],
    from: usize,
    a: &Range<usize>,
    b: &Range<usize>,
    c: &Range<usize>,
) -> Option<Vec<char>> {
    if from == path.len() {
        return Some(vec![]);
    }
    if range_match(path, from, a)
        && let Some(mut solution) = check_solution(path, from + a.len(), a, b, c)
    {
        solution.push('A');
        return Some(solution);
    }
    if range_match(path, from, b)
        && let Some(mut solution) = check_solution(path, from + b.len(), a, b, c)
    {
        solution.push('B');
        return Some(solution);
    }
    if range_match(path, from, c)
        && let Some(mut solution) = check_solution(path, from + c.len(), a, b, c)
    {
        solution.push('C');
        return Some(solution);
    }

    None
}

fn range_match(path: &[Move], from: usize, range: &Range<usize>) -> bool {
    if from + range.len() > path.len() {
        return false;
    }
    for (i, r) in range.clone().enumerate() {
        if path[r] != path[from + i] {
            return false;
        }
    }
    true
}

// Assumptions about the input:
// - we can traverse the scaffold by a series of moves, where a move is a turn
//   left or right then steps as far as possible
// - reaching the end of the straight line is always followed by a left or
//   right turn (never a choice)
// - When reaching a dead end, we've seen every part of the map.
fn find_path(map: &Grid<Cell>, robot_start_pos: Pos, robot_start_dir: Compass) -> Vec<Move> {
    let mut path = vec![];
    let mut pos = robot_start_pos;
    let mut dir = robot_start_dir;

    loop {
        let turn = if is_scaffold(map.look(pos, dir.left90())) {
            Turn::Left
        } else if is_scaffold(map.look(pos, dir.right90())) {
            Turn::Right
        } else {
            return path;
        };
        dir = match turn {
            Turn::Left => dir.left90(),
            Turn::Right => dir.right90(),
        };
        let mut steps = 0;
        while is_scaffold(map.look(pos, dir)) {
            steps += 1;
            pos = pos.step(dir);
        }
        path.push(Move { turn, steps });
    }
}

#[derive(PartialEq)]
enum Turn {
    Left,
    Right,
}

#[derive(PartialEq)]
struct Move {
    turn: Turn,
    steps: u32,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{},{}",
            match self.turn {
                Turn::Left => "L",
                Turn::Right => "R",
            },
            self.steps
        )
    }
}

fn moves_size(moves: &[Move]) -> usize {
    moves_to_string(moves).len()
}

fn moves_to_string(moves: &[Move]) -> String {
    moves
        .iter()
        .map(|m| m.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn output_to_string(output: &[i64]) -> String {
    output
        .iter()
        .filter_map(|&i| char::from_u32(i as u32))
        .collect()
}

fn sum_intersections(image: &Grid<Cell>) -> i32 {
    let mut sum = 0;
    for (p, cell) in image.iter() {
        if [
            cell,
            image.look(p, North),
            image.look(p, East),
            image.look(p, South),
            image.look(p, West),
        ]
        .into_iter()
        .all(is_scaffold)
        {
            sum += p.alignment_parameter();
        }
    }
    sum
}

fn is_scaffold(cell: &Cell) -> bool {
    matches!(cell, Cell::Scaffold)
}

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Unknown,
    Space,
    Scaffold,
}

impl Pos {
    fn alignment_parameter(&self) -> i32 {
        self.x * self.y
    }
}
