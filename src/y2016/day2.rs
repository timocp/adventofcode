pub fn part1(input: &[Vec<Direction>]) -> String {
    solve(input, normal_keypad)
}

pub fn part2(input: &[Vec<Direction>]) -> String {
    solve(input, diamond_keypad)
}

fn normal_keypad(pos: char, dir: Direction) -> char {
    match (pos, dir) {
        ('1', Direction::Down) => '4',
        ('1', Direction::Right) => '2',
        ('2', Direction::Down) => '5',
        ('2', Direction::Left) => '1',
        ('2', Direction::Right) => '3',
        ('3', Direction::Down) => '6',
        ('3', Direction::Left) => '2',
        ('4', Direction::Up) => '1',
        ('4', Direction::Down) => '7',
        ('4', Direction::Right) => '5',
        ('5', Direction::Up) => '2',
        ('5', Direction::Down) => '8',
        ('5', Direction::Left) => '4',
        ('5', Direction::Right) => '6',
        ('6', Direction::Up) => '3',
        ('6', Direction::Down) => '9',
        ('6', Direction::Left) => '5',
        ('7', Direction::Up) => '4',
        ('7', Direction::Right) => '8',
        ('8', Direction::Up) => '5',
        ('8', Direction::Left) => '7',
        ('8', Direction::Right) => '9',
        ('9', Direction::Up) => '6',
        ('9', Direction::Left) => '8',
        _ => pos,
    }
}

fn diamond_keypad(pos: char, dir: Direction) -> char {
    match (pos, dir) {
        ('1', Direction::Down) => '3',
        ('2', Direction::Down) => '6',
        ('2', Direction::Right) => '3',
        ('3', Direction::Up) => '1',
        ('3', Direction::Down) => '7',
        ('3', Direction::Left) => '2',
        ('3', Direction::Right) => '4',
        ('4', Direction::Down) => '8',
        ('4', Direction::Left) => '3',
        ('5', Direction::Right) => '6',
        ('6', Direction::Up) => '2',
        ('6', Direction::Down) => 'A',
        ('6', Direction::Left) => '5',
        ('6', Direction::Right) => '7',
        ('7', Direction::Up) => '3',
        ('7', Direction::Down) => 'B',
        ('7', Direction::Left) => '6',
        ('7', Direction::Right) => '8',
        ('8', Direction::Up) => '4',
        ('8', Direction::Down) => 'C',
        ('8', Direction::Left) => '7',
        ('8', Direction::Right) => '9',
        ('9', Direction::Left) => '8',
        ('A', Direction::Up) => '6',
        ('A', Direction::Right) => 'B',
        ('B', Direction::Up) => '7',
        ('B', Direction::Down) => 'D',
        ('B', Direction::Left) => 'A',
        ('B', Direction::Right) => 'C',
        ('C', Direction::Up) => '8',
        ('C', Direction::Left) => 'B',
        ('D', Direction::Up) => 'B',
        _ => pos,
    }
}

fn solve(input: &[Vec<Direction>], keypad: fn(char, Direction) -> char) -> String {
    let mut pos = '5';
    let mut output = "".to_string();
    for line in input {
        for dir in line {
            pos = keypad(pos, *dir);
        }
        output.push(pos);
    }

    output
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_input(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unexpected character: {}", c),
                })
                .collect()
        })
        .collect()
}

#[test]
fn test() {
    let test_input = "ULL\nRRDDD\nLURDL\nUUUUD\n";
    assert_eq!("1985", part1(&parse_input(test_input)));
    assert_eq!("5DB3", part2(&parse_input(test_input)));
}
