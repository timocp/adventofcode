use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => combinations(&input, 0, 150),
            Part::Two => 0,
        }
    )
}

fn combinations(input: &Vec<i32>, index: usize, target: i32) -> u32 {
    if target == 0 {
        return 1;
    }
    if target < 0 {
        return 0;
    }
    if index == input.len() {
        return 0;
    }
    // combinations over the rest of the index that:
    //  include this container + don't include this container
    combinations(input, index + 1, target - input[index]) + combinations(input, index + 1, target)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[test]
fn test() {
    let test_input = vec![20, 15, 10, 5, 5];
    assert_eq!(combinations(&test_input, 0, 25), 4);
}
