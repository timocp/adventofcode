use super::intcode::Vm;
use crate::grid::{Compass, ORIGIN, Pos, SparseGrid};
use crate::pixel_buffer::PixelBuffer;
use std::fmt;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(vm: &Vm) -> u32 {
    let mut hull = new_hull(Paint::Black);
    paint(vm, &mut hull);
    // hull.to_string()
    hull.len()
}

pub fn part2(vm: &Vm) -> String {
    let mut hull = new_hull(Paint::White);
    paint(vm, &mut hull);
    hull.to_string()
}

fn new_hull(start: Paint) -> SparseGrid<Paint> {
    SparseGrid::new(start)
}

#[derive(Clone, PartialEq)]
enum Paint {
    Black,
    White,
}

impl From<i64> for Paint {
    fn from(i: i64) -> Self {
        match i {
            0 => Self::Black,
            1 => Self::White,
            _ => panic!("Invalid colour: {}", i),
        }
    }
}

impl From<Paint> for i64 {
    fn from(paint: Paint) -> Self {
        match paint {
            Paint::Black => 0,
            Paint::White => 1,
        }
    }
}

struct Robot {
    pos: Pos,
    facing: Compass,
}

impl Robot {
    fn turn_left(&mut self) {
        self.facing = self.facing.left90();
    }

    fn turn_right(&mut self) {
        self.facing = self.facing.right90();
    }

    fn move_forward(&mut self) {
        self.pos = self.pos.step(self.facing);
    }
}

fn paint(paint_vm: &Vm, hull: &mut SparseGrid<Paint>) {
    let mut vm = paint_vm.clone();
    let mut robot = Robot {
        pos: ORIGIN,
        facing: Compass::North,
    };
    while !vm.is_halted() {
        let commands = vm.run(&[hull.get(robot.pos).clone().into()]);
        for cmd in commands.chunks(2) {
            hull.set(robot.pos, Paint::from(cmd[0]));
            match cmd[1] {
                0 => robot.turn_left(),
                1 => robot.turn_right(),
                _ => panic!(),
            }
            robot.move_forward()
        }
    }
}

impl fmt::Display for SparseGrid<Paint> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = PixelBuffer::new(self.width(), self.height());
        for p in self.iter().filter_map(|(p, paint)| {
            if *paint == Paint::White {
                Some(p)
            } else {
                None
            }
        }) {
            buffer.set(
                (p.x - self.minx()).try_into().unwrap(),
                (p.y - self.miny()).try_into().unwrap(),
                true,
            )
        }
        write!(f, "{}", buffer)
    }
}
