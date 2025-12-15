use super::intcode::Vm;
use itertools::Itertools;
use std::collections::HashSet;
use std::io;

pub struct Input {
    vm: Vm,
    interactive: bool,
}

pub fn parse_input(input: &str) -> Input {
    Input {
        vm: Vm::from(input),
        interactive: false, // true to play manually, false to solve automatically
    }
}

pub fn part1(input: &Input) -> String {
    if input.interactive {
        interactive_mode(input.vm.clone());
    }
    let mut vm = input.vm.clone();
    gather_stuff(&mut vm);
    find_code(&mut vm)
}

pub fn part2(_input: &Input) -> &str {
    "n/a"
}

fn interactive_mode(mut vm: Vm) {
    // gather_stuff(&mut vm);
    let mut input = "".to_owned();
    loop {
        println!("exec");
        let output = execute(&mut vm, &input);
        input.clear();
        print!("{}", output);
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(error) => {
                println!("ERROR: {}", error);
                return;
            }
        }
    }
}

fn execute(vm: &mut Vm, input: &str) -> String {
    let input: Vec<i64> = input.chars().map(|c| c as i64).collect();
    let output = vm.run(&input);
    output
        .iter()
        .filter_map(|&i| char::from_u32(i as u32))
        .collect()
}

// hardcoded version to get all the objects and move them to the security checkpoint
// exploring the maze programmatically to find this would be interesting but I can't be bothered
fn gather_stuff(vm: &mut Vm) -> String {
    execute(
        vm,
        "north\n\
        take tambourine\n\
        east\n\
        take astrolabe\n\
        south\n\
        take shell\n\
        north\n\
        east\n\
        north\n\
        take klein bottle\n\
        north\n\
        take easter egg\n\
        south\n\
        south\n\
        west\n\
        west\n\
        south\n\
        south\n\
        south\n\
        take hypercube\n\
        north\n\
        north\n\
        west\n\
        take dark matter\n\
        west\n\
        north\n\
        west\n\
        take coin\n\
        south\n\
        drop astrolabe\n\
        drop coin\n\
        drop dark matter\n\
        drop easter egg\n\
        drop hypercube\n\
        drop klein bottle\n\
        drop shell\n\
        drop tambourine\n\
        ",
    )
}

const ITEMS: [&str; 8] = [
    "astrolabe",
    "coin",
    "dark matter",
    "easter egg",
    "hypercube",
    "klein bottle",
    "shell",
    "tambourine",
];

// brute force the combination of items that has the correct weight
fn find_code(vm: &mut Vm) -> String {
    let mut holding: HashSet<usize> = HashSet::new(); // things we're currently holding
    let mut input = "".to_owned();
    let mut drop: Vec<usize> = vec![];
    let mut take: Vec<usize> = vec![];
    for count in 1..=8 {
        for set in (0..ITEMS.len()).permutations(count) {
            // work out what we need to take/drop, then move south to test the weight
            drop.clear();
            take.clear();
            input.clear();
            for i in 0..ITEMS.len() {
                if set.contains(&i) && !holding.contains(&i) {
                    take.push(i);
                } else if holding.contains(&i) && !set.contains(&i) {
                    drop.push(i);
                }
            }
            for i in drop.iter() {
                input += &format!("drop {}\n", ITEMS[*i]);
                holding.remove(i);
            }
            for i in take.iter() {
                input += &format!("take {}\n", ITEMS[*i]);
                holding.insert(*i);
            }
            input += "south\n";

            let output = execute(vm, &input);
            if output.contains("Droids on this ship are heavier")
                || output.contains("Droids on this ship are lighter")
            {
            } else if output.contains("You may proceed.") {
                //println!(
                //    "items needed: {:?}",
                //    holding.iter().map(|i| ITEMS[*i]).collect::<Vec<_>>()
                //);
                let mut words_iter = output.split_whitespace();
                // consume up to: "you should be able to get in by typing ..."
                words_iter.position(|s| s == "typing");
                let code = words_iter.next();
                return code.unwrap().to_string();
            } else {
                panic!("Unexpected output: {:?}", output);
            }
        }
    }
    unreachable!()
}

// stuff to take:
// hypercube
// coin
// klein bottle
// shell
// easter egg
// astrolabe
// tambourine
// dark matter
//
// don't take:
// photons
// giant electromagnet
// molten lava
// escape pod
// infinite loop
