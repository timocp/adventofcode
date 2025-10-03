use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Vm {
    mem: Vec<i64>,        // memory
    ip: usize,            // instruction pointer
    input: VecDeque<i64>, // input queue
    output: Vec<i64>,     // output
    relbase: i64,         // offset for relative mode parameters
}

impl Vm {
    fn exec_op(&mut self) {
        // println!("{}", self.dump_state());
        match self.opcode() {
            1 => self.add(),
            2 => self.mul(),
            3 => self.inp(),
            4 => self.out(),
            5 => self.jit(),
            6 => self.jif(),
            7 => self.lt(),
            8 => self.eq(),
            9 => self.arb(),
            99 => {}
            _ => panic!("invalid instruction: {} at {}", self.mem[self.ip], self.ip),
        }
    }

    // run VM until it halts OR needs input but has none available
    pub fn exec(&mut self) {
        while !self.is_halted() && !self.is_paused() {
            self.exec_op();
        }
    }

    // run VM with specific input and return its output
    pub fn run(&mut self, input: &[i64]) -> Vec<i64> {
        self.set_input(input);
        self.output = vec![];
        self.exec();
        self.output.clone()
    }

    pub fn direct_write(&mut self, pos: usize, value: i64) {
        self.mem[pos] = value;
    }

    pub fn direct_read(&self, pos: usize) -> i64 {
        self.mem[pos]
    }

    pub fn set_input(&mut self, input: &[i64]) {
        self.input = input.to_vec().into();
    }

    pub fn is_halted(&self) -> bool {
        self.mem[self.ip] == 99
    }

    // true if next op is INP but there is no input ready
    fn is_paused(&self) -> bool {
        self.opcode() == 3 && self.input.is_empty()
    }

    fn current(&self) -> i64 {
        self.mem[self.ip]
    }

    // ones and tens digit determines opcode
    fn opcode(&self) -> i64 {
        self.current() % 100
    }

    // mode is determined by:
    //   hundreds (param 1)
    //   thousands (param 2)
    //   ten thousands (param 3)
    fn mode(&self, param: u8) -> Mode {
        match match param {
            1 => (self.current() / 100) % 10,
            2 => (self.current() / 1000) % 10,
            3 => (self.current() / 10000) % 10,
            _ => panic!(),
        } {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!(),
        }
    }

    // returns the real address that a param (relative to IP) refers to given its mode
    fn address(&self, param: u8) -> usize {
        match self.mode(param) {
            Mode::Position => self.mem[self.ip + param as usize] as usize,
            Mode::Immediate => self.ip + param as usize,
            Mode::Relative => (self.relbase + self.mem[self.ip + param as usize]) as usize,
        }
    }

    fn read(&self, param: u8) -> i64 {
        let addr = self.address(param);
        if addr < self.mem.len() {
            self.mem[self.address(param)]
        } else {
            0
        }
    }

    fn write(&mut self, param: u8, value: i64) {
        let addr = self.address(param);
        if addr >= self.mem.len() {
            self.mem.resize(addr + 1, 0);
        }
        self.mem[addr] = value;
    }

    fn add(&mut self) {
        self.write(3, self.read(1) + self.read(2));
        self.ip += 4;
    }

    fn mul(&mut self) {
        self.write(3, self.read(1) * self.read(2));
        self.ip += 4;
    }

    fn inp(&mut self) {
        let value = self.input.pop_front().unwrap();
        self.write(1, value);
        self.ip += 2;
    }

    fn out(&mut self) {
        self.output.push(self.read(1));
        self.ip += 2;
    }

