use std::fs;

pub fn solve_part1() -> u64 {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    input.lines().map(|x| fuel(x.parse().unwrap())).sum()
}

pub fn solve_part2() -> u64 {
    let input = fs::read_to_string("input/day1.txt").unwrap();
    input.lines().map(|x| total_fuel(x.parse().unwrap())).sum()
}

fn fuel(mass: u64) -> u64 {
    (mass / 3).saturating_sub(2)
}

fn total_fuel(mut mass: u64) -> u64 {
    let mut total = 0;
    while mass > 0 {
        mass = fuel(mass);
        total += mass;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(14), 2);
        assert_eq!(total_fuel(1969), 966);
        assert_eq!(total_fuel(100_756), 50_346);
    }
}
