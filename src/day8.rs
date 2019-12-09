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

fn count_char(layer: &[u8], c: char) -> usize {
    layer.iter().filter(|x| **x as char == c).count()
}
