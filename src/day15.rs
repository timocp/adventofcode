use crate::Part;
use std::collections::HashMap;

pub fn run(input: &str, part: Part) -> String {
    let ingredients = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => best_score(&ingredients, None),
            Part::Two => best_score(&ingredients, Some(500)),
        }
    )
}

fn best_score(ingredients: &[Ingredient], calories: Option<i32>) -> i32 {
    let len = ingredients.len();
    let mut recipe: Vec<usize> = vec![0; len];
    let mut max = 0;

    loop {
        if match calories {
            Some(c) => c == cookie_calories(ingredients, &recipe),
            None => true,
        } {
            let score = cookie_score(ingredients, &recipe);
            if score > max {
                max = score;
            }
        } else {
        }
        increment(&mut recipe);
        if recipe[len - 2] == 100 {
            return max;
        }
    }
}

fn increment(recipe: &mut [usize]) {
    // current sum (apart from final column)
    let mut current_sum = recipe.iter().take(recipe.len() - 1).sum::<usize>();
    for d in 0..(recipe.len() - 1) {
        if current_sum == 100 {
            current_sum -= recipe[d];
            recipe[d] = 0;
        } else {
            current_sum += 1;
            recipe[d] += 1;
            break;
        }
    }
    recipe[recipe.len() - 1] = 100 - current_sum;
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
    _name: String,
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
            .map(|d| d.split(' ').collect::<Vec<_>>())
            .into_iter()
            .map(|prop| (prop[0], prop[1].parse().unwrap()))
            .collect();
        Self {
            _name: def[0].to_string(),
            capacity: *properties.get("capacity").unwrap(),
            durability: *properties.get("durability").unwrap(),
            flavor: *properties.get("flavor").unwrap(),
            texture: *properties.get("texture").unwrap(),
            calories: *properties.get("calories").unwrap(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Ingredient> {
    input.lines().map(Ingredient::from).collect()
}

#[test]
fn test() {
    let test_input = "\
Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
";
    let ingredients = parse_input(test_input);
    assert_eq!(62842880, best_score(&ingredients, None));
    assert_eq!(57600000, best_score(&ingredients, Some(500)));
}
