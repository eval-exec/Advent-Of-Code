use std::collections::{HashMap, HashSet};

fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    let content = std::fs::read_to_string(file_name).unwrap();
    content.lines().map(|line| line.chars().collect()).collect()
}

fn get_locations(chars: &Vec<Vec<char>>) -> HashMap<char, HashSet<[usize; 2]>> {
    let mut locations = HashMap::new();
    for (i, line) in chars.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if is_antenna(c) {
                locations.entry(c).or_insert(HashSet::new()).insert([i, j]);
            }
        }
    }
    locations
}

fn is_antenna(c: char) -> bool {
    c.is_ascii_digit() || c.is_ascii_uppercase() || c.is_ascii_lowercase()
}

struct Map {
    chars: Vec<Vec<char>>,
    locations: HashMap<char, HashSet<[usize; 2]>>,
    width: usize,
    height: usize,
}

impl Map {
    fn cal_antinode(&self, antenna0: [usize; 2], antenna1: [usize; 2]) -> Vec<[usize; 2]> {
        let mut results: Vec<[usize; 2]> = Default::default();
        let delta_init = {
            let delta0 = antenna1[0] as i64 - antenna0[0] as i64;
            let delta1 = antenna1[1] as i64 - antenna0[1] as i64;
            [delta0, delta1]
        };
        {
            let mut delta = delta_init;
            loop {
                let a0 = {
                    let a0_0 = antenna0[0] as i64 - delta[0];
                    let a0_1 = antenna0[1] as i64 - delta[1];
                    [a0_0, a0_1]
                };

                if a0[0] >= 0
                    && (a0[0] as usize) < self.height
                    && a0[1] >= 0
                    && (a0[1] as usize) < self.width
                {
                    results.push([a0[0] as usize, a0[1] as usize]);
                    delta[0] += delta_init[0];
                    delta[1] += delta_init[1];
                } else {
                    break;
                }
            }
        }

        {
            let mut delta = delta_init;
            loop {
                let a0 = {
                    let a0_0 = antenna0[0] as i64 + delta[0];
                    let a0_1 = antenna0[1] as i64 + delta[1];
                    [a0_0, a0_1]
                };

                if a0[0] >= 0
                    && (a0[0] as usize) < self.height
                    && a0[1] >= 0
                    && (a0[1] as usize) < self.width
                {
                    results.push([a0[0] as usize, a0[1] as usize]);
                    delta[0] += delta_init[0];
                    delta[1] += delta_init[1];
                } else {
                    break;
                }
            }
        }

        // info!("{:?},{:?}-> {:?}", antenna0, antenna1, results);

        results
    }
}
fn part1(file_name: &str) {
    let input = parse_input(file_name);
    let locations: HashMap<char, HashSet<[usize; 2]>> = get_locations(&input);
    let map = Map {
        chars: input.clone(),
        locations,
        width: input[0].len(),
        height: input.len(),
    };

    let mut results: HashMap<[usize; 2], HashSet<char>> = Default::default();

    map.locations.iter().for_each(|(&c, locs)| {
        let locs_vec: Vec<[usize; 2]> = locs.iter().map(|v| *v).collect();

        for i in 0..locs_vec.len() {
            for j in i + 1..locs_vec.len() {
                results
                    .entry(locs_vec[0])
                    .or_insert(HashSet::new())
                    .insert(c);

                results
                    .entry(locs_vec[1])
                    .or_insert(HashSet::new())
                    .insert(c);
                map.cal_antinode(locs_vec[i], locs_vec[j])
                    .iter()
                    .for_each(|&antinode| {
                        results.entry(antinode).or_insert(HashSet::new()).insert(c);
                    });
            }
        }
    });
    println!("part1: results.len: {}", results.len());
}

#[test]
fn part1_example() {
    part1("input.txt.example");
}

#[test]
fn part1_main() {
    part1("input.txt");
}
