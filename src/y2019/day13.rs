use super::intcode::Vm;
use crate::grid::{Grid, Pos};
use core::fmt;
use std::cmp::Ordering;
use std::{thread, time};

pub struct Solver {
    arcade_vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            arcade_vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let mut game = Game::new(self.arcade_vm.clone());
        game.step(&[]);
        game.count(Tile::Block).to_string()
    }

    fn part2(&self) -> String {
        let mut game = Game::new(self.arcade_vm.clone());
        game.play(false); // change to true to watch game play
        game.score.to_string()
    }
}

struct Game {
    vm: Vm,
    screen: Grid<Tile>,
    score: i64,  // most recent score
    paddle: i32, // x position of paddle
    ball: i32,   // x position of ball
}

impl Game {
    fn new(vm: Vm) -> Self {
        // run once to determine screen size (the last instruction emitted is for
        // the bottom-right tile)
        let output: Vec<_> = vm.clone().run(&[]);
        let width = output[output.len() - 3] as u32 + 1;
        let height = output[output.len() - 2] as u32 + 1;

        Game {
            vm,
            screen: Grid::new(width, height, Tile::Empty),
            score: -1,
            paddle: -1,
            ball: -1,
        }
    }

    fn step(&mut self, input: &[i64]) {
        for slice in self.vm.run(input).chunks_exact(3) {
            if slice[0] == -1 {
                self.score = slice[2];
            } else {
                self.draw(slice[0] as i32, slice[1] as i32, slice[2].into());
            }
        }
    }

    fn play(&mut self, watch: bool) {
        self.vm.direct_write(0, 2); // play for free
        self.step(&[]);
        if watch {
            println!("{}", self);
        }
        while !self.vm.is_halted() {
            self.step(&[self.joystick_direction()]);
            if watch {
                println!("{}", self);
                thread::sleep(time::Duration::from_millis(10));
            }
        }
    }

    fn draw(&mut self, x: i32, y: i32, tile: Tile) {
        self.screen.set(Pos { x, y }, tile);
        match tile {
            Tile::Paddle => self.paddle = x,
            Tile::Ball => self.ball = x,
            _ => (),
        }
    }

    fn count(&self, tile: Tile) -> usize {
        self.screen.iter().filter(|(_p, t)| **t == tile).count()
    }

    // command that will move the joystick towards the ball
    fn joystick_direction(&self) -> i64 {
        match self.paddle.cmp(&self.ball) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{esc}[2J{esc}[1;1H", esc = 27 as char)?; // clear screen
        for (p, t) in self.screen.iter() {
            write!(
                f,
                "{}",
                match t {
                    Tile::Empty => ' ',
                    Tile::Wall => '█',
                    Tile::Block => '#',
                    Tile::Paddle => '=',
                    Tile::Ball => 'O',
                }
            )?;
            if p.x == self.screen.maxx() {
                if p.y == 0 {
                    write!(f, "   Score: {}", self.score)?;
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl From<i64> for Tile {
    fn from(i: i64) -> Self {
        match i {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!(),
        }
    }
}
