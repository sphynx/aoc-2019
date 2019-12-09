use crate::intcode::*;

pub fn solve_part1() -> u32 {
    let mut comp = Computer::new();
    comp.load_from_file("input/day5.txt");
    comp.set_str_input("1");
    let output = comp.run();
    output.iter().last().unwrap().parse().unwrap()
}

pub fn solve_part2() -> u32 {
    let mut comp = Computer::new();
    comp.load_from_file("input/day5.txt");
    comp.set_str_input("5");
    let output = comp.run();
    output.iter().last().unwrap().parse().unwrap()
}
