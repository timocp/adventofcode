use self::Dir::*;
use crate::grid::{Grid, P, parse_each_char};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt;
use std::fmt::Write;
use std::slice::Iter;

pub struct Solver {
    input: String,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }

    fn part1(&self) -> String {
        let mut game = Game::new(&self.input);
        game.simulate(None);
        game.outcome().to_string()
    }

    fn part2(&self) -> String {
        Game::help_elves(&self.input).outcome().to_string()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Team {
    Elves,
    Goblins,
}

impl Team {
    #[allow(clippy::wrong_self_convention)]
    fn to_char(&self) -> char {
        match &self {
            Team::Elves => 'E',
            Team::Goblins => 'G',
        }
    }

    fn other(&self) -> Team {
        match &self {
            Team::Elves => Team::Goblins,
            Team::Goblins => Team::Elves,
        }
    }
}

// order is important (reading order = row first)
impl Ord for P {
    fn cmp(&self, other: &Self) -> Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl PartialOrd for P {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl P {
    fn in_range(&self, other: Self) -> bool {
        self.x == other.x && ((other.y > 0 && self.y == other.y - 1) || self.y == other.y + 1)
            || self.y == other.y
                && ((other.x > 0 && self.x == other.x - 1) || self.x == other.x + 1)
    }

    fn step(&self, dir: Dir) -> Self {
        match dir {
            North => P {
                x: self.x,
                y: self.y - 1,
            },
            East => P {
                x: self.x + 1,
                y: self.y,
            },
            South => P {
                x: self.x,
                y: self.y + 1,
            },
            West => P {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
struct Unit {
    hp: u8,
    p: P,
    team: Team,
    power: u8,
}

impl Unit {
    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn other(&self) -> Team {
        self.team.other()
    }
}

#[derive(Clone)]
enum Tile {
    Open,
    Wall,
}

struct Game {
    grid: Grid<Tile>,
    units: Vec<Unit>,
    rounds: u32,
    debug: bool,
    elf_power: u8,
    winner: Option<Team>,
}

// Order is important for derived ordering (NWES is reading order)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    North,
    West,
    East,
    South,
}

impl Dir {
    fn each() -> Iter<'static, Dir> {
        static DIRS: [Dir; 4] = [North, West, East, South];
        DIRS.iter()
    }
}

#[derive(Debug)]
struct Path {
    start_dir: Dir,
    p: P,
    inrange: bool,
}

impl Game {
    fn new(input: &str) -> Game {
        let width = input.lines().next().unwrap().len() as u32;
        let height = input.lines().count() as u32;
        let mut game = Game {
            grid: Grid::new(width, height, Tile::Wall),
            units: vec![],
            rounds: 0,
            debug: false,
            elf_power: 3,
            winner: None,
        };
        for (p, c) in parse_each_char(input) {
            match c {
                '#' => {}
                '.' => {
                    game.grid.set(p, Tile::Open);
                }
                'E' | 'G' => {
                    game.grid.set(p, Tile::Open);
                    game.units.push(Unit {
                        team: if c == 'E' { Team::Elves } else { Team::Goblins },
                        hp: 200,
                        p,
                        power: 3,
                    });
                }
                _ => {
                    panic!("Unhandled input character: {}", c);
                }
            }
        }
        game
    }

    // part 2, brute-force the minimal attack strength elves would need to win
    fn help_elves(input: &str) -> Game {
        let mut elf_power = 3;
        loop {
            elf_power += 1;
            let mut game = Game::new(input);
            game.set_elf_power(elf_power);
            game.simulate(Some(Team::Elves));
            if game.winner == Some(Team::Elves) {
                return game;
            }
        }
    }

    fn simulate(&mut self, require_total_victory: Option<Team>) {
        loop {
            if self.debug {
                print!("After {} round(s)\n{:?}", self.rounds, self);
            }
            for u in 0..self.units.len() {
                if self.is_victory(&self.units[u].team) {
                    self.winner = Some(self.units[u].team);
                    if self.debug {
                        println!("Unit {} sees the battlefield is clear. Victory!", u);
                    }
                    return;
                }
                self.move_unit(u);
                self.attack(u);
            }
            if let Some(team) = require_total_victory
                && self
                    .units
                    .iter()
                    .any(|unit| unit.team == team && !unit.is_alive())
            {
                return;
            }
            self.units.retain(|unit| unit.is_alive());
            self.sort_units();
            self.rounds += 1;
        }
    }

    // returns false if there are no enemies left anywhere
    fn move_unit(&mut self, u: usize) {
        let unit = &self.units[u];

        if !unit.is_alive() {
            return;
        }

        // if we're already in range of an enemy, don't move
        if self.can_attack(u) {
            if self.debug {
                print!("Unit {} is near an enemy.", u);
            }
            return;
        }

        // build up a BFS, but we only need to retain the deepest node of each
        // path.  each path remembers its starting step so that ties can be
        // separated.
        // note that the ordering of the directions is used to ensure we only
        // follow the correctly ordered path to any target square.
        let mut paths: Vec<Path> = vec![Path {
            start_dir: North, // ignored
            p: unit.p,
            inrange: false,
        }];

        // set to record length of best known path to any position, which
        // is used to prune loops and detect inability to move.
        let mut seen = HashSet::new();

        let mut stop = false;
        let mut depth = 0;
        while !stop {
            // build a new vector which is the next layer of the tree
            depth += 1;
            let mut new_paths: Vec<Path> = vec![];
            for old_path in paths.into_iter() {
                for dir in Dir::each() {
                    let p = old_path.p.step(*dir);
                    if seen.contains(&p) {
                        continue;
                    }
                    seen.insert(p);
                    if self.is_empty(p) {
                        let inrange = self.in_range(p, unit.other());
                        if inrange {
                            stop = true;
                        }
                        new_paths.push(Path {
                            start_dir: if depth == 1 { *dir } else { old_path.start_dir },
                            p,
                            inrange,
                        });
                    }
                }
            }
            if new_paths.is_empty() {
                if self.debug {
                    print!("Unit {} has no path to targets.", u);
                }
                return;
            }
            paths = new_paths;
        }

        let target_tile = paths
            .iter()
            .filter(|node| node.inrange)
            .map(|node| node.p)
            .min()
            .unwrap();

        // there should only be one remaining path to the target square, so
        // move in that direction.
        let dir = paths
            .iter()
            .find(|node| node.p == target_tile)
            .unwrap()
            .start_dir;

        if self.debug {
            print!(
                "Unit {} moves {:?}, (heading for {:?}).",
                u, dir, target_tile
            );
        }
        self.units[u].p = unit.p.step(dir);
    }

    fn is_victory(&self, team: &Team) -> bool {
        !self
            .units
            .iter()
            .any(|other| other.is_alive() && other.team == team.other())
    }

    fn attack(&mut self, u: usize) {
        let unit = &self.units[u];
        if !unit.is_alive() {
            return;
        }
        let other_team = unit.team.other();
        let neighbours: Vec<P> = Dir::each().map(|dir| unit.p.step(*dir)).collect();
        let mut enemies: Vec<(usize, &mut Unit)> = self
            .units
            .iter_mut()
            .enumerate()
            .filter(|(_, other)| other.is_alive() && other.team == other_team)
            .filter(|(_, enemy)| neighbours.contains(&enemy.p))
            .collect();
        if enemies.is_empty() {
            if self.debug {
                println!("  Noone to attack.");
            }
            return;
        }
        let min_hp = enemies.iter().map(|(_, enemy)| enemy.hp).min().unwrap();
        enemies.retain(|(_, enemy)| enemy.hp == min_hp);
        let target = enemies.iter().min_by_key(|(_, enemy)| enemy.p).unwrap().0;
        let power = self.units[u].power;
        if self.debug {
            print!("  Attacks unit {} with power {}.", target, power);
        }
        if self.units[target].hp > power {
            if self.debug {
                println!();
            }
            self.units[target].hp -= power;
        } else {
            if self.debug {
                println!("  It dies!");
            }
            self.units[target].hp = 0;
        }
    }

    fn set_elf_power(&mut self, elf_power: u8) {
        self.elf_power = elf_power;
        for elf in self
            .units
            .iter_mut()
            .filter(|unit| unit.team == Team::Elves)
        {
            elf.power = elf_power;
        }
    }

    fn total_hp(&self) -> u32 {
        self.units
            .iter()
            .filter(|unit| unit.is_alive())
            .map(|unit| unit.hp as u32)
            .sum()
    }

    fn can_attack(&self, u: usize) -> bool {
        let unit = &self.units[u];
        let other_team = unit.team.other();
        self.units
            .iter()
            .filter(|other| other.is_alive() && other.team == other_team)
            .any(|enemy| enemy.p.in_range(unit.p))
    }

    fn is_empty(&self, p: P) -> bool {
        match self.grid.get(p) {
            Tile::Wall => false,
            Tile::Open => !self.units.iter().any(|unit| unit.hp > 0 && unit.p == p),
        }
    }

    // return true if position `p` in in range of a unit of team team
    fn in_range(&self, p: P, team: Team) -> bool {
        self.units
            .iter()
            .filter(|unit| unit.is_alive() && unit.team == team)
            .any(|enemy| enemy.p.in_range(p))
    }

    // sort units into reading order
    fn sort_units(&mut self) {
        self.units.sort_by_key(|u| u.p);
    }

    fn outcome(&self) -> u32 {
        self.rounds * self.total_hp()
    }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut u = 0;
        let mut annot = "".to_string();
        for (p, tile) in self.grid.iter() {
            if u < self.units.len() && self.units[u].p == p {
                // a unit is in this position
                f.write_char(self.units[u].team.to_char())?;
                annot.push_str(&format!(
                    " {} {}({})",
                    u,
                    self.units[u].team.to_char(),
                    self.units[u].hp
                ));
                u += 1;
            } else {
                f.write_char(match tile {
                    Tile::Open => '.',
                    Tile::Wall => '#',
                })?;
            }
            if p.x >= self.grid.maxx() {
                if !annot.is_empty() {
                    f.write_str(&annot)?;
                    annot.clear();
                }
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        "\
#######
#E..G.#
#...#.#
#.G.#G#
#######
"
    }

    fn test_input2() -> &'static str {
        "\
#######
#.E...#
#.....#
#...G.#
#######
"
    }

    fn test_input3() -> &'static str {
        "\
#########
#G..G..G#
#.......#
#.......#
#G..E..G#
#.......#
#.......#
#G..G..G#
#########
"
    }

    fn test_input4() -> &'static str {
        "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
"
    }

    fn test_input5() -> &'static str {
        "\
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
"
    }

    fn test_input6() -> &'static str {
        "\
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
"
    }

    fn test_input7() -> &'static str {
        "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
"
    }

    fn test_input8() -> &'static str {
        "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
"
    }

    fn test_input9() -> &'static str {
        "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
