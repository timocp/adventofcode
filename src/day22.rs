use crate::Part;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

pub fn run(input: &str, part: Part) -> String {
    let state = parse_input(input);
    format!(
        "{}",
        find_cheapest_mana_win(&match part {
            Part::One => state,
            Part::Two => state.hard(),
        })
        .unwrap()
    )
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

impl Spell {
    fn cost(&self) -> i32 {
        match self {
            Spell::MagicMissile => 53,
            Spell::Drain => 73,
            Spell::Shield => 113,
            Spell::Poison => 173,
            Spell::Recharge => 229,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct GameState {
    player_hp: i32,
    player_mana: i32,
    spent_mana: i32,
    boss_hp: i32,
    boss_damage: i32,
    shield_timer: i32,
    poison_timer: i32,
    recharge_timer: i32,
    trace: bool,
    hard_mode: bool,
    history: Vec<Spell>, // debugging only.
}

impl GameState {
    fn new(player_hp: i32, player_mana: i32, boss_hp: i32, boss_damage: i32) -> Self {
        Self {
            player_hp,
            player_mana,
            spent_mana: 0,
            boss_hp,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            trace: false,
            hard_mode: false,
            history: vec![],
        }
    }

    fn hard(&self) -> GameState {
        let mut state = self.clone();
        state.hard_mode = true;
        state
    }

    fn valid_spells(&self) -> Vec<Spell> {
        let mut moves = Vec::new();
        if self.player_mana >= Spell::MagicMissile.cost() {
            moves.push(Spell::MagicMissile);
        }
        if self.player_mana >= Spell::Drain.cost() {
            moves.push(Spell::Drain);
        }
        if self.player_mana >= Spell::Shield.cost() && self.shield_timer <= 1 {
            moves.push(Spell::Shield);
        }
        if self.player_mana >= Spell::Poison.cost() && self.poison_timer <= 1 {
            moves.push(Spell::Poison);
        }
        if self.player_mana >= Spell::Recharge.cost() && self.recharge_timer <= 1 {
            moves.push(Spell::Recharge);
        }
        moves
    }

    fn output_stats(&self) {
        println!(
            "- Player has {} hit points, {} armor, {} mana",
            self.player_hp,
            self.player_armor(),
            self.player_mana
        );
        println!("- Boss has {} hit points", self.boss_hp);
    }

    fn player_turn(&self, spell: Spell) -> GameState {
        if self.trace {
            println!("-- Player turn --");
            self.output_stats();
        }

        let mut next_state = self.clone();
        if self.hard_mode {
            next_state.player_hp -= 1;
            if self.trace {
                println!("Player loses 1 HP to hard mode.");
            }
            if next_state.player_hp <= 0 {
                if self.trace {
                    println!("Player dies to hard mode.");
                }
                return next_state;
            }
        }

        next_state.history.push(spell.clone());
        next_state.apply_effects();
        if next_state.boss_hp > 0 {
            if self.trace {
                match spell {
                    Spell::MagicMissile => {
                        println!("Player casts Magic Missile, dealing 4 damage.")
                    }
                    Spell::Drain => {
                        println!("Player casts Drain, dealing 2 damage, and healing 2 hit points.")
                    }
                    Spell::Shield => println!("Player casts Shield, increasing armor by 7."),
                    Spell::Poison => println!("Player casts Poison."),
                    Spell::Recharge => println!("Player casts Recharge."),
                }
            }
            next_state.player_mana -= spell.cost();
            if next_state.player_mana < 0 {
                panic!("Player has negative mana!");
            }
            next_state.spent_mana += spell.cost();
            match spell {
                Spell::MagicMissile => next_state.boss_hp -= 4,
                Spell::Drain => {
                    next_state.boss_hp -= 2;
                    next_state.player_hp += 2;
                }
                Spell::Shield => next_state.shield_timer = 6,
                Spell::Poison => next_state.poison_timer = 6,
                Spell::Recharge => next_state.recharge_timer = 5,
            }
        }
        next_state
    }

    fn player_armor(&self) -> i32 {
        if self.shield_timer > 0 {
            7
        } else {
            0
        }
    }

    fn boss_turn(&self) -> GameState {
        if self.trace {
            println!("-- Boss turn --");
            self.output_stats();
        }
        let mut next_state = self.clone();
        next_state.apply_effects();
        if next_state.boss_hp > 0 {
            let damage = (self.boss_damage - self.player_armor()).max(1);
            if self.trace {
                if self.player_armor() > 0 {
                    println!(
                        "Boss attacks for {} - {} = {} damage!",
                        self.boss_damage,
                        self.player_armor(),
                        damage
                    );
                } else {
                    println!("Boss attacks for {} damage!", damage);
                }
            }
            next_state.player_hp -= damage;
        }
        next_state
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
            if self.trace {
                println!("Shield's timer is now {}.", self.shield_timer);
                if self.shield_timer == 0 {
                    println!("Shield wears off, decreasing armor by 7.");
                }
            }
        }
        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
            if self.trace {
                println!(
                    "Poison deals 3 damage; its timer is now {}.",
                    self.poison_timer
                );
            }
        }
        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
            if self.trace {
                println!(
                    "Recharge provides 101 mana; its timer is now {}.",
                    self.recharge_timer
                );
                if self.recharge_timer == 0 {
                    println!("Recharge wears off.");
                }
            }
        }
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.spent_mana.cmp(&other.spent_mana).reverse()
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_cheapest_mana_win(initial_state: &GameState) -> Option<i32> {
    let mut frontier = BinaryHeap::new();
    frontier.push(initial_state.clone());
    let mut states = 0;
    while let Some(state) = frontier.pop() {
        states += 1;
        if state.boss_hp <= 0 {
            panic!("should never pop a state where boss_hp <= 0");
        }
        let spells = state.valid_spells();
        if spells.is_empty() {
            continue;
        }
        for spell in state.valid_spells() {
            let next_state = state.player_turn(spell);
            if next_state.boss_hp <= 0 {
                println!(
                    "**** found win with spent_mana = {} (A)",
                    next_state.spent_mana
                );
                println!("     history: {:?}", next_state.history);
                // replay(initial_state, &next_state.history);
                return Some(next_state.spent_mana);
            }
            let next_state = next_state.boss_turn();
            if next_state.boss_hp <= 0 {
                println!(
                    "**** found win with spent_mana = {} (B)",
                    next_state.spent_mana
                );
                println!("     history: {:?}", next_state.history);
                // replay(initial_state, &next_state.history);
                return Some(next_state.spent_mana);
            }
            if next_state.player_hp <= 0 {
                continue;
            }
            frontier.push(next_state);
        }
    }

    println!("Exhausted after {} states", states);
    None
}

// This was only used for debugging the rule implementaion
fn replay(initial_state: &GameState, history: &[Spell]) {
    let mut state = initial_state.clone();
    state.trace = true;
    for spell in history {
        state = state.player_turn(spell.clone());
        if state.player_hp <= 0 {
            println!("Boss wins!");
            break;
        }
        println!();

        if state.boss_hp <= 0 {
            println!("Player wins!");
            break;
        }
        state = state.boss_turn();
        println!();
        if state.player_hp <= 0 && state.boss_hp <= 0 {
            println!("Both player and boss are dead!");
            break;
        }
        if state.player_hp <= 0 {
            println!("Boss wins!");
            break;
        }
        if state.boss_hp <= 0 {
            println!("Player wins!");
            break;
        }
    }
    println!("Mana spent was: {}", state.spent_mana);
}

fn parse_input(input: &str) -> GameState {
    let mut boss_hp = 0;
    let mut boss_damage = 0;
    for line in input.lines() {
        let mut parts = line.split(": ");
        let stat = parts.next().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        match stat {
            "Hit Points" => boss_hp = value,
            "Damage" => boss_damage = value,
            _ => panic!("Unknown stat: {}", stat),
        }
    }
    GameState::new(50, 500, boss_hp, boss_damage)
}

#[test]
fn test() {
    // assert_eq!(
    //     find_cheapest_mana_win(&GameState::new(10, 250, 13, 8)),
    //     Some(226)
    // );
    assert_eq!(
        find_cheapest_mana_win(&GameState::new(10, 250, 14, 8)),
        Some(641)
    );
}
