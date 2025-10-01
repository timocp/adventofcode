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
        self.vm.clone().run(&[1]).last().unwrap().to_string()
    }

    fn part2(&self) -> String {
        self.vm.clone().run(&[5]).last().unwrap().to_string()
    }
}
