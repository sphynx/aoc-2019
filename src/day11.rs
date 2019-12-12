use crate::intcode::*;
use std::collections::HashMap;

pub fn solve_part1() -> usize {
    let mut comp = Computer::new();
    comp.load_from_file("input/day11.txt");

    let mut hull = Hull::new();
    let mut pos = Position { x: 0, y: 0 };
    let mut dir = Dir::Up;

    loop {
        let color_below = hull.look(&pos);
        comp.run_as_coroutine();
        match comp.status {
            Status::Halted => break,
            Status::RequiresInput => comp.send_input(color_below as i64),
            Status::ProducedOutput(_) => panic!("unexpected output"),
        }
        let paint_in = Color::from(comp.peek_output().unwrap());
        comp.run_as_coroutine();
        let turn = Turn::from(comp.peek_output().unwrap());
        hull.paint(&pos, paint_in);
        dir = dir.turn(&turn);
        pos.go(&dir);
    }
    hull.painted_plates()
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

enum Turn {
    TurnLeft,
    TurnRight,
}

impl Dir {
    pub fn turn(self, t: &Turn) -> Self {
        use Dir::*;
        use Turn::*;
        match (self, t) {
            (Up, TurnLeft) => Left,
            (Up, TurnRight) => Right,
            (Down, TurnLeft) => Right,
            (Down, TurnRight) => Left,
            (Left, TurnLeft) => Down,
            (Left, TurnRight) => Up,
            (Right, TurnLeft) => Up,
            (Right, TurnRight) => Down,
        }
    }
}

impl From<i64> for Turn {
    fn from(x: i64) -> Self {
        match x {
            0 => Turn::TurnLeft,
            1 => Turn::TurnRight,
            _ => panic!("wrong int for Turn"),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Color {
    Black = 0,
    White = 1,
}

impl From<i64> for Color {
    fn from(x: i64) -> Self {
        match x {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("wrong int for Color"),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn go(&mut self, dir: &Dir) {
        use Dir::*;
        match dir {
            Up => self.y -= 1,
            Down => self.y += 1,
            Left => self.x -= 1,
            Right => self.x += 1,
        }
    }
}

struct Hull(HashMap<Position, Color>);

impl Hull {
    pub fn new() -> Self {
        Hull(HashMap::new())
    }

    pub fn look(&self, pos: &Position) -> Color {
        match self.0.get(&pos) {
            None => Color::Black,
            Some(color) => *color,
        }
    }

    pub fn paint(&mut self, pos: &Position, color: Color) {
        self.0.insert(*pos, color);
    }

    pub fn painted_plates(&self) -> usize {
        self.0.len()
    }
}
