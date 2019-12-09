use std::cmp;
use std::collections::HashSet;
use std::str::FromStr;

use itertools::Itertools;

type Program<'a> = &'a [Move];

pub enum Move {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() > 1);
        let (head, tail) = s.split_at(1);
        let tail = tail.parse().unwrap();
        match head {
            "U" => Ok(Move::Up(tail)),
            "D" => Ok(Move::Down(tail)),
            "L" => Ok(Move::Left(tail)),
            "R" => Ok(Move::Right(tail)),
            _ => Err(format!("Wrong move: {}", s)),
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Frame {
    fn new(prog: &[Move]) -> Self {
        let mut min_x = 0;
        let mut max_x = 0;
        let mut min_y = 0;
        let mut max_y = 0;
        let mut curr_x: i32 = 0;
        let mut curr_y: i32 = 0;

        for mov in prog.iter() {
            match *mov {
                Move::Up(dy) => curr_y -= dy as i32,
                Move::Down(dy) => curr_y += dy as i32,
                Move::Left(dx) => curr_x -= dx as i32,
                Move::Right(dx) => curr_x += dx as i32,
            }

            min_x = cmp::min(min_x, curr_x);
            max_x = cmp::max(max_x, curr_x);
            min_y = cmp::min(min_y, curr_y);
            max_y = cmp::max(max_y, curr_y);
        }

        Frame {
            x: min_x,
            y: min_y,
            width: (max_x - min_x + 1) as u32,
            height: (max_y - min_y + 1) as u32,
        }
    }

    fn combine_with(&self, other: &Frame) -> Self {
        let x_min = cmp::min(self.x, other.x);
        let y_min = cmp::min(self.y, other.y);

        let x_max = cmp::max(
            self.x + self.width as i32 - 1,
            other.x + other.width as i32 - 1,
        );
        let y_max = cmp::max(
            self.y + self.height as i32 - 1,
            other.y + other.height as i32 - 1,
        );

        Frame {
            x: x_min,
            y: y_min,
            width: (x_max - x_min + 1) as u32,
            height: (y_max - y_min + 1) as u32,
        }
    }

    fn new_from_list(progs: &Vec<Vec<Move>>) -> Self {
        progs
            .iter()
            .map(|p| Frame::new(p))
            .fold1(|acc, f| acc.combine_with(&f))
            .unwrap()
    }
}

pub fn solve_part1() -> u32 {
    let data = std::fs::read_to_string("input/day3.txt").unwrap();
    let programs: Vec<_> = data.lines().map(mk_program).collect();

    let frame = Frame::new_from_list(&programs);
    let mut bitmap: Vec<u8> = vec![0; (frame.width * frame.height) as usize];

    mark_path_with_val(&mut bitmap, &frame, 1, &programs[0]);
    let dist = follow_path_with_min_dist(&mut bitmap, &frame, &programs[1]);

    // println!("day 3, part 1: {}", dist);
    dist
}

pub fn solve_part1_with_hashset() -> u32 {
    let data = std::fs::read_to_string("input/day3.txt").unwrap();
    let programs: Vec<_> = data.lines().map(mk_program).collect();

    let frame = Frame::new_from_list(&programs);

    let mut sparse = HashSet::with_capacity(128_000);
    mark_path_with_val_alt(&mut sparse, &frame, &programs[0]);
    let dist = follow_path_with_min_dist_alt(&mut sparse, &frame, &programs[1]);

    // println!("day 3, part 1: {}", dist);
    dist
}

pub fn solve_part2() {
    let data = std::fs::read_to_string("input/day3.txt").unwrap();
    let programs: Vec<_> = data.lines().map(mk_program).collect();

    let frame = Frame::new_from_list(&programs);
    let mut bitmap: Vec<u32> = vec![0; (frame.width * frame.height) as usize];

    mark_path_with_dist(&mut bitmap, &frame, &programs[0]);
    let min_total_steps = follow_path_with_total_steps(&mut bitmap, &frame, &programs[1]);

    println!("day 3, part 2: {}", min_total_steps);
}

fn mk_program(s: &str) -> Vec<Move> {
    s.trim()
        .split(",")
        .map(|s| Move::from_str(s).unwrap())
        .collect()
}

fn mark_path_with_val<'a>(bitmap: &mut Vec<u8>, frame: &Frame, val: u8, prog: Program<'a>) {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    bitmap[convert_to_1d(frame, curr_x, curr_y)] = val;

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = val;
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = val;
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = val;
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = val;
                }
            }
        }
    }
}

