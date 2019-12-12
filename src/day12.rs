use log::info;
use std::cmp::Ordering;
use std::fs;

pub fn solve_part1() -> i64 {
    let input = fs::read_to_string("input/day12.txt").unwrap();
    let moons = parse_data(input);
    simulate(moons, 1000)
}

fn simulate(mut moons: Vec<Moon>, niter: usize) -> i64 {
    for _ in 0..niter {
        for i in 1..moons.len() {
            let (head, tail) = moons.split_at_mut(i);
            let this = head.last_mut().unwrap();
            for other in tail {
                apply_gravity(this, other);
            }
        }

        for moon in &mut moons {
            moon.apply_velocity();
            info!("{:?}", moon);
        }
    }

    moons.iter().map(|m| m.total_energy()).sum()
}

fn parse_data(input: String) -> Vec<Moon> {
    let mut moons = vec![];
    for moon_str in input.lines() {
        let parts: Vec<&str> = moon_str
            .trim_matches(|c| c == '<' || c == '>')
            .split(",")
            .collect();

        let x = parts[0].trim().split("=").nth(1).unwrap().parse().unwrap();
        let y = parts[1].trim().split("=").nth(1).unwrap().parse().unwrap();
        let z = parts[2].trim().split("=").nth(1).unwrap().parse().unwrap();

        moons.push(Moon {
            x,
            y,
            z,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        });
    }

    moons
}

#[derive(Debug)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

impl Moon {
    fn apply_velocity(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.z += self.vel_z;
    }

    fn potential_energy(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i64 {
        self.vel_x.abs() + self.vel_y.abs() + self.vel_z.abs()
    }

    fn total_energy(&self) -> i64 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn apply_gravity(moon1: &mut Moon, moon2: &mut Moon) {
    match moon1.x.cmp(&moon2.x) {
        Ordering::Less => {
            moon1.vel_x += 1;
            moon2.vel_x -= 1;
        }
        Ordering::Greater => {
            moon1.vel_x -= 1;
            moon2.vel_x += 1;
        }
        Ordering::Equal => {}
    }

    match moon1.y.cmp(&moon2.y) {
        Ordering::Less => {
            moon1.vel_y += 1;
            moon2.vel_y -= 1;
        }
        Ordering::Greater => {
            moon1.vel_y -= 1;
            moon2.vel_y += 1;
        }
        Ordering::Equal => {}
    }

    match moon1.z.cmp(&moon2.z) {
        Ordering::Less => {
            moon1.vel_z += 1;
            moon2.vel_z -= 1;
        }
        Ordering::Greater => {
            moon1.vel_z -= 1;
            moon2.vel_z += 1;
        }
        Ordering::Equal => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_1() {
        let input = indoc!(
            "<x=-1, y=0, z=2>
             <x=2, y=-10, z=-7>
             <x=4, y=-8, z=8>
             <x=3, y=5, z=-1>"
        );
        let moons = parse_data(String::from(input));
        assert_eq!(simulate(moons, 10), 179);
    }

    #[test]
    fn test_2() {
        let input = indoc!(
            "<x=-8, y=-10, z=0>
             <x=5, y=5, z=10>
             <x=2, y=-7, z=3>
             <x=9, y=-8, z=-3>"
        );
        let moons = parse_data(String::from(input));
        assert_eq!(simulate(moons, 100), 1940);
    }
}
