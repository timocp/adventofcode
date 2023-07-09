use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    format!(
        "{}",
        match part {
            Part::One => part1(parse_input(input)),
            Part::Two => 0,
        }
    )
}

fn part1(min_presents: u32) -> u32 {
    let mut house = 1;
    loop {
        if presents(house) >= min_presents {
            return house;
        }
        house += 1;
    }
}

fn presents(house: u32) -> u32 {
    let mut presents = 0;
    let sqrt = (house as f64).sqrt() as u32;
    for elf in 1..=sqrt {
        if house % elf == 0 {
            if elf * elf == house {
                presents += elf * 10;
                continue;
            } else {
                presents += elf * 10;
                presents += (house / elf) * 10;
            }
        }
    }
    presents
}

fn parse_input(input: &str) -> u32 {
    input.trim().parse().unwrap()
}

#[test]
fn test() {
    assert_eq!(presents(1), 10);
    assert_eq!(presents(2), 30);
    assert_eq!(presents(3), 40);
    assert_eq!(presents(4), 70);
    assert_eq!(presents(5), 60);
    assert_eq!(presents(6), 120);
    assert_eq!(presents(7), 80);
    assert_eq!(presents(8), 150);
    assert_eq!(presents(9), 130);
}
