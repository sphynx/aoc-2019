use num::Integer;
use std::fs;
use log::*;

type Map = Vec<Vec<char>>;

pub fn solve_part1() -> usize {
    let map_data = fs::read_to_string("input/day10.txt").unwrap();
    let map = read_map(&map_data);
    solve(&map).0
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
    info!("los vectors for ({}, {}): {:?}", x, y, &result);
    result
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
