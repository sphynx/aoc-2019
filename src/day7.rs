use crate::intcode::*;
use permutohedron::Heap;

pub fn solve_part1() -> i64 {
    let memory = Computer::read_program("input/day7.txt");
    enumerate_phases(&memory)
}

pub fn solve_part2() -> i64 {
    let memory = Computer::read_program("input/day7.txt");
    enumerate_phases_2(&memory)
}

fn enumerate_phases(mem: &[i64]) -> i64 {
    let mut phases = [0, 1, 2, 3, 4];
    Heap::new(&mut phases)
        .map(|phases| run_pipeline(&phases, &mem))
        .max()
        .unwrap()
}

fn run_pipeline(permutation: &[i64], mem: &[i64]) -> i64 {
    let mut out: i64 = 0;
    for ix in 0..5 {
        let mut comp = Computer::new();
        comp.load_memory(mem);
        comp.run_as_coroutine();
        comp.send_input(permutation[ix]);
        comp.send_input(out);
        out = comp.peek_output().unwrap();
    }
    out
}

fn enumerate_phases_2(mem: &[i64]) -> i64 {
    let mut phases = [5, 6, 7, 8, 9];
    Heap::new(&mut phases)
        .map(|phases| run_feedback_loop(&phases, &mem))
        .max()
        .unwrap()
}

fn run_feedback_loop(permutation: &[i64], mem: &[i64]) -> i64 {
    let mut farm = [
        Computer::new(),
        Computer::new(),
        Computer::new(),
        Computer::new(),
        Computer::new(),
    ];

    let mut signal: i64 = 0;

    // "Priming" computers/coroutines.
    for ix in 0..5 {
        let comp = &mut farm[ix];
        comp.load_memory(mem);
        comp.run_as_coroutine();
        comp.send_input(permutation[ix]);
        comp.send_input(signal);
        signal = comp.peek_output().unwrap();
    }

    // Feedback loop. We schedule coroutings sequentially ("round
    // robin" style) and just use a single `signal` variable to pass
    // around signals. Also, we keep track of halted coroutines and
    // terminate once all coroutings halted.
    let mut halted = [false; 5];
    loop {
        for ix in 0..5 {
            let comp = &mut farm[ix];
            comp.run_as_coroutine();
            match comp.status {
                Status::RequiresInput => {
                    comp.send_input(signal);

                    // Here we rely on the fact that coroutines always
                    // produce output for every input and then either
                    // require more input or halt, as per
                    // specification.
                    signal = comp.peek_output().unwrap();
                }
                Status::ProducedOutput(_) => panic!("we don't expect output here"),
                Status::Halted => {
                    halted[ix] = true;
                }
            }
        }

        if halted.iter().all(|x| *x) {
            break;
        }
    }

    signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let memory = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(enumerate_phases(&memory), 43210);
    }

    #[test]
    fn test_2() {
        let memory = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(enumerate_phases(&memory), 54321);
    }

    #[test]
    fn test_3() {
        let memory = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(enumerate_phases(&memory), 65210);
    }

    #[test]
    fn test_4() {
        let memory = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(enumerate_phases_2(&memory), 139629729);
    }

    #[test]
    fn test_5() {
        let memory = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(enumerate_phases_2(&memory), 18216);
    }
}
