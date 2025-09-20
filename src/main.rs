use std::io::Read;
use std::time::Instant;

mod y2015;

pub trait Puzzle {
    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() == 3 {
        run(args[1].parse().unwrap(), args[2].parse().unwrap());
    } else if args.len() == 2 {
        let t0 = Instant::now();
        for day in 1..=25 {
            run(args[1].parse().unwrap(), day);
        }
        println!(
            "{:>80}",
            format!("TOTAL: {:.2}s", t0.elapsed().as_secs_f64())
        );
    } else {
        eprintln!("Usage: cargo run year [day]");
        std::process::exit(1);
    }
}

fn solve<P: Puzzle>(year: usize, day: usize, input: &str) {
    let t0 = Instant::now();
    let puzzle: P = P::new(input);
    measure(&format!("{} day {:02} part 1", year, day), t0, || {
        puzzle.part1()
    });
    let t0 = Instant::now();
    measure(&format!("{} day {:02} part 2", year, day), t0, || {
        puzzle.part2()
    });
}

fn run(year: usize, day: usize) {
    let filename = format!("input/{}/day{}.txt", year, day);

    if let Ok(input) = read_file(&filename) {
        match (year, day) {
            (2015, 1) => solve::<y2015::day1::Solver>(year, day, &input),
            (2015, 2) => solve::<y2015::day2::Solver>(year, day, &input),
            (2015, 3) => solve::<y2015::day3::Solver>(year, day, &input),
            (2015, 4) => solve::<y2015::day4::Solver>(year, day, &input),
            (2015, 5) => solve::<y2015::day5::Solver>(year, day, &input),
            (2015, 6) => solve::<y2015::day6::Solver>(year, day, &input),
            (2015, 7) => solve::<y2015::day7::Solver>(year, day, &input),
            (2015, 8) => solve::<y2015::day8::Solver>(year, day, &input),
            (2015, 9) => solve::<y2015::day9::Solver>(year, day, &input),
            (2015, 10) => solve::<y2015::day10::Solver>(year, day, &input),
            (2015, 11) => solve::<y2015::day11::Solver>(year, day, &input),
            (2015, 12) => solve::<y2015::day12::Solver>(year, day, &input),
            (2015, 13) => solve::<y2015::day13::Solver>(year, day, &input),
            (2015, 14) => solve::<y2015::day14::Solver>(year, day, &input),
            (2015, 15) => solve::<y2015::day15::Solver>(year, day, &input),
            (2015, 16) => solve::<y2015::day16::Solver>(year, day, &input),
            (2015, 17) => solve::<y2015::day17::Solver>(year, day, &input),
            (2015, 18) => solve::<y2015::day18::Solver>(year, day, &input),
            (2015, 19) => solve::<y2015::day19::Solver>(year, day, &input),
            (2015, 20) => solve::<y2015::day20::Solver>(year, day, &input),
            (2015, 21) => solve::<y2015::day21::Solver>(year, day, &input),
            (2015, 22) => solve::<y2015::day22::Solver>(year, day, &input),
            (2015, 23) => solve::<y2015::day23::Solver>(year, day, &input),
            (2015, 24) => solve::<y2015::day24::Solver>(year, day, &input),
            (2015, 25) => solve::<y2015::day25::Solver>(year, day, &input),
            (_, _) => todo!(),
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
        "{:54} {1:.2}s",
        if result.contains('\n') {
            result.lines().next().unwrap()
        } else {
            &result
        },
        t0.elapsed().as_secs_f64()
    );
    if result.contains('\n') {
        for line in result.lines().skip(1) {
            println!("{:20}{}", "", line);
        }
    }
}

fn read_file(filename: &str) -> Result<String, std::io::Error> {
    let mut f = std::fs::File::open(filename)?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    Ok(input)
}
