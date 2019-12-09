use std::cmp;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
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

pub fn solve_part1() {
    let data = std::fs::read_to_string("input/day3.txt").unwrap();
    let programs: Vec<_> = data.lines().map(mk_program).collect();
    let (_, _, dist) = ppm::execute_programs(&programs);
    println!("day 3, part 1: {}", dist);
}

pub fn draw_part1() -> io::Result<()> {
    let data = std::fs::read_to_string("input/day3.txt").unwrap();
    let programs: Vec<_> = data.lines().map(mk_program).collect();
    svg::print(&programs);
    // ppm::print(&programs)?;
    Ok(())
}

fn mk_program(s: &str) -> Vec<Move> {
    s.trim()
        .split(",")
        .map(|s| Move::from_str(s).unwrap())
        .collect()
}


#[allow(dead_code)]
mod ppm {
    use super::*;

    pub fn execute_programs(progs: &Vec<Vec<Move>>) -> (Frame, Vec<u8>, u32) {
        let frame = Frame::new_from_list(&progs);
        let mut bitmap: Vec<u8> = vec![0; (frame.width * frame.height) as usize];
        let mut dist = u32::max_value();
        for (ix, prog) in progs.iter().enumerate() {
            dist = cmp::min(dist, mark_path(&mut bitmap, &frame, (ix + 1) as u8, prog));
        }
        (frame, bitmap, dist)
    }

    fn mark_path<'a>(bitmap: &mut Vec<u8>, frame: &Frame, val: u8, prog: Program<'a>) -> u32 {
        let convert_to_1d = |x: i32, y: i32| {
            ((y - frame.y) as usize) * (frame.width as usize) + ((x - frame.x) as usize)
        };

        let mut curr_x: i32 = 0;
        let mut curr_y: i32 = 0;
        bitmap[convert_to_1d(curr_x, curr_y)] = val;

        // FIXME: this is just a hack, we calculate minimal distance
        // here, even though this code was originally intended just
        // for tracing paths in a bitmap
        let mut dist = u32::max_value();

        // FIXME: this is ugly and has lots of copy paste
        for mov in prog.iter() {
            match *mov {
                Move::Up(dy) => {
                    for _ in 0..dy {
                        curr_y -= 1;
                        let ix = convert_to_1d(curr_x, curr_y);
                        if bitmap[ix] > 0 && bitmap[ix] < val {
                            dist = cmp::min(dist, (curr_x.abs() + curr_y.abs()) as u32)
                        }
                        bitmap[ix] = val;
                    }
                }
                Move::Down(dy) => {
                    for _ in 0..dy {
                        curr_y += 1;
                        let ix = convert_to_1d(curr_x, curr_y);
                        if bitmap[ix] > 0 && bitmap[ix] < val {
                            dist = cmp::min(dist, (curr_x.abs() + curr_y.abs()) as u32)
                        }
                        bitmap[ix] = val;
                    }
                }
                Move::Left(dx) => {
                    for _ in 0..dx {
                        curr_x -= 1;
                        let ix = convert_to_1d(curr_x, curr_y);
                        if bitmap[ix] > 0 && bitmap[ix] < val {
                            dist = cmp::min(dist, (curr_x.abs() + curr_y.abs()) as u32)
                        }
                        bitmap[ix] = val;
                    }
                }
                Move::Right(dx) => {
                    for _ in 0..dx {
                        curr_x += 1;
                        let ix = convert_to_1d(curr_x, curr_y);
                        if bitmap[ix] > 0 && bitmap[ix] < val {
                            dist = cmp::min(dist, (curr_x.abs() + curr_y.abs()) as u32)
                        }
                        bitmap[ix] = val;
                    }
                }
            }
        }
        dist
    }

    pub fn print(progs: &Vec<Vec<Move>>) -> io::Result<()> {
        let (frame, bitmap, _) = execute_programs(progs);
        print_image_as_ppm(&frame, &bitmap)?;
        Ok(())
    }

    fn print_image_as_ppm(frame: &Frame, bitmap: &[u8]) -> io::Result<()> {
        let mut buffer = BufWriter::new(File::create("1.ppm")?);
        writeln!(buffer, "P3 {} {} 1", frame.width, frame.height)?;
        for b in bitmap.iter() {
            match *b {
                1 => write!(buffer, "1 0 0 ")?,
                2 => write!(buffer, "0 1 0 ")?,
                3 => write!(buffer, "0 0 1 ")?,
                _ => write!(buffer, "1 1 1 ")?,
            }
        }
        buffer.flush()?;
        Ok(())
    }
}

mod svg {
    use super::*;

    pub fn print(progs: &Vec<Vec<Move>>) {
        let f = Frame::new_from_list(&progs);
        println!(
            r#"<svg width="{}" height="{}" viewBox="{} {} {} {}" xmlns="http://www.w3.org/2000/svg">"#,
            f.width / 5,
            f.height / 5,
            f.x,
            f.y,
            f.width,
            f.height
        );
        println!(r#"<g fill="transparent" stroke-width="5">"#);
        for (ix, prog) in progs.iter().enumerate() {
            print_prog(ix as u32, prog)
        }
        println!("</g>");
        println!("</svg>");
    }

    fn print_prog(ix: u32, prog: &[Move]) {
        print!("<path d=\"M 0 0 ");
        for mov in prog.iter() {
            match *mov {
                Move::Up(dy) => print!("v {} ", -(dy as i32)),
                Move::Down(dy) => print!("v {} ", dy),
                Move::Left(dx) => print!("h {} ", -(dx as i32)),
                Move::Right(dx) => print!("h {} ", dx),
            }
        }
        let color = match ix {
            0 => "red",
            1 => "green",
            _ => "black",
        };
        println!("\" stroke=\"{}\" />", color);
    }
}
