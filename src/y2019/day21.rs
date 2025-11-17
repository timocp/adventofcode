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
        self.vm.clone().run(&walk()).last().unwrap().to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

// @
// ##..#    D has to be true or we'll die
//          A,B or C should be a gap or there's no point jumping
//
// Jump if: D && !(A && B && C)
fn walk() -> Vec<i64> {
    "\
OR A T
AND B T
AND C T
NOT T T
AND D T
OR T J
WALK
"
    .chars()
    .map(|c| c as i64)
    .collect()
}