"
    }

    #[test]
    fn test_move_unit() {
        let mut game = Game::new(test_input());
        println!("{:?}", game);
        game.move_unit(0);
        assert_eq!(P { x: 2, y: 1 }, game.units[0].p);

        let mut game = Game::new(test_input2());
        println!("{:?}", game);
        game.move_unit(0);
        assert_eq!(P { x: 3, y: 1 }, game.units[0].p);

        let mut game = Game::new(test_input3());
        println!("{:?}", game);
        for (u, &(x, y)) in [
            (2, 1),
            (4, 2),
            (6, 1),
            (2, 4),
            (4, 3),
            (7, 3),
            (1, 6),
            (4, 6),
            (7, 6),
        ]
        .iter()
        .enumerate()
        {
            game.move_unit(u);
            assert_eq!(P { x, y }, game.units[u].p);
        }
        game.sort_units();

        // round 2 has units not moving because they are already adjacent
        println!("{:?}", game);
        for (u, &(x, y)) in [
            (3, 1),
            (5, 1),
            (4, 2),
            (4, 3),
            (6, 3),
            (2, 3),
            (1, 5),
            (4, 5),
            (7, 5),
        ]
        .iter()
        .enumerate()
        {
            game.move_unit(u);
            assert_eq!(P { x, y }, game.units[u].p);
        }
        game.sort_units();

        // round 3 has units not moving because there is no path
        println!("{:?}", game);
        for (u, &(x, y)) in [
            (3, 2),
            (5, 2),
            (4, 2),
            (3, 3),
            (4, 3),
            (5, 3),
            (1, 4),
            (4, 4),
            (7, 5),
        ]
        .iter()
        .enumerate()
        {
            game.move_unit(u);
            assert_eq!(P { x, y }, game.units[u].p);
        }
        game.sort_units();

        // in round 4, noone can move at all
        println!("{:?}", game);
        for (u, &(x, y)) in [
            (3, 2),
            (4, 2),
            (5, 2),
            (3, 3),
            (4, 3),
            (5, 3),
            (1, 4),
            (4, 4),
            (7, 5),
        ]
        .iter()
        .enumerate()
        {
            game.move_unit(u);
            assert_eq!(P { x, y }, game.units[u].p);
        }
        game.sort_units();
    }

    #[test]
    fn test_combat() {
        for &(input, expected_rounds, expected_total_hp) in [
            (test_input4(), 47, 590),
            (test_input5(), 37, 982),
            (test_input6(), 46, 859),
            (test_input7(), 35, 793),
            (test_input8(), 54, 536),
            (test_input9(), 20, 937),
        ]
        .iter()
        {
            let mut game = Game::new(input);
            game.simulate(None);
            assert_eq!(expected_rounds, game.rounds);
            assert_eq!(expected_total_hp, game.total_hp());
        }
    }

    #[test]
    fn test_help_elves() {
        for &(input, expected_elf_power, expected_rounds, expected_total_hp) in [
            (test_input4(), 15, 29, 172),
            (test_input6(), 4, 33, 948),
            (test_input7(), 15, 37, 94),
            (test_input8(), 12, 39, 166),
            (test_input9(), 34, 30, 38),
        ]
        .iter()
        {
            let game = Game::help_elves(input);
            assert_eq!(expected_elf_power, game.elf_power);
            assert_eq!(expected_rounds, game.rounds);
            assert_eq!(expected_total_hp, game.total_hp());
        }
    }
}
