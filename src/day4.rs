pub fn solve_part1() -> usize {
    let start = 152085;
    let end = 670283;
    (start..=end).filter(is_good).count()
}

pub fn solve_part2() -> usize {
    let start = 152085;
    let end = 670283;
    (start..=end).filter(is_good_2).count()
}

fn is_good(cand: &u32) -> bool {
    let digits: Vec<_> = cand
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let has_dup = digits.windows(2).any(|ref win| win[0] == win[1]);
    has_dup && digits.is_sorted()
}

fn is_good_2(cand: &u32) -> bool {
    let digits: Vec<_> = cand
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let mut has_pair = false;
    let mut prev = 10;
    let mut banned_num = 10;

    for d in &digits {
        if prev < 10 {
            if has_pair {
                if *d == prev {
                    has_pair = false;
                    banned_num = prev;
                } else {
                    break;
                }
            } else if *d != banned_num && *d == prev {
                has_pair = true;
            }
        }
        prev = *d;
    }

    digits.is_sorted() & has_pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_good_2(&112233), true);
        assert_eq!(is_good_2(&123444), false);
        assert_eq!(is_good_2(&111122), true);
        assert_eq!(is_good_2(&111178), false);
    }
}
