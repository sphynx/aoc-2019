use std::convert::TryFrom;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Cursor;
use std::path::Path;

use log::info;

pub struct Computer {
    memory: Vec<i64>,
    ip: usize,
    rel_base: i64,
    pub status: Status,
    input: Box<dyn BufRead>,
    output: Vec<String>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Status {
    RequiresInput,
    ProducedOutput(i64),
    Halted,
}

impl Computer {
    pub fn new() -> Self {
        Computer {
            memory: vec![],
            ip: 0,
            rel_base: 0,
            input: Box::new(BufReader::new(io::stdin())),
            output: vec![],
            status: Status::Halted,
        }
    }

    fn set_input(&mut self, input: Box<dyn BufRead>) {
        self.input = input;
    }

    pub fn set_input_lines(&mut self, lines: &[&str]) {
        let vec = lines.join("\n").into_bytes();
        self.set_input(Box::new(BufReader::new(Cursor::new(vec))));
    }

    pub fn load_from_file<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.load_memory(Self::read_program(path));
    }

    pub fn read_program<P>(path: P) -> Vec<i64>
    where
        P: AsRef<Path>,
    {
        let program = fs::read_to_string(path).expect("can't load program");
        program
            .trim()
            .split(",")
            .map(|x| {
                x.parse()
                    .expect(format!("can't parse int code in a program: {}", x).as_str())
            })
            .collect()
    }

    pub fn load_memory<T>(&mut self, mem: T)
    where
        T: AsRef<[i64]>,
    {
        mem.as_ref().clone_into(&mut self.memory);
        self.ip = 0;
        self.rel_base = 0;
    }

    pub fn run_with_memory<T>(&mut self, mem: T) -> &[i64]
    where
        T: AsRef<[i64]>,
    {
        mem.as_ref().clone_into(&mut self.memory);
        self.ip = 0;
        self.rel_base = 0;
        self.run();
        self.dump_memory()
    }

    pub fn dump_memory(&self) -> &[i64] {
        &self.memory
    }

    pub fn last_output(&self) -> &str {
        &self.output.iter().last().unwrap()
    }

    pub fn send_input(&mut self, input: i64) {
        assert_eq!(self.status, Status::RequiresInput);
        if let Instruction::Input(mode) = Instruction::from(self.read_mem(self.ip)) {
            let write_addr = self.resolve_write_addr(mode, self.read_mem(self.ip + 1));
            self.write_mem(write_addr as usize, input);
            self.ip += 2;
            self.run_as_coroutine();
        } else {
            panic!("expected op_code = Input in send_input()");
        }
    }

    pub fn peek_output(&self) -> Option<i64> {
        match self.status {
            Status::ProducedOutput(out) => Some(out),
            _ => None,
        }
    }

    pub fn run(&mut self) {
        self.run_as_coroutine();
        loop {
            match self.status {
                Status::Halted => {
                    break;
                }
                Status::RequiresInput => {
                    let mut buf = String::new();
                    self.input.read_line(&mut buf).expect("cannot read line");
                    let input = buf.trim().parse().expect("cannot parse input");
                    self.send_input(input);
                }
                Status::ProducedOutput(out) => {
                    self.output.push(format!("{}", out));
                    self.run_as_coroutine();
                }
            }
        }
    }

