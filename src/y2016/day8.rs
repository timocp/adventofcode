use std::collections::VecDeque;
use std::fmt;

use crate::Puzzle;

pub struct Solver {
    input: Vec<Instruction>,
}

struct Screen {
    pixels: Vec<VecDeque<bool>>,
}

impl Screen {
    fn new() -> Self {
        Self {
            pixels: (0..6).map(|_| VecDeque::from([false; 50])).collect(),
        }
    }

    fn light_rect(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.pixels[row][col] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, by: usize) {
        self.pixels[row].rotate_right(by % 50);
    }

    fn rotate_column(&mut self, col: usize, by: usize) {
        let mut new_column =
            VecDeque::from(self.pixels.iter().map(|row| row[col]).collect::<Vec<_>>());
        new_column.rotate_right(by % 6);
        for (row, value) in new_column.iter().enumerate() {
            self.pixels[row][col] = *value;
        }
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().flatten().filter(|&&pixel| pixel).count()
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.pixels.iter() {
            for pixel in row.iter() {
                write!(f, "{}", if *pixel { '█' } else { ' ' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        process(&self.input).count_lit().to_string()
    }

    fn part2(&self) -> String {
        process(&self.input).to_string()
    }
}

fn process(instructions: &[Instruction]) -> Screen {
    let mut screen = Screen::new();
    for instruction in instructions {
        match instruction {
            Instruction::Rect(width, height) => screen.light_rect(*width, *height),
            Instruction::RotateColumn(col, by) => screen.rotate_column(*col, *by),
            Instruction::RotateRow(row, by) => screen.rotate_row(*row, *by),
        }
    }
    screen
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("rect ") {
                Instruction::Rect(
                    line[5..line.find('x').unwrap()].parse().unwrap(),
                    line[line.find('x').unwrap() + 1..].parse().unwrap(),
                )
            } else if line.starts_with("rotate row y=") {
                let by = line.find(" by ").unwrap();
                Instruction::RotateRow(
                    line[13..by].parse().unwrap(),
                    line[by + 4..].parse().unwrap(),
                )
            } else if line.starts_with("rotate column x=") {
                let by = line.find(" by ").unwrap();
                Instruction::RotateColumn(
                    line[16..by].parse().unwrap(),
                    line[by + 4..].parse().unwrap(),
                )
            } else {
                panic!("Invalid instruction: {}", line);
            }
        })
        .collect()
}
