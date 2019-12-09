use crate::intcode::*;
use permutohedron::Heap;

pub fn solve_part1() -> u32 {
    let mut phases = ["0", "1", "2", "3", "4"];
    Heap::new(&mut phases)
        .map(|phases| run_pipeline(&phases, 5))
        .max()
        .unwrap()
}

fn run_pipeline(permutation: &[&str], times: usize) -> u32 {
    let mut comp = Computer::new();
    let mut out: u32 = 0;

    for ix in 0..times {
        comp.load_from_file("input/day7.txt");
        comp.set_input_lines(&[permutation[ix], &out.to_string()]);
        comp.run();
        out = comp.last_output().parse().unwrap();
    }

    out
}
