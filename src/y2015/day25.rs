use crate::Part;

use regex::Regex;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    match part {
        Part::One => format!("{}", calc_code(input.0, input.1)),
        Part::Two => "N/A".to_string(),
    }
}

fn calc_code(row: u32, col: u32) -> u64 {
    let mut code = 20151125;
    let index = (row + col - 2) * (row + col - 1) / 2 + col - 1;
    for _ in 0..index {
        code = code * 252533 % 33554393;
    }
    code
}

fn parse_input(input: &str) -> (u32, u32) {
    let re = Regex::new(r"row (\d+), column (\d+)").unwrap();
    let caps = re.captures(input).unwrap();
    (caps[1].parse().unwrap(), caps[2].parse().unwrap())
}

#[test]
fn test() {
    let expected = [
        [20151125, 18749137, 17289845, 30943339, 10071777, 33511524],
        [31916031, 21629792, 16929656, 7726640, 15514188, 4041754],
        [16080970, 8057251, 1601130, 7981243, 11661866, 16474243],
        [24592653, 32451966, 21345942, 9380097, 10600672, 31527494],
        [77061, 17552253, 28094349, 6899651, 9250759, 31663883],
        [33071741, 6796745, 25397450, 24659492, 1534922, 27995004],
    ];
    for row in 1..6 {
        for col in 1..6 {
            assert_eq!(
                expected[row - 1][col - 1],
                calc_code(row as u32, col as u32)
            );
        }
    }
}
