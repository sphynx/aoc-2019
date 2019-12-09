use crate::intcode::*;
use permutohedron::Heap;

pub fn solve_part1() -> i64 {
    let memory = Computer::read_program("input/day7.txt");
    enumerate_phases(&memory)
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
}