fn mark_path_with_val_alt<'a>(sparse: &mut HashSet<u32>, frame: &Frame, prog: Program<'a>) {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    sparse.insert(convert_to_1d_u32(frame, curr_x, curr_y));

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    sparse.insert(convert_to_1d_u32(frame, curr_x, curr_y));
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    sparse.insert(convert_to_1d_u32(frame, curr_x, curr_y));
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    sparse.insert(convert_to_1d_u32(frame, curr_x, curr_y));
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    sparse.insert(convert_to_1d_u32(frame, curr_x, curr_y));
                }
            }
        }
    }
}


fn mark_path_with_dist<'a>(bitmap: &mut Vec<u32>, frame: &Frame, prog: Program<'a>) {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    let mut dist = 0;
    bitmap[convert_to_1d(frame, curr_x, curr_y)] = dist;

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    dist += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = dist;
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    dist += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = dist;
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    dist += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = dist;
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    dist += 1;
                    bitmap[convert_to_1d(frame, curr_x, curr_y)] = dist;
                }
            }
        }
    }
}

fn follow_path_with_min_dist<'a>(bitmap: &mut Vec<u8>, frame: &Frame, prog: Program<'a>) -> u32 {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    let mut dist = u32::max_value();

    fn upd(bitmap: &mut Vec<u8>, frame: &Frame, curr_x: i32, curr_y: i32, dist: &mut u32) {
        let ix = convert_to_1d(frame, curr_x, curr_y);
        if bitmap[ix] > 0 {
            *dist = cmp::min(*dist, (curr_x.abs() + curr_y.abs()) as u32)
        }
    }

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    upd(bitmap, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    upd(bitmap, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    upd(bitmap, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    upd(bitmap, frame, curr_x, curr_y, &mut dist);
                }
            }
        }
    }
    dist
}

fn follow_path_with_min_dist_alt<'a>(sparse: &mut HashSet<u32>, frame: &Frame, prog: Program<'a>) -> u32 {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;
    let mut dist = u32::max_value();

    fn upd(sprase: &mut HashSet<u32>, frame: &Frame, curr_x: i32, curr_y: i32, dist: &mut u32) {
        let ix = convert_to_1d_u32(frame, curr_x, curr_y);
        if sprase.contains(&ix) {
            *dist = cmp::min(*dist, (curr_x.abs() + curr_y.abs()) as u32)
        }
    }

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    upd(sparse, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    upd(sparse, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    upd(sparse, frame, curr_x, curr_y, &mut dist);
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    upd(sparse, frame, curr_x, curr_y, &mut dist);
                }
            }
        }
    }
    dist
}


fn follow_path_with_total_steps<'a>(
    bitmap: &mut Vec<u32>,
    frame: &Frame,
    prog: Program<'a>,
) -> u32 {
    let mut curr_x: i32 = 0;
    let mut curr_y: i32 = 0;

    let mut total_steps: u32 = 0;
    let mut min_total_steps: u32 = u32::max_value();

    fn upd(
        bitmap: &mut Vec<u32>,
        frame: &Frame,
        curr_x: i32,
        curr_y: i32,
        total_steps: &mut u32,
        min_total_steps: &mut u32,
    ) {
        let ix = convert_to_1d(frame, curr_x, curr_y);
        *total_steps += 1;
        if bitmap[ix] > 0 {
            *min_total_steps = cmp::min(*min_total_steps, bitmap[ix] + *total_steps);
        }
    }

    for mov in prog.iter() {
        match *mov {
            Move::Up(dy) => {
                for _ in 0..dy {
                    curr_y -= 1;
                    upd(
                        bitmap,
                        frame,
                        curr_x,
                        curr_y,
                        &mut total_steps,
                        &mut min_total_steps,
                    );
                }
            }
            Move::Down(dy) => {
                for _ in 0..dy {
                    curr_y += 1;
                    upd(
                        bitmap,
                        frame,
                        curr_x,
                        curr_y,
                        &mut total_steps,
                        &mut min_total_steps,
                    );
                }
            }
            Move::Left(dx) => {
                for _ in 0..dx {
                    curr_x -= 1;
                    upd(
                        bitmap,
                        frame,
                        curr_x,
                        curr_y,
                        &mut total_steps,
                        &mut min_total_steps,
                    );
                }
            }
            Move::Right(dx) => {
                for _ in 0..dx {
                    curr_x += 1;
                    upd(
                        bitmap,
                        frame,
                        curr_x,
                        curr_y,
                        &mut total_steps,
                        &mut min_total_steps,
                    );
                }
            }
        }
    }
    min_total_steps
}

fn convert_to_1d_u32(frame: &Frame, x: i32, y: i32) -> u32 {
    ((y - frame.y) as u32) * (frame.width as u32) + ((x - frame.x) as u32)
}

fn convert_to_1d(frame: &Frame, x: i32, y: i32) -> usize {
    ((y - frame.y) as usize) * (frame.width as usize) + ((x - frame.x) as usize)
}
