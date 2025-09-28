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
        let mut vm = self.vm.clone();
        vm.push_input(1);
        vm.run();
        vm.read_output().last().unwrap().to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}
