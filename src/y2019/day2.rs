use super::intcode::Vm;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(vm: &Vm) -> i64 {
    get_result(vm, 12, 2)
}

pub fn part2(vm: &Vm) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            if get_result(vm, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found");
}

// Return the final value at address 0 if the program is run to completion
fn get_result(vm: &Vm, noun: i64, verb: i64) -> i64 {
    let mut vm = vm.clone();
    vm.direct_write(1, noun);
    vm.direct_write(2, verb);
    vm.exec();
    vm.direct_read(0)
}

#[test]
fn test() {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let mut vm = Vm::from(test_input);
    vm.exec();
    assert_eq!(3500, vm.direct_read(0));
    assert_eq!(70, vm.direct_read(3));
}
