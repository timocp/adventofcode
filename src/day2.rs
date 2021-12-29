use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let input = parse_input(input);
    format!(
        "{}",
        match part {
            Part::One => total_paper(&input),
            Part::Two => 0,
        }
    )
}

struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl From<&str> for Present {
    fn from(s: &str) -> Self {
        let dims: Vec<u32> = s.split('x').map(|n| n.parse().unwrap()).collect();
        Self {
            l: dims[0],
            w: dims[1],
            h: dims[2],
        }
    }
}

impl Present {
    fn wrapping_paper(&self) -> u32 {
        let sides: Vec<u32> = vec![self.l * self.w, self.w * self.h, self.h * self.l];
        sides.iter().map(|s| s * 2).sum::<u32>() + sides.iter().min().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Present> {
    input.lines().map(Present::from).collect()
}

fn total_paper(presents: &[Present]) -> u32 {
    presents.iter().map(|p| p.wrapping_paper()).sum()
}

#[test]
fn test() {
    assert_eq!(58, total_paper(&parse_input("2x3x4\n")));
    assert_eq!(43, total_paper(&parse_input("1x1x10\n")));
}
