use crate::pixel_buffer::PixelBuffer;
use std::collections::VecDeque;
use std::fmt;

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
        let mut buffer = PixelBuffer::new(
            self.pixels.first().unwrap().len() as u32,
            self.pixels.len() as u32,
        );
        for (y, row) in self.pixels.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                buffer.set(x as u32, y as u32, pixel);
            }
        }
        write!(f, "{}", buffer)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

pub fn part1(input: &[Instruction]) -> usize {
    process(input).count_lit()
}

pub fn part2(input: &[Instruction]) -> String {
    process(input).to_string()
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

pub fn parse_input(input: &str) -> Vec<Instruction> {
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
