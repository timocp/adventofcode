use crate::Part;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let ingredients = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => part1(&ingredients),
            Part::Two => part2(&ingredients),
        }
    )
}

fn part1(ingredients: &[Ingredient]) -> i32 {
    let len = ingredients.len();
    let mut recipe: Vec<usize> = vec![0; len];
    let mut max = 0;

    // TODO: dynamic based on length of ingredients
    for r0 in 0..=100 {
        for r1 in 0..=(100 - r0) {
            for r2 in 0..=(100 - (r0 + r1)) {
                recipe[0] = r0;
                recipe[1] = r1;
                recipe[2] = r2;
                recipe[3] = 100 - (r0 + r1 + r2);
                let score = cookie_score(ingredients, &recipe);
                if score > max {
                    max = score;
                }
            }
        }
    }
    max
}

fn part2(ingredients: &[Ingredient]) -> i32 {
    let len = ingredients.len();
    let mut recipe: Vec<usize> = vec![0; len];
    let mut max = 0;

    // TODO: dynamic based on length of ingredients
    for r0 in 0..=100 {
        for r1 in 0..=(100 - r0) {
            for r2 in 0..=(100 - (r0 + r1)) {
                recipe[0] = r0;
                recipe[1] = r1;
                recipe[2] = r2;
                recipe[3] = 100 - (r0 + r1 + r2);
                let calories = cookie_calories(ingredients, &recipe);
                if calories == 500 {
                    let score = cookie_score(ingredients, &recipe);
                    if score > max {
                        max = score;
                    }
                }
            }
        }
    }
    max
}

fn cookie_score(ingredients: &[Ingredient], recipe: &[usize]) -> i32 {
    let capacity: i32 = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.capacity * recipe[i] as i32)
        .sum();
    let durability: i32 = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.durability * recipe[i] as i32)
        .sum();
    let flavor: i32 = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.flavor * recipe[i] as i32)
        .sum();
    let texture: i32 = ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.texture * recipe[i] as i32)
        .sum();
    positive(capacity) * positive(durability) * positive(flavor) * positive(texture)
}

fn cookie_calories(ingredients: &[Ingredient], recipe: &[usize]) -> i32 {
    ingredients
        .iter()
        .enumerate()
        .map(|(i, v)| v.calories * recipe[i] as i32)
        .sum()
}

fn positive(i: i32) -> i32 {
    if i < 0 {
        0
    } else {
        i
    }
}

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl From<&str> for Ingredient {
    fn from(s: &str) -> Self {
        let def: Vec<&str> = s.split(": ").collect();
        let properties: HashMap<&str, i32> = def[1]
            .split(", ")
            .map(|d| d.split(" ").collect::<Vec<_>>())
            .into_iter()
            .map(|prop| return (prop[0], prop[1].parse().unwrap()))
            .collect();
        Self {
            name: def[0].to_string(),
            capacity: *properties.get("capacity").unwrap(),
            durability: *properties.get("durability").unwrap(),
            flavor: *properties.get("flavor").unwrap(),
            texture: *properties.get("texture").unwrap(),
            calories: *properties.get("calories").unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input.lines().map(|line| Ingredient::from(line)).collect()
}

#[test]
fn test() {
    let test_input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
Dummy1: capacity 0, durability 0, flavor 0, texture 0, calories 0
Dummy2: capacity 0, durability 0, flavor 0, texture 0, calories 0
";
    let ingredients = parse_input(test_input);
    assert_eq!(62842880, part1(&ingredients));
    assert_eq!(57600000, part2(&ingredients));
}