    fn jit(&mut self) {
        if self.read(1) != 0 {
            self.ip = self.read(2) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jif(&mut self) {
        if self.read(1) == 0 {
            self.ip = self.read(2) as usize;
        } else {
            self.ip += 3;
        }
    }

    fn lt(&mut self) {
        self.write(3, if self.read(1) < self.read(2) { 1 } else { 0 });
        self.ip += 4;
    }

    fn eq(&mut self) {
        self.write(3, if self.read(1) == self.read(2) { 1 } else { 0 });
        self.ip += 4;
    }

    // Adjust Relative Base
    fn arb(&mut self) {
        self.relbase += self.read(1);
        self.ip += 2;
    }

    //fn debug_address(&self, param: u8) -> String {
    //    format!(
    //        "{}({})",
    //        match self.mode(param) {
    //            Mode::Position => 'P',
    //            Mode::Immediate => 'I',
    //            Mode::Relative => 'R',
    //        },
    //        self.address(param)
    //    )
    //}

    #[allow(dead_code)]
    fn dump_state(&self) -> String {
        let mut s = String::new();
        s.push('[');
        for (i, mem) in self.mem.iter().enumerate() {
            if i == self.ip {
                s.push('*');
            } else {
                s.push(' ');
            }
            s.push_str(&mem.to_string());
        }

        s.push(']');
        s.push_str(&format!(" IP={}", self.ip));
        s.push_str(&format!(" RB={}", self.relbase));
        s.push_str(&format!(" I={:?}", self.input));
        s.push_str(&format!(" O={:?}", self.output));
        s
    }
}

// parameter modes
#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
    Relative,
}

impl From<&str> for Vm {
    fn from(s: &str) -> Self {
        Vm {
            mem: s.trim().split(',').map(|i| i.parse().unwrap()).collect(),
            ip: 0,
            input: [].into(),
            output: [].into(),
            relbase: 0,
        }
    }
}

#[test]
fn test_from() {
    let test_input = "1,9,10,3,2,3,11,0,99,30,40,50";
    let vm = Vm::from(test_input);
    assert_eq!(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50], vm.mem);
    assert_eq!(0, vm.ip);
    assert!(vm.input.is_empty());
    assert!(vm.output.is_empty());
}

#[test]
fn test_add() {
    let mut vm = Vm::from("1,9,10,3,2,3,11,0,99,30,40,50");
    assert_eq!(3, vm.mem[3]);
    vm.exec_op();
    assert_eq!(70, vm.mem[3]);
    assert_eq!(4, vm.ip);
}

#[test]
fn test_mul() {
    let mut vm = Vm {
        mem: vec![1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        ip: 4,
        input: [].into(),
        output: [].into(),
        relbase: 0,
    };
    assert_eq!(1, vm.mem[0]);
    vm.exec_op();
    assert_eq!(3500, vm.mem[0]);
    assert_eq!(8, vm.ip);
}

#[test]
fn test_is_halted() {
    let vm = Vm::from("99");
    assert!(vm.is_halted());
}

#[test]
fn test_exec() {
    for (program, expected_mem) in [
        ("1,0,0,0,99", vec![2, 0, 0, 0, 99]),         // (1 + 1 = 2)
        ("2,3,0,3,99", vec![2, 3, 0, 6, 99]),         // (3 * 2 = 6).
        ("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801]), // (99 * 99 = 9801).
        ("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
    ] {
        let mut vm = Vm::from(program);
        vm.exec();
        assert_eq!(expected_mem, vm.mem);
    }
}

#[test]
fn test_immediate_position() {
    let mut vm = Vm::from("1002,4,3,4,33");
    vm.exec_op();
    assert_eq!(99, vm.mem[4]);
}

#[test]
fn test_input_output() {
    let mut vm = Vm::from("3,0,4,0,99");
    vm.input = [1234].into();
    vm.exec();
    assert_eq!(vec![1234], vm.output);
    assert_eq!(1234, vm.mem[0]);
}

#[test]
fn test_eq() {
    // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not)
    let program = "3,9,8,9,10,9,4,9,99,-1,8";
    for (input, expected_output) in [(7, vec![0]), (8, vec![1]), (9, vec![0])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }

    // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
    let program = "3,3,1108,-1,8,3,4,3,99";
    for (input, expected_output) in [(7, vec![0]), (8, vec![1]), (9, vec![0])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }
}

#[test]
fn test_lt() {
    // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let program = "3,9,7,9,10,9,4,9,99,-1,8";
    for (input, expected_output) in [(7, vec![1]), (8, vec![0]), (9, vec![0])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }

    // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
    let program = "3,3,1107,-1,8,3,4,3,99";
    for (input, expected_output) in [(7, vec![1]), (8, vec![0]), (9, vec![0])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }
}

#[test]
fn test_jump() {
    // output 0 if the input was zero or 1 if the input was non-zero (using position mode and JIF)
    let program = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9";
    for (input, expected_output) in [(-1, vec![1]), (0, vec![0]), (1, vec![1])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }

    // (using immediate mode and JIT)
    let program = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1";
    for (input, expected_output) in [(-1, vec![1]), (0, vec![0]), (1, vec![1])] {
        assert_eq!(expected_output, Vm::from(program).run(&[input]));
    }
}

#[test]
fn test_relative_mode() {
    assert_eq!(
        vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99],
        Vm::from("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99").run(&[])
    );
    assert_eq!(
        vec![1219070632396864],
        Vm::from("1102,34915192,34915192,7,4,7,99,0").run(&[])
    );
    assert_eq!(
        vec![1125899906842624],
        Vm::from("104,1125899906842624,99").run(&[])
    );
}
