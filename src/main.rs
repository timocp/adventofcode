use std::fmt;
use std::io::Read;
use std::time::Instant;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 2 {
        run(args[1].parse().unwrap());
    } else {
        let t0 = Instant::now();
        for day in 1..=25 {
            run(day);
        }
        println!(
            "{:>80}",
            format!("TOTAL: {:.3}s", t0.elapsed().as_secs_f64())
        );
    }
}

fn run(day: usize) {
    let filename = format!("input/day{}.txt", day);
    if let Ok(input) = read_file(&filename) {
        for part in [Part::One, Part::Two] {
            print!("Day {:02}, part {}:  ", day, part);
            let t0 = Instant::now();
            let result = match day {
                1 => day1::run(&input, part),
                2 => day2::run(&input, part),
                3 => day3::run(&input, part),
                4 => day4::run(&input, part),
                5 => day5::run(&input, part),
                _ => "Not implemented".to_string(),
            };
            println!(
                "{:56} {1:.3}s",
                if result.contains('\n') {
                    result.lines().next().unwrap()
                } else {
                    &result
                },
                t0.elapsed().as_secs_f64()
            );
            if result.contains('\n') {
                for line in result.lines().skip(1) {
                    println!("{:17}{}", "", line);
                }
            }
        }
    }
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
