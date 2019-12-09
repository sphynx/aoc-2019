use crate::intcode::*;

pub fn solve_part1() -> u32 {
    let mut buf = vec![];
    {
        let mut comp = Computer::new();
        comp.load_from_file("input/day5.txt");
        comp.set_str_input("1");
        comp.capture_output(&mut buf);
        comp.run();
    }
    std::str::from_utf8(&buf[..])
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}

pub fn solve_part2() -> u32 {
    let mut buf = vec![];
    {
        let mut comp = Computer::new();
        comp.load_from_file("input/day5.txt");
        comp.set_str_input("5");
        comp.capture_output(&mut buf);
        comp.run();
    }

    std::str::from_utf8(&buf[..])
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .parse()
        .unwrap()
}
