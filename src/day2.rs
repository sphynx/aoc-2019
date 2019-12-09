use std::fs;

enum OpCode {
    Add,
    Mul,
    Stop,
}

impl From<u64> for OpCode {
    fn from(data: u64) -> Self {
        match data {
            1 => OpCode::Add,
            2 => OpCode::Mul,
            99 => OpCode::Stop,
            x => panic!("Unknown op code: {}", x),
        }
    }
}

pub fn solve_part1() {
    let program = fs::read_to_string("input/day2.txt").unwrap();
    let mut codes: Vec<u64> = program
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    codes[1] = 12;
    codes[2] = 2;
    run(&mut codes);
    println!("day 2, part 1: {}", codes[0]);
}

pub fn solve_part2() {
    let program = fs::read_to_string("input/day2.txt").unwrap();
    let orig_codes: Vec<u64> = program
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    const TARGET: u64 = 19690720;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut codes = orig_codes.clone();
            codes[1] = noun;
            codes[2] = verb;
            run(&mut codes);
            let output = codes[0];
            if output == TARGET {
                let result = noun * 100 + verb;
                println!("day 2, part 2: {}", result);
                return;
            }
        }
    }
}


fn run(data: &mut Vec<u64>) {
    let mut ip = 0;
    loop {
        match OpCode::from(data[ip]) {
            OpCode::Add => {
                let read_addr_0 = data[ip + 1] as usize;
                let read_addr_1 = data[ip + 2] as usize;
                let write_addr = data[ip + 3] as usize;
                data[write_addr] = data[read_addr_0] + data[read_addr_1];
            }
            OpCode::Mul => {
                let read_addr_0 = data[ip + 1] as usize;
                let read_addr_1 = data[ip + 2] as usize;
                let write_addr = data[ip + 3] as usize;
                data[write_addr] = data[read_addr_0] * data[read_addr_1];
            }
            OpCode::Stop => {
                break;
            }
        }
        ip += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut prog = vec![1, 0, 0, 0, 99];
        run(&mut prog);
        assert_eq!(prog, vec![2, 0, 0, 0, 99]);

        let mut prog = vec![2, 3, 0, 3, 99];
        run(&mut prog);
        assert_eq!(prog, vec![2, 3, 0, 6, 99]);

        let mut prog = vec![2, 4, 4, 5, 99, 0];
        run(&mut prog);
        assert_eq!(prog, vec![2, 4, 4, 5, 99, 9801]);

        let mut prog = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        run(&mut prog);
        assert_eq!(prog, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

        let mut prog = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
        run(&mut prog);
        assert_eq!(prog, vec![3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]);
    }
}
