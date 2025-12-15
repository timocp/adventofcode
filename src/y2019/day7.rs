use super::intcode::Vm;
use itertools::Itertools;

pub fn parse_input(input: &str) -> Vm {
    Vm::from(input)
}

pub fn part1(vm: &Vm) -> i64 {
    maximise_amps(vm)
}

pub fn part2(vm: &Vm) -> i64 {
    maximise_feedback(vm)
}

fn maximise_amps(vm: &Vm) -> i64 {
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

fn run(vm: &mut Vm, inputs: &[i64]) -> i64 {
    *vm.run(inputs).first().unwrap()
}

fn amp(vm: &Vm, phase_setting: i64, input_signal: i64) -> i64 {
    run(&mut vm.clone(), &[phase_setting, input_signal])
}

fn maximise_feedback(vm: &Vm) -> i64 {
    (5..10)
        .permutations(5)
        .map(|phase| {
            let mut amps = vec![vm.clone(), vm.clone(), vm.clone(), vm.clone(), vm.clone()];
            let mut signal = run(&mut amps[0], &[phase[0], 0]);
            signal = run(&mut amps[1], &[phase[1], signal]);
            signal = run(&mut amps[2], &[phase[2], signal]);
            signal = run(&mut amps[3], &[phase[3], signal]);
            signal = run(&mut amps[4], &[phase[4], signal]);
            while !amps[4].is_halted() {
                signal = run(&mut amps[0], &[signal]);
                signal = run(&mut amps[1], &[signal]);
                signal = run(&mut amps[2], &[signal]);
                signal = run(&mut amps[3], &[signal]);
                signal = run(&mut amps[4], &[signal]);
            }
            signal
        })
        .max()
        .unwrap()
}

#[test]
fn test_maximise_amps() {
    let vm = Vm::from("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(43210, maximise_amps(&vm));

    let vm = Vm::from("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(54321, maximise_amps(&vm));

    let vm = Vm::from(
        "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
    );
    assert_eq!(65210, maximise_amps(&vm));
}

#[test]
fn test_maximise_feedback() {
    let vm = Vm::from(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    );
    assert_eq!(139629729, maximise_feedback(&vm));

    let vm = Vm::from(
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    );
    assert_eq!(18216, maximise_feedback(&vm));
}
