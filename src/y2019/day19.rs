use super::intcode::Vm;

pub struct Solver {
    vm: Vm,
}

impl crate::Puzzle for Solver {
    fn new(input: &str) -> Self {
        Self {
            vm: Vm::from(input),
        }
    }

    fn part1(&self) -> String {
        let mut count = 0;
        for x in 0..50 {
            for y in 0..50 {
                if check(&self.vm, x, y) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part2(&self) -> String {
        find_fit(&self.vm).to_string()
    }
}

// We can check approx 100k times per second when compiled for release
fn check(vm: &Vm, x: i64, y: i64) -> bool {
    vm.clone().run(&[x, y])[0] == 1
}

fn find_fit(vm: &Vm) -> i64 {
    // start 100 rows down to avoid the stuttery start
    let mut p = (0, 100);
    loop {
        if check(vm, p.0, p.1) {
            break;
        }
        p.0 += 1
    }

    // search row by row until we find
    loop {
        // p is the leftmost part of a beam on a row.
        // move to the leftmost part of the next row
        p.1 += 1;
        while !check(vm, p.0, p.1) {
            p.0 += 1;
        }
        if check_answer(vm, p) {
            return p.0 * 10000 + (p.1 - 99);
        }
    }
}

// check the answer (from is a bottom-left corner)
fn check_answer(vm: &Vm, from: (i64, i64)) -> bool {
    let tr = (from.0 + 99, from.1 - 99);
    check(vm, tr.0, tr.1) && !check(vm, tr.0 + 1, tr.1) && !check(vm, tr.0, tr.1 - 1)
}
