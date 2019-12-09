use crate::intcode::*;

pub fn solve_part1() {
    let mut buf = vec![];
    {
        let mut comp = Computer::new();
        comp.load_from_file("input/day5.txt");
        comp.set_str_input("1");
        comp.capture_output(&mut buf);
        comp.run();
    }
    let last = std::str::from_utf8(&buf[..])
        .unwrap()
        .lines()
        .last()
        .unwrap();
    println!("day 5, part 1: {}", last);
}

pub fn solve_part2() {
    let mut comp = Computer::new();
    comp.load_from_file("input/day5.txt");
    comp.run();
}
