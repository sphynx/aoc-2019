use crate::intcode::*;

use std::collections::HashMap;

use image::ImageBuffer;

pub fn solve_part1() -> usize {
    let mut comp = Computer::new();
    comp.load_from_file("input/day11.txt");
    let mut hull = Hull::new();
    run_robot(&mut comp, &mut hull, Position { x: 0, y: 0 }, Dir::Up);
    hull.painted_plates()
}

pub fn solve_part2() {
    let mut comp = Computer::new();
    comp.load_from_file("input/day11.txt");

    let mut hull = Hull::new();
    let pos = Position { x: 0, y: 0 };
    hull.paint(&pos, Color::White);

    run_robot(&mut comp, &mut hull, pos, Dir::Up);
    draw(hull);
}

fn draw(hull: Hull) {
    let pixels = hull.pixels();

    let min_x = pixels.iter().map(|&p| p.0.x).min().unwrap();
    let min_y = pixels.iter().map(|&p| p.0.y).min().unwrap();
    let max_x = pixels.iter().map(|&p| p.0.x).max().unwrap();
    let max_y = pixels.iter().map(|&p| p.0.y).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;

    let mut img = ImageBuffer::new(width as u32, height as u32);

    for pix in pixels {
        let c = match pix.1 {
            Color::Black => image::Luma([0]),
            Color::White => image::Luma([255]),
        };
        img.put_pixel(pix.0.x as u32, pix.0.y as u32, c);
    }

    img.save("plate.png").unwrap();
}

fn run_robot(comp: &mut Computer, hull: &mut Hull, mut pos: Position, mut dir: Dir) {
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

    pub fn pixels(self) -> Vec<(Position, Color)> {
        let mut pixs = vec![];
        for (&coords, &color) in self.0.iter() {
            pixs.push((coords, color));
        }
        pixs
    }
}
