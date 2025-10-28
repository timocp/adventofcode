use super::intcode::Vm;
use crate::grid::{Compass, Compass::*, Grid, ORIGIN, Pos};

pub struct Solver {
    camera_vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            camera_vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let image = capture_image(&self.camera_vm);
        //println!("{}", print_image(&image));
        sum_intersections(&image).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn capture_image(camera_vm: &Vm) -> Grid<Cell> {
    let output = camera_vm.clone().run(&[]);
    let width = output.iter().position(|&i| i == 10).unwrap() as u32;
    let height = (output.len() as u32 - 1) / (width + 1);
    let mut image = Grid::new(width, height, Cell::Unknown);
    let mut p = ORIGIN;
    for i in output {
        if i == 10 {
            p.x = 0;
            p.y += 1;
        } else {
            image.set(
                p,
                match i {
                    35 => Cell::Scaffold,     // #
                    46 => Cell::Space,        // .
                    94 => Cell::Robot(North), // ^
                    _ => panic!("unhandled: {}", i),
                },
            );
            p.x += 1;
        }
    }

    image
}

#[allow(dead_code)]
fn print_image(image: &Grid<Cell>) -> String {
    let mut s = "".to_string();

    for (p, cell) in image.iter() {
        s.push(match cell {
            Cell::Unknown => panic!("grid is not filled {}", p),
            Cell::Space => '.',
            Cell::Scaffold => '#',
            Cell::Robot(dir) => match dir {
                North => '^',
                East => '>',
                South => 'v',
                West => '<',
                _ => unreachable!(),
            },
        });
        if p.x == image.maxx() {
            s += "\n";
        }
    }

    s
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
    matches!(cell, Cell::Scaffold | Cell::Robot(_))
    //match cell {
    //    Cell::Scaffold => true,
    //    Cell::Robot(_) => true,
    //    _ => false,
    //}
}

#[derive(Clone, PartialEq)]
enum Cell {
    Unknown,
    Space,
    Scaffold,
    Robot(Compass),
}

impl Pos {
    fn alignment_parameter(&self) -> i32 {
        self.x * self.y
    }
}
