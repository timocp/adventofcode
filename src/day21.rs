use itertools::Itertools;

use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let result = solve(&parse_input(input));
    format!(
        "{}",
        match part {
            Part::One => result.0,
            Part::Two => result.1,
        }
    )
}

#[derive(Debug)]
struct Boss {
    hit_points: i32,
    damage: i32,
    armor: i32,
}

fn solve(boss: &Boss) -> (i32, i32) {
    // iterate over all choices, there's not that many
    let mut cheapest_win = i32::MAX;
    let mut most_expensive_loss = 0;

    for weapon in weapons() {
        for armor in armors() {
            for rings in rings().iter().combinations(2) {
                let cost = weapon.cost + armor.cost + rings[0].cost + rings[1].cost;
                if cost >= cheapest_win && cost <= most_expensive_loss {
                    continue;
                }
                let player_damage = weapon.damage + rings[0].damage + rings[1].damage;
                let player_armor = armor.armor + rings[0].armor + rings[1].armor;
                let player_win = fight(boss, player_damage, player_armor);
                if player_win && cost < cheapest_win {
                    cheapest_win = cost;
                } else if !player_win && cost > most_expensive_loss {
                    most_expensive_loss = cost;
                }
            }
        }
    }

    (cheapest_win, most_expensive_loss)
}

// return true if player will win this fight
fn fight(boss: &Boss, player_damage: i32, player_armor: i32) -> bool {
    let mut player_hp = 100;
    let mut boss_hp = boss.hit_points;
    loop {
        boss_hp -= calc_damage(player_damage, boss.armor);
        if boss_hp <= 0 {
            return true;
        }
        player_hp -= calc_damage(boss.damage, player_armor);
        if player_hp <= 0 {
            return false;
        }
    }
}

fn calc_damage(damage: i32, armor: i32) -> i32 {
    if damage > armor {
        damage - armor
    } else {
        1
    }
}

#[derive(Debug)]
struct Item {
    cost: i32,
    damage: i32,
    armor: i32,
    #[allow(dead_code)]
    descr: &'static str,
}

fn weapons() -> Vec<Item> {
    vec![
        Item {
            cost: 8,
            damage: 4,
            armor: 0,
            descr: "Dagger",
        },
        Item {
            cost: 10,
            damage: 5,
            armor: 0,
            descr: "Shortsword",
        },
        Item {
            cost: 25,
            damage: 6,
            armor: 0,
            descr: "Warhammer",
        },
        Item {
            cost: 40,
            damage: 7,
            armor: 0,
            descr: "Longsword",
        },
        Item {
            cost: 74,
            damage: 8,
            armor: 0,
            descr: "Greataxe",
        },
    ]
}

fn armors() -> Vec<Item> {
    vec![
        Item {
            cost: 13,
            damage: 0,
            armor: 1,
            descr: "Leather",
        },
        Item {
            cost: 31,
            damage: 0,
            armor: 2,
            descr: "Chainmail",
        },
        Item {
            cost: 53,
            damage: 0,
            armor: 3,
            descr: "Splintmail",
        },
        Item {
            cost: 75,
            damage: 0,
            armor: 4,
            descr: "Bandedmail",
        },
        Item {
            cost: 102,
            damage: 0,
            armor: 5,
            descr: "Platemail",
        },
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
            descr: "No armor",
        },
    ]
}

fn rings() -> Vec<Item> {
    vec![
        Item {
            cost: 25,
            damage: 1,
            armor: 0,
            descr: "Damage +1",
        },
        Item {
            cost: 50,
            damage: 2,
            armor: 0,
            descr: "Damage +2",
        },
        Item {
            cost: 100,
            damage: 3,
            armor: 0,
            descr: "Damage +3",
        },
        Item {
            cost: 20,
            damage: 0,
            armor: 1,
            descr: "Defense +1",
        },
        Item {
            cost: 40,
            damage: 0,
            armor: 2,
            descr: "Defense +2",
        },
        Item {
            cost: 80,
            damage: 0,
            armor: 3,
            descr: "Defense +3",
        },
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
            descr: "No left ring",
        },
        Item {
            cost: 0,
            damage: 0,
            armor: 0,
            descr: "No right ring",
        },
    ]
}

fn parse_input(input: &str) -> Boss {
    let mut hit_points = 0;
    let mut damage = 0;
    let mut armor = 0;
    for line in input.lines() {
        let data = line.split(": ").collect::<Vec<_>>();
        match data[0] {
            "Hit Points" => hit_points = data[1].parse().unwrap(),
            "Damage" => damage = data[1].parse().unwrap(),
            "Armor" => armor = data[1].parse().unwrap(),
            _ => panic!("Unknown input: {}", line),
        }
    }
    Boss {
        hit_points,
        damage,
        armor,
    }
}
