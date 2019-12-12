use log::*;
use num::Integer;
use std::fs;

type Map = Vec<Vec<char>>;

pub fn solve_part1() -> usize {
    let map_data = fs::read_to_string("input/day10.txt").unwrap();
    let map = read_map(&map_data);
    solve(&map).0
}

pub fn solve_part2() -> usize {
    let map_data = fs::read_to_string("input/day10.txt").unwrap();
    let mut map = read_map(&map_data);
    solve2(&mut map).unwrap()
}

fn read_map(s: &str) -> Map {
    let mut map: Map = vec![];
    for row in s.lines() {
        let row_vec: Vec<char> = row.trim().chars().collect();
        map.push(row_vec);
    }
    map
}

fn solve(map: &Map) -> (usize, (i32, i32)) {
    let num_rows = map.len() as i32;
    let num_cols = map[0].len() as i32;

    let mut total;
    let mut max_total = 0;
    let mut max_pos = (-1, -1);

    for y in 0..num_rows {
        for x in 0..num_cols {
            if map[y as usize][x as usize] == '.' {
                continue;
            }
            total = enumerate_los_vectors(x, y, num_cols, num_rows)
                .iter()
                .map(|&los| count_visible_along_los(&map, x, y, los))
                .sum();
            if total > max_total {
                max_total = total;
                max_pos = (x, y);
            }
        }
    }

    (max_total, max_pos)
}

fn solve2(map: &mut Map) -> Option<usize> {
    let num_rows = map.len() as i32;
    let num_cols = map[0].len() as i32;
    let (x, y) = solve(map).1;
    let los_vectors = enumerate_los_vectors(x, y, num_cols, num_rows);
    let mut evaporated_so_far = 0;
    loop {
        let last_round = evaporated_so_far;
        for &los in &los_vectors {
            match evaporate_along_los(map, x, y, los) {
                Some((x, y)) => {
                    evaporated_so_far += 1;
                    if evaporated_so_far == 200 {
                        return Some((100 * x + y) as usize);
                    }
                }
                None => {}
            }
        }

        if last_round == evaporated_so_far {
            return None;
        }
    }
}

fn count_visible_along_los(map: &Map, mut x: i32, mut y: i32, los_vec: (i32, i32)) -> usize {
    let num_rows = map.len();
    let num_cols = map[0].len();

    loop {
        x += los_vec.0;
        y += los_vec.1;

        if x < 0 || x >= num_cols as i32 || y < 0 || y >= num_rows as i32 {
            break;
        }

        if map[y as usize][x as usize] == '#' {
            return 1;
        }
    }

    0
}

fn evaporate_along_los(
    map: &mut Map,
    mut x: i32,
    mut y: i32,
    los_vec: (i32, i32),
) -> Option<(i32, i32)> {
    let num_rows = map.len();
    let num_cols = map[0].len();

    loop {
        x += los_vec.0;
        y += los_vec.1;

        if x < 0 || x >= num_cols as i32 || y < 0 || y >= num_rows as i32 {
            return None;
        }

        if map[y as usize][x as usize] == '#' {
            map[y as usize][x as usize] = '.';
            return Some((x, y));
        }
    }
}

fn enumerate_los_vectors(x: i32, y: i32, width: i32, height: i32) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for i in 0..width {
        for j in 0..height {
            let dx = i - x;
            let dy = j - y;
            if dx.gcd(&dy) == 1 {
                result.push((dx, dy));
            }
        }
    }
    result.sort_unstable_by(|&(dx1, dy1), &(dx2, dy2)| {
        calc_angle(dx1 as f64, dy1 as f64)
            .partial_cmp(&calc_angle(dx2 as f64, dy2 as f64))
            .unwrap()
    });

    info!("los vectors for ({}, {}): {:?}", x, y, &result);

    result
}

fn calc_angle(dx: f64, dy: f64) -> f64 {
    // First calculate the angle counter-clockwise from X axis
    // positive direction using `atan2`.
    // See: https://en.wikipedia.org/wiki/Atan2
    let mut angle = (-dy).atan2(dx);

    // Now we need clockwise instead, so we invert the sign:
    angle = -angle;

    // And finally we want Up direction to be the first, the smallest,
    // so we shift [-pi; -pi/2) angles by 2*pi.
    if angle < -std::f64::consts::FRAC_PI_2 {
        angle += 2.0 * std::f64::consts::PI;
    }

    angle
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn assert_zero_gcd() {
        assert_eq!(0_u8.gcd(&0), 0);
    }

    #[test]
    fn test_1() {
        let map = indoc!(
            ".#..#
             .....
             #####
             ....#
             ...##"
        );
        assert_eq!(solve(&read_map(&map)), (8, (3, 4)));
    }

    #[test]
    fn test_2() {
        let map = indoc!(
            "......#.#.
             #..#.#....
             ..#######.
             .#.#.###..
             .#..#.....
             ..#....#.#
             #..#....#.
             .##.#..###
             ##...#..#.
             .#....####"
        );
        assert_eq!(solve(&read_map(&map)), (33, (5, 8)));
    }

    #[test]
    fn test_3() {
        let map = indoc!(
            "#.#...#.#.
             .###....#.
             .#....#...
             ##.#.#.#.#
             ....#.#.#.
             .##..###.#
             ..#...##..
             ..##....##
             ......#...
             .####.###."
        );
        assert_eq!(solve(&read_map(&map)), (35, (1, 2)));
    }

    #[test]
    fn test_4() {
        let map = indoc!(
            ".#..#..###
             ####.###.#
             ....###.#.
             ..###.##.#
             ##.##.#.#.
             ....###..#
             ..#.#..#.#
             #..#.#.###
             .##...##.#
             .....#.#.."
        );
        assert_eq!(solve(&read_map(&map)), (41, (6, 3)));
    }

    #[test]
    fn test_5() {
        let map = indoc!(
            ".#..##.###...#######
             ##.############..##.
             .#.######.########.#
             .###.#######.####.#.
             #####.##.#.##.###.##
             ..#####..#.#########
             ####################
             #.####....###.#.#.##
             ##.#################
             #####.##.###..####..
             ..######..##.#######
             ####.##.####...##..#
             .#####..#.######.###
             ##...#.##########...
             #.##########.#######
             .####.#.###.###.#.##
             ....##.##.###..#####
             .#.#.###########.###
             #.#.#.#####.####.###
             ###.##.####.##.#..##"
        );
        assert_eq!(solve(&read_map(&map)), (210, (11, 13)));
    }
}
