use crate::Part;

pub fn run(input: &str, part: Part) -> String {
    let program = parse_input(input);
    let mut vm = VM::new();
    if part == Part::Two {
        vm.write_register(Register::A, 1);
    }
    vm.execute(&program);
    format!("{}", vm.read_register(Register::B))
}

#[derive(Debug)]
struct VM {
    registers: [u64; 2],
    pc: usize,
}

impl VM {
    fn new() -> Self {
        VM {
            registers: [0; 2],
            pc: 0,
        }
    }

    fn read_register(&self, register: Register) -> u64 {
        self.registers[register.index()]
    }

    fn write_register(&mut self, register: Register, value: u64) {
        self.registers[register.index()] = value;
    }

    fn execute(&mut self, program: &[Instruction]) {
        while self.pc < program.len() {
            match &program[self.pc] {
                Instruction::Half(r) => {
                    self.registers[r.index()] /= 2;
                    self.pc += 1;
                }
                Instruction::Triple(r) => {
                    self.registers[r.index()] *= 3;
                    self.pc += 1;
                }
                Instruction::Increment(r) => {
                    self.registers[r.index()] += 1;
                    self.pc += 1;
                }
                Instruction::Jump(offset) => {
                    self.pc = (self.pc as i64 + offset) as usize;
                }
                Instruction::JumpIfEven(r, offset) => {
                    if self.registers[r.index()] % 2 == 0 {
                        self.pc = (self.pc as i64 + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Instruction::JumpIfOne(r, offset) => {
                    if self.registers[r.index()] == 1 {
                        self.pc = (self.pc as i64 + offset) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

impl Register {
    fn index(&self) -> usize {
        match self {
            Register::A => 0,
            Register::B => 1,
        }
    }
}

impl From<&str> for Register {
    fn from(s: &str) -> Self {
        match s {
            "a" => Register::A,
            "b" => Register::B,
            _ => panic!("Unknown register: {}", s),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Half(Register),
    Triple(Register),
    Increment(Register),
    Jump(i64),
    JumpIfEven(Register, i64),
    JumpIfOne(Register, i64),
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let instruction = parts.next().unwrap();
            let arg1 = parts.next().unwrap();
            match instruction {
                "hlf" => Instruction::Half(arg1.into()),
                "tpl" => Instruction::Triple(arg1.into()),
                "inc" => Instruction::Increment(arg1.into()),
                "jmp" => Instruction::Jump(arg1.parse().unwrap()),
                "jie" => Instruction::JumpIfEven(
                    arg1[0..1].into(),
                    parts.next().unwrap().parse().unwrap(),
                ),
                "jio" => Instruction::JumpIfOne(
                    arg1[0..1].into(),
                    parts.next().unwrap().parse().unwrap(),
                ),
                _ => panic!("Unknown instruction: {}", instruction),
            }
        })
        .collect()
}

#[test]
fn test() {
    let test_input = "inc a\njio a, +2\ntpl a\ninc a\n";
    let mut vm = VM::new();
    assert_eq!(0, vm.read_register(Register::A));
    vm.execute(&parse_input(test_input));
    assert_eq!(2, vm.read_register(Register::A));
}
