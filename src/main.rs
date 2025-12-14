use std::io::{self, Read, Write};
use std::time::Instant;

mod bfs;
mod dijkstra;
mod grid;
mod numeric;
mod pixel_buffer;

mod y2015;
mod y2016;
mod y2018;
mod y2019;
mod y2021;
mod y2025;

pub trait Puzzle {
    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

struct Solver {
    year: u32,
    day: u32,
    run: fn(&str),
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let solvers = all_solvers();

    if args.len() == 3 {
        run(&solvers, args[1].parse().unwrap(), args[2].parse().unwrap());
    } else if args.len() == 2 {
        let t0 = Instant::now();
        let year = args[1].parse().unwrap();
        for day in 1..=25 {
            run(&solvers, year, day);
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

fn must_extract_year(s: &str) -> u32 {
    s.strip_prefix("y").unwrap().parse().unwrap()
}

fn must_extract_day(s: &str) -> u32 {
    s.strip_prefix("day").unwrap().parse().unwrap()
}

macro_rules! solvers {
    ($year:tt $($day:tt),*) => {
        vec![$(
        {
            let year = must_extract_year(stringify!($year));
            let day = must_extract_day(stringify!($day));
            Solver {
                year,
                day,
                run: |input: &str| {
                    let year = must_extract_year(stringify!($year));
                    let day = must_extract_day(stringify!($day));
                    let t0 = Instant::now();
                    let puzzle = $year::$day::Solver::new(input);
                    measure(&format!("{} day {:02} part 1", year, day), t0, || {
                        puzzle.part1()
                    });
                    let t0 = Instant::now();
                    measure(&format!("{} day {:02} part 2", year, day), t0, || {
                        puzzle.part2()
                    });
                },
            }
        }
        ,)*]
    };
}

fn all_solvers() -> Vec<Solver> {
    vec![
        solvers!(y2015
            day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
            day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
        ),
        solvers!(y2016
            day1, day2, day3, day4, day5, day6, day7, day8
        ),
        solvers!(y2018
            day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
            day14, day15, day16
        ),
        solvers!(y2019
            day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
            day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
        ),
        solvers!(y2021
            day1, day2, day3, day4, day5, day6, day7, day8, day9, day10, day11, day12, day13,
            day14, day15, day16, day17, day18, day19, day20, day21, day22, day23, day24, day25
        ),
        solvers!(y2025
            day1, day2, day3, day4
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

fn run(solvers: &[Solver], year: u32, day: u32) {
    if let Some(solver) = solvers.iter().find(|s| s.year == year && s.day == day) {
        let filename = format!("input/{}/day{}.txt", year, day);
        if let Ok(input) = read_file(&filename) {
            (solver.run)(&input)
        } else {
            eprintln!("{} day {:02}: Can't read {}", year, day, filename);
        }
    }
}

fn measure<F>(label: &str, t0: Instant, f: F)
where
    F: FnOnce() -> String,
{
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let result = f();
    println!(
        "{:53} {1:5.2}s",
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
