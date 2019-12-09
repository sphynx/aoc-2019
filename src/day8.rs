use std::fs;

pub fn solve_part1() -> usize {
    let input = fs::read_to_string("input/day8.txt").unwrap();
    let min_ix = input
        .trim()
        .as_bytes()
        .chunks(25 * 6)
        .enumerate()
        .min_by_key(|(_ix, layer)| count_char(layer, '0'))
        .unwrap()
        .0;

    let min_layer = input.as_bytes().chunks(25 * 6).nth(min_ix).unwrap();
    count_char(min_layer, '1') * count_char(min_layer, '2')
}

pub fn solve_part2() {
    let input = fs::read_to_string("input/day8.txt").unwrap();
    let bytes = input.trim().as_bytes();

    let mut image = vec![2; 25 * 6];

    // Handle layering.
    for y in 0..6 {
        for x in 0..25 {
            let coord_1d = y * 25 + x;
            for layer in (&bytes).chunks(25 * 6) {
                if image[coord_1d] == 2 {
                    image[coord_1d] = layer[coord_1d] - '0' as u8;
                }
            }
        }
    }

    // Display resulting image.
    for y in 0..6 {
        for x in 0..25 {
            let coord_1d = y * 25 + x;
            print!(
                "{}",
                match image[coord_1d] {
                    1 => "*",
                    0 => " ",
                    _ => "!",
                }
            );
        }
        println!();
    }
}

fn count_char(layer: &[u8], c: char) -> usize {
    layer.iter().filter(|x| **x as char == c).count()
}
