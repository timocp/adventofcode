use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Vm {
    mem: Vec<i32>,        // memory
    ip: usize,            // instruction pointer
    input: VecDeque<i32>, // input queue
    output: Vec<i32>,     // output
}

impl Vm {
    fn exec(&mut self) {
        match self.opcode() {
            1 => self.add(),
            2 => self.mul(),
            3 => self.inp(),
            4 => self.out(),
            99 => {}
            _ => panic!("invalid instruction: {} at {}", self.mem[self.ip], self.ip),
        }
    }

    pub fn run(&mut self) {
        while !self.is_halted() {
            self.exec();
        }
    }

    pub fn direct_write(&mut self, pos: usize, value: i32) {
        self.mem[pos] = value;
    }

    pub fn direct_read(&self, pos: usize) -> i32 {
        self.mem[pos]
    }

    pub fn push_input(&mut self, input: i32) {
        self.input.push_back(input);
    }

    pub fn read_output(&self) -> Vec<i32> {
        self.output.clone()
    }

    fn is_halted(&self) -> bool {
        self.mem[self.ip] == 99
    }

    fn current(&self) -> i32 {
        self.mem[self.ip]
    }

    // ones and tens digit determines opcode
    fn opcode(&self) -> i32 {
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
            _ => panic!(),
        }
    }

    // returns the real address that a param (reletive to IP) refers to given its mode
    fn address(&self, param: u8) -> usize {
        match self.mode(param) {
            Mode::Position => self.mem[self.ip + param as usize].try_into().unwrap(),
            Mode::Immediate => self.ip + param as usize,
        }
    }

    fn read(&self, param: u8) -> i32 {
        self.mem[self.address(param)]
    }

    fn write(&mut self, param: u8, value: i32) {
        let addr = self.address(param);
        self.mem[addr] = value;
    }

    fn add(&mut self) {
        //println!("ADD {:?}", &self.mem[self.ip..(self.ip + 4)]);
        self.write(3, self.read(1) + self.read(2));
        self.ip += 4;
    }

    fn mul(&mut self) {
        //println!("MUL {:?}", &self.mem[self.ip..(self.ip + 4)]);
        self.write(3, self.read(1) * self.read(2));
        self.ip += 4;
    }

    fn inp(&mut self) {
        //println!("INP {:?}", &self.mem[self.ip..(self.ip + 2)]);
        let value = self.input.pop_front().unwrap();
        self.write(1, value);
        self.ip += 2;
    }

    fn out(&mut self) {
        //println!("OUT {:?}", &self.mem[self.ip..(self.ip + 2)]);
        self.output.push(self.read(1));
        self.ip += 2;
    }
}

// parameter modes
enum Mode {
    Position,
    Immediate,
}

impl From<&str> for Vm {
    fn from(s: &str) -> Self {
        Vm {
            mem: s
                .lines()
                .next()
                .unwrap()
                .split(',')
                .map(|i| i.parse().unwrap())
                .collect(),
            ip: 0,
            input: [].into(),
            output: [].into(),
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
    vm.exec();
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
    };
    assert_eq!(1, vm.mem[0]);
    vm.exec();
    assert_eq!(3500, vm.mem[0]);
    assert_eq!(8, vm.ip);
}

#[test]
fn test_is_halted() {
    let vm = Vm::from("99");
    assert!(vm.is_halted());
}

#[test]
fn test_run() {
    for (program, expected_mem) in [
        ("1,0,0,0,99", vec![2, 0, 0, 0, 99]),         // (1 + 1 = 2)
        ("2,3,0,3,99", vec![2, 3, 0, 6, 99]),         // (3 * 2 = 6).
        ("2,4,4,5,99,0", vec![2, 4, 4, 5, 99, 9801]), // (99 * 99 = 9801).
        ("1,1,1,4,99,5,6,0,99", vec![30, 1, 1, 4, 2, 5, 6, 0, 99]),
    ] {
        let mut vm = Vm::from(program);
        vm.run();
        assert_eq!(expected_mem, vm.mem);
    }
}

#[test]
fn test_immediate_position() {
    let mut vm = Vm::from("1002,4,3,4,33");
    vm.exec();
    assert_eq!(99, vm.mem[4]);
}

#[test]
fn test_input_output() {
    let mut vm = Vm::from("3,0,4,0,99");
    vm.push_input(1234);
    vm.run();
    assert_eq!(vec![1234], vm.output);
    assert_eq!(1234, vm.mem[0]);
}
