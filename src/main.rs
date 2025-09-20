use std::fmt;
use std::io::Read;
use std::time::Instant;

mod y2015;

#[derive(Eq, PartialEq)]
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

pub trait Puzzle {
    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 3 {
        run(&args[1], args[2].parse().unwrap());
    } else if args.len() == 2 {
        let t0 = Instant::now();
        for day in 1..=25 {
            run(&args[1], day);
        }
        println!(
            "{:>80}",
            format!("TOTAL: {:.3}s", t0.elapsed().as_secs_f64())
        );
    } else {
        eprintln!("Usage: cargo run year [day]");
        std::process::exit(1);
    }
}

type RunFn = fn(usize, &str, Part) -> String;

fn solve<P: Puzzle>(year: &str, day: usize, input: &str) {
    let t0 = Instant::now();
    let puzzle: P = P::new(input);
    measure(&format!("EXP {} day {:01} part 1", year, day), t0, || {
        puzzle.part1()
    });
    let t0 = Instant::now();
    measure(&format!("EXP {} day {:01} part 2", year, day), t0, || {
        puzzle.part2()
    });
}

fn run(year: &str, day: usize) {
    let run: RunFn = match year {
        "2015" => y2015::run,
        _ => panic!("Unimplemented year: {}", year),
    };

    let filename = format!("input/{}/day{}.txt", year, day);

    if let Ok(input) = read_file(&filename) {
        if year == "2015" && day <= 2 {
            // experimental puzzle/solver traits
            match (year, day) {
                ("2015", 1) => solve::<y2015::day1::Solver>(year, day, &input),
                ("2015", 2) => solve::<y2015::day2::Solver>(year, day, &input),
                (&_, _) => todo!(),
            }
        } else {
            for part in [Part::One, Part::Two] {
                print!("{} Day {:02}, part {}:  ", year, day, part);
                let t0 = Instant::now();
                let result = run(day, &input, part);
                println!(
                    "{:51} {1:.3}s",
                    if result.contains('\n') {
                        result.lines().next().unwrap()
                    } else {
                        &result
                    },
                    t0.elapsed().as_secs_f64()
                );
                if result.contains('\n') {
                    for line in result.lines().skip(1) {
                        println!("{:21}{}", "", line);
                    }
                }
            }
        }
    } else {
        eprintln!("Can't read {}", filename);
    }
}

fn measure<F>(label: &str, t0: Instant, f: F)
where
    F: FnOnce() -> String,
{
    print!("{}: ", label);
    let result = f();
    println!(
        "{:51} {1:.3}s",
        if result.contains('\n') {
            result.lines().next().unwrap()
        } else {
            &result
        },
        t0.elapsed().as_secs_f64()
    );
    if result.contains('\n') {
        for line in result.lines().skip(1) {
            println!("{:21}{}", "", line);
        }
    }
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
