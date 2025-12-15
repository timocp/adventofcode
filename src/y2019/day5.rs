use super::intcode::Vm;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(vm: &Vm) -> i64 {
    *vm.clone().run(&[1]).last().unwrap()
}

pub fn part2(vm: &Vm) -> i64 {
    *vm.clone().run(&[5]).last().unwrap()
}
