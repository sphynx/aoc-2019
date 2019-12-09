use crate::intcode::*;

pub fn solve_part1() -> i64 {
    let mut comp = Computer::new();
    comp.load_from_file("input/day9.txt");
    comp.set_input_lines(&["1"]);
    comp.run();
    comp.last_output().parse().unwrap()
}

pub fn solve_part2() -> i64 {
    let mut comp = Computer::new();
    comp.load_from_file("input/day9.txt");
    comp.set_input_lines(&["2"]);
    comp.run();
    comp.last_output().parse().unwrap()
}
