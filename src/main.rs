use aoc_2019::*;
use std::io;

fn main() -> io::Result<()> {
    println!("day 6, part 2: {}", day6::solve_part2());
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
}
