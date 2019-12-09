use crate::intcode::*;
use permutohedron::Heap;

pub fn solve_part1() -> u32 {
    let memory = Computer::read_program("input/day7.txt");
    check_phases(&memory)
}

fn check_phases(mem: &[i64]) -> u32 {
    let mut phases = ["0", "1", "2", "3", "4"];
    Heap::new(&mut phases)
        .map(|phases| run_pipeline(&phases, &mem, 5))
        .max()
        .unwrap()
}

fn run_pipeline(permutation: &[&str], mem: &[i64], times: usize) -> u32 {
    let mut comp = Computer::new();
    let mut out: u32 = 0;

    for ix in 0..times {
        comp.load_memory(mem);
        comp.set_input_lines(&[permutation[ix], &out.to_string()]);
        comp.run();
        out = comp.last_output().parse().unwrap();
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
        assert_eq!(check_phases(&memory), 43210);
    }

    #[test]
    fn test_2() {
        let memory = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(check_phases(&memory), 54321);
    }

    #[test]
    fn test_3() {
        let memory = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(check_phases(&memory), 65210);
    }
}
