use std::io::Read;
use std::time::Instant;

mod pixel_buffer;
mod y2015;
mod y2016;
mod y2018;
mod y2019;
mod y2021;

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
            (2016, 1) => solve::<y2016::day1::Solver>(year, day, &input),
            (2016, 2) => solve::<y2016::day2::Solver>(year, day, &input),
            (2016, 3) => solve::<y2016::day3::Solver>(year, day, &input),
            (2016, 4) => solve::<y2016::day4::Solver>(year, day, &input),
            (2016, 5) => solve::<y2016::day5::Solver>(year, day, &input),
            (2016, 6) => solve::<y2016::day6::Solver>(year, day, &input),
            (2016, 7) => solve::<y2016::day7::Solver>(year, day, &input),
            (2016, 8) => solve::<y2016::day8::Solver>(year, day, &input),
            (2018, 1) => solve::<y2018::day1::Solver>(year, day, &input),
            (2018, 2) => solve::<y2018::day2::Solver>(year, day, &input),
            (2018, 3) => solve::<y2018::day3::Solver>(year, day, &input),
            (2018, 4) => solve::<y2018::day4::Solver>(year, day, &input),
            (2018, 5) => solve::<y2018::day5::Solver>(year, day, &input),
            (2018, 6) => solve::<y2018::day6::Solver>(year, day, &input),
            (2018, 7) => solve::<y2018::day7::Solver>(year, day, &input),
            (2018, 8) => solve::<y2018::day8::Solver>(year, day, &input),
            (2018, 9) => solve::<y2018::day9::Solver>(year, day, &input),
            (2018, 10) => solve::<y2018::day10::Solver>(year, day, &input),
            (2018, 11) => solve::<y2018::day11::Solver>(year, day, &input),
            (2018, 12) => solve::<y2018::day12::Solver>(year, day, &input),
            (2018, 13) => solve::<y2018::day13::Solver>(year, day, &input),
            (2018, 14) => solve::<y2018::day14::Solver>(year, day, &input),
            (2018, 15) => solve::<y2018::day15::Solver>(year, day, &input),
            (2018, 16) => solve::<y2018::day16::Solver>(year, day, &input),
            (2019, 1) => solve::<y2019::day1::Solver>(year, day, &input),
            (2019, 2) => solve::<y2019::day2::Solver>(year, day, &input),
            (2019, 3) => solve::<y2019::day3::Solver>(year, day, &input),
            (2019, 4) => solve::<y2019::day4::Solver>(year, day, &input),
            (2019, 5) => solve::<y2019::day5::Solver>(year, day, &input),
            (2019, 6) => solve::<y2019::day6::Solver>(year, day, &input),
            (2019, 7) => solve::<y2019::day7::Solver>(year, day, &input),
            (2019, 8) => solve::<y2019::day8::Solver>(year, day, &input),
            (2019, 9) => solve::<y2019::day9::Solver>(year, day, &input),
            (2021, 1) => solve::<y2021::day1::Solver>(year, day, &input),
            (2021, 2) => solve::<y2021::day2::Solver>(year, day, &input),
            (2021, 3) => solve::<y2021::day3::Solver>(year, day, &input),
            (2021, 4) => solve::<y2021::day4::Solver>(year, day, &input),
            (2021, 5) => solve::<y2021::day5::Solver>(year, day, &input),
            (2021, 6) => solve::<y2021::day6::Solver>(year, day, &input),
            (2021, 7) => solve::<y2021::day7::Solver>(year, day, &input),
            (2021, 8) => solve::<y2021::day8::Solver>(year, day, &input),
            (2021, 9) => solve::<y2021::day9::Solver>(year, day, &input),
            (2021, 10) => solve::<y2021::day10::Solver>(year, day, &input),
            (2021, 11) => solve::<y2021::day11::Solver>(year, day, &input),
            (2021, 12) => solve::<y2021::day12::Solver>(year, day, &input),
            (2021, 13) => solve::<y2021::day13::Solver>(year, day, &input),
            (2021, 14) => solve::<y2021::day14::Solver>(year, day, &input),
            (2021, 15) => solve::<y2021::day15::Solver>(year, day, &input),
            (2021, 16) => solve::<y2021::day16::Solver>(year, day, &input),
            (2021, 17) => solve::<y2021::day17::Solver>(year, day, &input),
            (2021, 18) => solve::<y2021::day18::Solver>(year, day, &input),
            (2021, 19) => solve::<y2021::day19::Solver>(year, day, &input),
            (2021, 20) => solve::<y2021::day20::Solver>(year, day, &input),
            (2021, 21) => solve::<y2021::day21::Solver>(year, day, &input),
            (2021, 22) => solve::<y2021::day22::Solver>(year, day, &input),
            (2021, 23) => solve::<y2021::day23::Solver>(year, day, &input),
            (2021, 24) => solve::<y2021::day24::Solver>(year, day, &input),
            (2021, 25) => solve::<y2021::day25::Solver>(year, day, &input),
            (_, _) => {}
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