    pub fn run_as_coroutine(&mut self) {
        loop {
            match Instruction::from(self.read_mem(self.ip)) {
                Instruction::Add(p1_mode, p2_mode, p3_mode) => {
                    info!("add");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    let param_3 = self.resolve_write_addr(p3_mode, self.read_mem(self.ip + 3));
                    self.write_mem(param_3 as usize, param_1 + param_2);
                    self.ip += 4;
                }

                Instruction::Mul(p1_mode, p2_mode, p3_mode) => {
                    info!("mul");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    let param_3 = self.resolve_write_addr(p3_mode, self.read_mem(self.ip + 3));
                    self.write_mem(param_3 as usize, param_1 * param_2);
                    self.ip += 4;
                }

                Instruction::Input(_) => {
                    info!("input");
                    self.status = Status::RequiresInput;
                    break;
                }

                Instruction::Output(p1_mode) => {
                    info!("output");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    self.ip += 2;
                    self.status = Status::ProducedOutput(param_1);
                    break;
                }

                Instruction::JumpIfTrue(p1_mode, p2_mode) => {
                    info!("jump if true");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    if param_1 != 0 {
                        self.ip = usize::try_from(param_2).expect("ip is not usize");
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::JumpIfFalse(p1_mode, p2_mode) => {
                    info!("jump if false");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    if param_1 == 0 {
                        self.ip = usize::try_from(param_2).expect("ip is not usize");
                    } else {
                        self.ip += 3;
                    }
                }

                Instruction::LessThan(p1_mode, p2_mode, p3_mode) => {
                    info!("less than");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    let param_3 = self.resolve_write_addr(p3_mode, self.read_mem(self.ip + 3));
                    self.write_mem(param_3 as usize, (param_1 < param_2) as i64);
                    self.ip += 4;
                }

                Instruction::Equals(p1_mode, p2_mode, p3_mode) => {
                    info!("equals");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    let param_2 = self.resolve_param(p2_mode, self.read_mem(self.ip + 2));
                    let param_3 = self.resolve_write_addr(p3_mode, self.read_mem(self.ip + 3));
                    self.write_mem(param_3 as usize, (param_1 == param_2) as i64);
                    self.ip += 4;
                }

                Instruction::AdjustRelativeBase(p1_mode) => {
                    info!("adjust relative");
                    let param_1 = self.resolve_param(p1_mode, self.read_mem(self.ip + 1));
                    self.rel_base += param_1;
                    self.ip += 2;
                }

                Instruction::Stop => {
                    info!("stop");
                    self.status = Status::Halted;
                    break;
                }
            }
        }
    }

    fn write_mem(&mut self, addr: usize, value: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        info!("writing to addr: {}, value: {}", addr, value);
        self.memory[addr] = value;
    }

    fn read_mem(&self, addr: usize) -> i64 {
        if addr < self.memory.len() {
            self.memory[addr]
        } else {
            0
        }
    }

    fn resolve_param(&self, mode: ParameterMode, param: i64) -> i64 {
        info!("resolving param {} with mode: {:?}", param, mode);
        match mode {
            ParameterMode::Immediate => param,
            ParameterMode::Position => self.read_mem(param as usize),
            ParameterMode::Relative => self.read_mem((param + self.rel_base) as usize),
        }
    }

    fn resolve_write_addr(&self, mode: ParameterMode, param: i64) -> usize {
        info!("resolving write addr {} with mode: {:?}", param, mode);
        match mode {
            ParameterMode::Immediate => panic!("write address can't use Immediate mode"),
            ParameterMode::Position => param as usize,
            ParameterMode::Relative => (param + self.rel_base) as usize,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(data: i64) -> Self {
        match data {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            x => panic!("Unknown parameter mode: {}", x),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Instruction {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mul(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    AdjustRelativeBase(ParameterMode),
    Stop,
}

impl From<i64> for Instruction {
    fn from(data: i64) -> Self {
        assert!(data > 0);
        let op_code = data % 100;
        let param1_mode = ParameterMode::from((data / 100) % 10);
        let param2_mode = ParameterMode::from((data / 1_000) % 10);
        let param3_mode = ParameterMode::from((data / 10_000) % 10);

        info!(
            "param modes: p1: {:?}, p2: {:?}, p3: {:?}",
            param1_mode, param2_mode, param3_mode
        );
        match op_code {
            1 => {
                assert!(param3_mode != ParameterMode::Immediate);
                Instruction::Add(param1_mode, param2_mode, param3_mode)
            }
            2 => {
                assert!(param3_mode != ParameterMode::Immediate);
                Instruction::Mul(param1_mode, param2_mode, param3_mode)
            }
            3 => {
                assert!(param1_mode != ParameterMode::Immediate);
                Instruction::Input(param1_mode)
            }
            4 => Instruction::Output(param1_mode),
            5 => Instruction::JumpIfTrue(param1_mode, param2_mode),
            6 => Instruction::JumpIfFalse(param1_mode, param2_mode),
            7 => {
                assert!(param3_mode != ParameterMode::Immediate);
                Instruction::LessThan(param1_mode, param2_mode, param3_mode)
            }
            8 => {
                assert!(param3_mode != ParameterMode::Immediate);
                Instruction::Equals(param1_mode, param2_mode, param3_mode)
            }
            9 => Instruction::AdjustRelativeBase(param1_mode),
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

    #[allow(dead_code)]
    fn test_computer_output(comp: &mut Computer, initial_mem: Vec<i64>, out: Vec<i64>) {
        comp.run_with_memory(initial_mem);
        let out_64: Vec<String> = out.iter().map(|c| c.to_string()).collect();
        assert_eq!(comp.output, out_64);
    }

    #[test]
    fn test_basic_features() {
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

    #[test]
    fn test_day9_features() {
        let mut comp = Computer::new();

        comp.run_with_memory(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        assert_eq!(comp.last_output().len(), 16);

        comp.run_with_memory(vec![104, 1125899906842624, 99]);
        assert_eq!(comp.last_output(), "1125899906842624".to_owned());
    }

    #[test]
    fn test_long_features() {
        let mut comp = Computer::new();

        test_computer_output(
            &mut comp,
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
        );
    }
}
