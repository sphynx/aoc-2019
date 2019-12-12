use aoc_2019::*;

use env_logger;
use std::io;

fn main() -> io::Result<()> {
    env_logger::init();
    day11::solve_part2();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1_solution() {
        assert_eq!(day1::solve_part1(), 3296269);
        assert_eq!(day1::solve_part2(), 4941547);
    }

    #[test]
    fn day2_solution() {
        assert_eq!(day2::solve_part1(), 3166704);
        assert_eq!(day2::solve_part2(), 8018);
    }

    #[test]
    fn day3_solution() {
        assert_eq!(day3::solve_part1(), 855);
        assert_eq!(day3::solve_part2(), 11238);
    }

    #[test]
    #[ignore] // It's too long!
    fn day4_solution() {
        assert_eq!(day4::solve_part1(), 1764);
        assert_eq!(day4::solve_part2(), 1196);
    }

    #[test]
    fn day5_solution() {
        assert_eq!(day5::solve_part1(), 15097178);
        assert_eq!(day5::solve_part2(), 1558663);
    }

    #[test]
    fn day6_solution() {
        assert_eq!(day6::solve_part1(), 223251);
        assert_eq!(day6::solve_part2(), 430);
    }

    #[test]
    fn day7_solution() {
        assert_eq!(day7::solve_part1(), 30940);
        assert_eq!(day7::solve_part2(), 76211147);
    }

    #[test]
    fn day8_solution() {
        assert_eq!(day8::solve_part1(), 1206);
    }

    #[test]
    fn day9_solution() {
        assert_eq!(day9::solve_part1(), 3533056970);
        assert_eq!(day9::solve_part2(), 72852);
    }

    #[test]
    fn day10_solution() {
        assert_eq!(day10::solve_part1(), 221);
        assert_eq!(day10::solve_part2(), 806);
    }

    #[test]
    fn day11_solution() {
        assert_eq!(day11::solve_part1(), 2056);
    }
}
