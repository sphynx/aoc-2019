use std::cmp::Ordering;
use std::fs;

pub fn solve_part1() -> i64 {
    let input = fs::read_to_string("input/day12.txt").unwrap();
    let mut system = System::from_str(&input);
    system.run(1000);
    system.total_energy()
}

pub fn solve_part2() -> usize {
    let input = fs::read_to_string("input/day12.txt").unwrap();
    let mut system = System::from_str(&input);
    let niter = 1_000_000_000;
    system.run(niter);
    println!("after {} iterations: {:?}", niter, system);
    0
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Moon {
    x: i64,
    y: i64,
    z: i64,
    vel_x: i64,
    vel_y: i64,
    vel_z: i64,
}

#[derive(Debug)]
pub struct System {
    moons: Vec<Moon>,
}

impl System {
    pub fn new(moons: Vec<Moon>) -> Self {
        System { moons }
    }

    pub fn from_str(input: &str) -> System {
        let mut moons = vec![];
        for moon_str in input.lines() {
            let parts: Vec<&str> = moon_str
                .trim_matches(|c| c == '<' || c == '>')
                .split(",")
                .collect();

            fn parse_coord(parts: &Vec<&str>, ix: usize) -> i64 {
                parts[ix].trim().split("=").nth(1).unwrap().parse().unwrap()
            }

            let x = parse_coord(&parts, 0);
            let y = parse_coord(&parts, 1);
            let z = parse_coord(&parts, 2);

            moons.push(Moon::new(x, y, z));
        }

        System::new(moons)
    }

    pub fn step(&mut self) {
        for i in 1..self.moons.len() {
            let (head, tail) = self.moons.split_at_mut(i);
            let this = head.last_mut().unwrap();
            for other in tail {
                this.apply_gravity(other);
            }
        }

        for moon in &mut self.moons {
            moon.apply_velocity();
        }
    }

    pub fn run(&mut self, niter: usize) {
        for _ in 0..niter {
            self.step();
        }
    }

    pub fn total_energy(&self) -> i64 {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Moon {
            x,
            y,
            z,
            vel_x: 0,
            vel_y: 0,
            vel_z: 0,
        }
    }

    fn apply_velocity(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
        self.z += self.vel_z;
    }

    fn apply_gravity(&mut self, other: &mut Moon) {
        Self::adjust_velocity(&self.x, &other.x, &mut self.vel_x, &mut other.vel_x);
        Self::adjust_velocity(&self.y, &other.y, &mut self.vel_y, &mut other.vel_y);
        Self::adjust_velocity(&self.z, &other.z, &mut self.vel_z, &mut other.vel_z);
    }

    fn adjust_velocity(coord1: &i64, coord2: &i64, vel1: &mut i64, vel2: &mut i64) {
        match coord1.cmp(coord2) {
            Ordering::Less => {
                *vel1 += 1;
                *vel2 -= 1;
            }
            Ordering::Greater => {
                *vel1 -= 1;
                *vel2 += 1;
            }
            Ordering::Equal => {}
        }
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
        let mut system = System::from_str(input);
        system.run(10);
        assert_eq!(system.total_energy(), 179);
    }

    #[test]
    fn test_2() {
        let input = indoc!(
            "<x=-8, y=-10, z=0>
             <x=5, y=5, z=10>
             <x=2, y=-7, z=3>
             <x=9, y=-8, z=-3>"
        );
        let mut system = System::from_str(input);
        system.run(100);
        assert_eq!(system.total_energy(), 1940);
    }

    // #[test]
    // fn test_3() {
    //     let input = indoc!(
    //         "<x=-1, y=0, z=2>
    //          <x=2, y=-10, z=-7>
    //          <x=4, y=-8, z=8>
    //          <x=3, y=5, z=-1>"
    //     );
    //     let mut system = parse_data(String::from(input));
    //     system.run_until_repeat();
    //     assert_eq!(system.iterations(), 2772);
    // }
}
