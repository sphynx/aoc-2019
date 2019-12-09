use std::convert::TryFrom;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::path::Path;

pub struct Computer {
    memory: Vec<i64>,
    ip: usize,
    input: Box<dyn BufRead>,
    output: Vec<String>,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            memory: vec![],
            ip: 0,
            input: Box::new(BufReader::new(io::stdin())),
            output: vec![],
        }
    }

    pub fn set_input(&mut self, input: Box<dyn BufRead>) {
        self.input = input;
    }

    pub fn set_str_input(&mut self, input_str: &'static str) {
        self.set_input(Box::new(BufReader::new(input_str.as_bytes())));
    }

    pub fn set_input_lines(&mut self, lines: &[&str]) {
        let vec = lines.join("\n").into_bytes();
        self.set_input(Box::new(BufReader::new(Cursor::new(vec))));
    }

    pub fn load_from_file<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        let program = fs::read_to_string(path).expect("can't load program");
        let data: Vec<i64> = program
            .trim()
            .split(",")
            .map(|x| {
                x.parse()
                    .expect(format!("can't parse int code in a program: {}", x).as_str())
            })
            .collect();
        self.load_memory(data);
    }

    pub fn load_memory<T>(&mut self, mem: T)
    where
        T: AsRef<[i64]>,
    {
        mem.as_ref().clone_into(&mut self.memory);
        self.ip = 0;
    }

    pub fn run_with_memory<T>(&mut self, mem: T) -> &[i64]
    where
        T: AsRef<[i64]>,
    {
        mem.as_ref().clone_into(&mut self.memory);
        self.ip = 0;
        self.run();
        self.dump_memory()
    }

    pub fn dump_memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn run(&mut self) -> &Vec<String> {
        loop {
            match Instruction::from(self.memory[self.ip]) {
                Instruction::Add(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    let write_addr = self.memory[self.ip + 3] as usize;
                    self.memory[write_addr] = param_1 + param_2;
                    self.ip += 4;
                }

                Instruction::Mul(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    let write_addr = self.memory[self.ip + 3] as usize;
                    self.memory[write_addr] = param_1 * param_2;
                    self.ip += 4;
                }

                Instruction::Input => {
                    let write_addr = self.memory[self.ip + 1] as usize;
                    let mut buf = String::new();
                    self.input.read_line(&mut buf).expect("cannot read line");
                    self.memory[write_addr] = buf.trim().parse().expect("cannot parse input");
                    self.ip += 2;
                }

                Instruction::Output(p1_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    self.output.push(format!("{}", param_1));
                    self.ip += 2;
                }

                Instruction::JumpIfTrue(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    if param_1 != 0 {
                        self.ip = usize::try_from(param_2).expect("ip is not usize");
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::JumpIfFalse(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    if param_1 == 0 {
                        self.ip = usize::try_from(param_2).expect("ip is not usize");
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::LessThan(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    let write_addr = self.memory[self.ip + 3] as usize;
                    self.memory[write_addr] = (param_1 < param_2) as i64;
                    self.ip += 4;
                }

                Instruction::Equals(p1_mode, p2_mode) => {
                    let param_1 = self.resolve_param(p1_mode, self.memory[self.ip + 1]);
                    let param_2 = self.resolve_param(p2_mode, self.memory[self.ip + 2]);
                    let write_addr = self.memory[self.ip + 3] as usize;
                    self.memory[write_addr] = (param_1 == param_2) as i64;
                    self.ip += 4;
                }

                Instruction::Stop => {
                    return &self.output;
                }
            }
        }
    }

    fn resolve_param(&self, mode: ParameterMode, param: i64) -> i64 {
        match mode {
            ParameterMode::Immediate => param,
            ParameterMode::Position => self.memory[param as usize],
        }
    }
}

#[derive(PartialEq, Eq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(data: i64) -> Self {
        match data {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            x => panic!("Unknown parameter mode: {}", x),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Instruction {
    Add(ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Stop,
}

impl From<i64> for Instruction {
    fn from(data: i64) -> Self {
        assert!(data > 0);
        let op_code = data % 100;
        let param1_mode = ParameterMode::from((data / 100) % 10);
        let param2_mode = ParameterMode::from((data / 1_000) % 10);
        let param3_mode = ParameterMode::from((data / 10_000) % 10);
        match op_code {
            1 => {
                assert!(param3_mode == ParameterMode::Position);
                Instruction::Add(param1_mode, param2_mode)
            }
            2 => {
                assert!(param3_mode == ParameterMode::Position);
                Instruction::Mul(param1_mode, param2_mode)
            }
            3 => {
                assert!(param1_mode == ParameterMode::Position);
                Instruction::Input
            }
            4 => Instruction::Output(param1_mode),
            5 => Instruction::JumpIfTrue(param1_mode, param2_mode),
            6 => Instruction::JumpIfFalse(param1_mode, param2_mode),
            7 => {
                assert!(param3_mode == ParameterMode::Position);
                Instruction::LessThan(param1_mode, param2_mode)
            }
            8 => {
                assert!(param3_mode == ParameterMode::Position);
                Instruction::Equals(param1_mode, param2_mode)
            }
            99 => Instruction::Stop,
            _ => panic!("Unknown op code: {}", op_code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_computer(comp: &mut Computer, initial_mem: Vec<i64>, final_mem: Vec<i64>) {
        assert_eq!(comp.run_with_memory(initial_mem), &final_mem[..]);
    }

    #[test]
    fn test_running() {
        let mut comp = Computer::new();

        test_computer(&mut comp, vec![1, 0, 0, 0, 99], vec![2, 0, 0, 0, 99]);
        test_computer(&mut comp, vec![2, 3, 0, 3, 99], vec![2, 3, 0, 6, 99]);
        test_computer(
            &mut comp,
            vec![2, 4, 4, 5, 99, 0],
            vec![2, 4, 4, 5, 99, 9801],
        );
        test_computer(
            &mut comp,
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
        test_computer(
            &mut comp,
            vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
            vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
        );
        test_computer(&mut comp, vec![1002, 4, 3, 4, 33], vec![1002, 4, 3, 4, 99]);
    }
}
