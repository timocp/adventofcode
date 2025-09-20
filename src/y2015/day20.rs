pub struct Solver {
    min_presents: u32,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            min_presents: parse_input(input),
        }
    }

    fn part1(&self) -> String {
        part1(self.min_presents).to_string()
    }

    fn part2(&self) -> String {
        part2(self.min_presents).to_string()
    }
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

fn part2(min_presents: u32) -> u32 {
    let mut elf = 1u32;
    let mut count = vec![0u32]; // house 0 doesn't exist

    loop {
        if elf as usize * 50 + 1 > count.len() {
            count.resize(elf as usize * 50 + 1, 0);
        }
        for gift in 1..=50u32 {
            let h = gift * elf;
            count[h as usize] += elf * 11;
        }

        // house #elf can no longer receive gifts, so check if it has received enough
        if count[elf as usize] >= min_presents {
            return elf;
        }

        elf += 1;
    }
}

fn presents(house: u32) -> u32 {
    let mut presents = 0;
    let sqrt = (house as f64).sqrt() as u32;
    for elf in 1..=sqrt {
        if house.is_multiple_of(elf) {
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
