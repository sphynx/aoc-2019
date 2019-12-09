use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;

type OrbitalMap = HashMap<String, Vec<String>>;

pub fn solve_part1() {
    let orbital_map_content = fs::read_to_string("input/day6.txt").unwrap();
    let orbital_map = to_directed_graph(&orbital_map_content);
    println!("day 6 part 1: {}", solve(&orbital_map, "COM"));
}

pub fn solve_part2() {
    let orbital_map_content = fs::read_to_string("input/day6.txt").unwrap();
    let orbital_map = to_undirected_graph(&orbital_map_content);
    println!(
        "day 6 part 2: {}",
        min_distance(&orbital_map, "YOU", "SAN").unwrap() - 2
    );
}

fn to_directed_graph(data: &str) -> OrbitalMap {
    let mut orbital_map: OrbitalMap = HashMap::new();
    for line in data.split_whitespace() {
        let parts: Vec<_> = line.trim().split(")").collect();
        assert!(parts.len() == 2);
        let orbit_centre = String::from(parts[0]);
        let orbitant = String::from(parts[1]);
        orbital_map.entry(orbit_centre).or_default().push(orbitant);
    }
    orbital_map
}

fn to_undirected_graph(data: &str) -> OrbitalMap {
    let mut orbital_map: OrbitalMap = HashMap::new();
    for line in data.split_whitespace() {
        let parts: Vec<_> = line.trim().split(")").collect();
        assert!(parts.len() == 2);
        let orbit_centre = String::from(parts[0]);
        let orbitant = String::from(parts[1]);
        orbital_map
            .entry(orbit_centre.clone())
            .or_default()
            .push(orbitant.clone());
        orbital_map.entry(orbitant).or_default().push(orbit_centre);
    }
    orbital_map
}

fn solve(orb_map: &OrbitalMap, start: &str) -> usize {
    let mut cache = HashMap::new();

    fn go<'a>(orb_map: &'a OrbitalMap, s: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
        let res = orb_map.get(s).map_or(0, |v| {
            v.iter().map(|node| go(orb_map, node, cache) + 1).sum()
        });

        cache.insert(s, res);
        res
    }

    go(orb_map, start, &mut cache);
    cache.values().sum()
}

fn min_distance(orb_map: &OrbitalMap, start: &str, finish: &str) -> Option<usize> {
    let mut q: VecDeque<(&str, usize)> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    q.push_back((start, 0));
    visited.insert(start);

    loop {
        match q.pop_front() {
            None => break,
            Some((val, dist)) => {
                if let Some(ref vec) = orb_map.get(val) {
                    for v in vec.iter() {
                        if v == finish {
                            return Some(dist + 1);
                        }
                        if !visited.contains(v.as_str()) {
                            q.push_back((v, dist + 1));
                            visited.insert(v);
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "COM)B B)C C)D D)E E)F B)G G)H D)I E)J J)K K)L";
        let map = to_directed_graph(&input);
        assert_eq!(solve(&map, "K"), 1);
        assert_eq!(solve(&map, "L"), 0);
        assert_eq!(solve(&map, "J"), 3);
        assert_eq!(solve(&map, "E"), 7);
        assert_eq!(solve(&map, "COM"), 42);
    }

    #[test]
    fn hm_test1() {
        let mut hm: HashMap<String, u32> = HashMap::new();
        hm.insert(String::from("aaa"), 1);
        hm.insert(String::from("aaa"), 2);
        assert_eq!(hm["aaa"], 2);
    }

    #[test]
    fn hm_test2() {
        let mut hm: HashMap<&str, u32> = HashMap::new();
        let ss = "aaa";
        let tt = "aaa";
        assert_eq!(ss, tt);
        hm.insert(ss, 1);
        hm.insert(tt, 2);
        assert_eq!(hm[&ss], 2);
        assert_eq!(hm[&tt], 2);
    }
}
