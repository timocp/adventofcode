use super::intcode::Vm;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(boost_vm: &Vm) -> i64 {
    let output = boost_vm.clone().run(&[1]);
    if output.len() != 1 {
        panic!("BOOST error: {:?}", output);
    }
    output[0]
}

pub fn part2(boost_vm: &Vm) -> i64 {
    boost_vm.clone().run(&[2])[0]
}
