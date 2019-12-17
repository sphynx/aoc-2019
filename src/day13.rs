use crate::intcode::*;

#[derive(PartialEq)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<i64> for Tile {
    // Hm, we should be able to derive this! Apparently, the go-to way
    // for that is now `num_enum` crate.
    fn from(x: i64) -> Self {
        match x {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("bad tile number: {}", x),
        }
    }
}

pub fn solve_part1() -> usize {
    let mut comp = Computer::new();
    comp.load_from_file("input/day13.txt");
    comp.run();

    fn is_block(s: &str) -> bool {
        Tile::from(s.parse::<i64>().unwrap()) == Tile::Block
    }

    comp.output
        .iter()
        .skip(2)
        .step_by(3)
        .filter(|s| is_block(s))
        .count()
}
