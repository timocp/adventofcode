use super::intcode::Vm;
use itertools::Itertools;

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
        maximise_amps(&self.vm).to_string()
    }

    fn part2(&self) -> String {
        "unimplemented".to_string()
    }
}

fn maximise_amps(vm: &Vm) -> i32 {
    (0..5)
        .permutations(5)
        .map(|phase| {
            let a = amp(vm, phase[0], 0);
            let b = amp(vm, phase[1], a);
            let c = amp(vm, phase[2], b);
            let d = amp(vm, phase[3], c);
            amp(vm, phase[4], d)
        })
        .max()
        .unwrap()
}

fn amp(vm: &Vm, phase_setting: i32, input_signal: i32) -> i32 {
    *vm.runio(&[phase_setting, input_signal])
        .first()
        .unwrap()
}

#[test]
fn test_maximise_amps() {
    let vm = Vm::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(43210, maximise_amps(&vm));

    let vm = Vm::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(54321, maximise_amps(&vm));

    let vm = Vm::from("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(65210, maximise_amps(&vm));
}
