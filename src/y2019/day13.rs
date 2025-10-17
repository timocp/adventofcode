use super::intcode::Vm;
use crate::grid::{Grid, Pos};
use core::fmt;

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
        game.run();
        game.count(Tile::Block).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

struct Game {
    vm: Vm,
    screen: Grid<Tile>,
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
        }
    }

    fn run(&mut self) {
        for slice in self.vm.run(&[]).chunks_exact(3) {
            self.draw(slice[0] as i32, slice[1] as i32, slice[2].into());
        }
    }

    fn draw(&mut self, x: i32, y: i32, tile: Tile) {
        self.screen.set(Pos { x, y }, tile);
    }

    fn count(&self, tile: Tile) -> usize {
        self.screen.iter().filter(|(_p, t)| **t == tile).count()
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
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
